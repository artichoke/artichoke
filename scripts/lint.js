#!/usr/bin/env node
/* eslint-disable no-console */

const child = require("child_process");
const fs = require("fs").promises;
const path = require("path");

require("array-flat-polyfill");
const prettier = require("prettier");
const shebangCommand = require("shebang-command");
const { spawnClangFormat } = require("clang-format");
const { CLIEngine } = require("eslint");

const STATUS = Object.freeze({
  ok: "ok",
  failed: "failed"
});

const IGNORE_DIRECTORIES = Object.freeze([
  ".git",
  "node_modules",
  "target",
  "vendor"
]);
const args = process.argv.slice(2);
const checkMode = args.includes("--check");

async function walk(dir) {
  try {
    const files = await fs.readdir(dir);
    const children = files
      .filter(file => !IGNORE_DIRECTORIES.includes(file))
      .map(async file => {
        try {
          const filepath = path.join(dir, file);
          const stats = await fs.stat(filepath);
          if (stats.isDirectory()) {
            return walk(filepath);
          }
          if (stats.isFile()) {
            return Promise.resolve(filepath);
          }
          return Promise.reject(filepath);
        } catch (err) {
          return Promise.reject(err);
        }
      });
    return Promise.all(children).then(dirEntries =>
      dirEntries.flat(Infinity).filter(entry => entry != null)
    );
  } catch (err) {
    return Promise.reject(err);
  }
}

const execAsync = (cmd, spawnArgs) => {
  const promise = new Promise((resolve, reject) => {
    const opts = {
      encoding: "utf8",
      stdio: "inherit"
    };

    const subprocess = child.spawn(cmd, spawnArgs, opts);
    subprocess.on("close", code => resolve(code));
    subprocess.on("error", reject);
  });
  return promise;
};

const filesWithExtension = (files, ext) =>
  files.filter(file => {
    const extname = path.extname(file);
    return [ext, `.${ext}`].includes(extname);
  });

const eslintFiles = files => [
  ...new Set([
    ...filesWithExtension(files, "js"),
    ...files.filter(file => shebangCommand(file) === "node"),
    ...filesWithExtension(files, "html")
  ])
];

const shellFiles = files => [
  ...new Set([
    ...filesWithExtension(files, "bash"),
    ...filesWithExtension(files, "sh"),
    ...files.filter(file => ["bash", "sh"].includes(shebangCommand(file)))
  ])
];

const cFiles = files => [
  ...filesWithExtension(files, "c"),
  ...filesWithExtension(files, "h")
];

const rubyFiles = files => [
  ...new Set([
    ...filesWithExtension(files, "rb"),
    ...files.filter(file => shebangCommand(file) === "ruby")
  ])
];

const formatWithPrettier = (files, parser) => {
  const opts = { parser };
  if (parser === "markdown") {
    opts.proseWrap = "always";
  }
  return Promise.all(
    files.map(async file => {
      try {
        const contents = await fs.readFile(file);
        const formatted = prettier.format(contents.toString(), opts);
        if (formatted !== contents.toString()) {
          await fs.writeFile(file, formatted);
        }
        return Promise.resolve(STATUS.ok);
      } catch (err) {
        return Promise.reject(err);
      }
    })
  );
};

const checkWithPrettier = (files, parser) => {
  const opts = { parser };
  if (parser === "markdown") {
    opts.proseWrap = "always";
  }
  return Promise.all(
    files.map(async file => {
      try {
        const contents = await fs.readFile(file);
        const formatted = prettier.check(contents.toString(), opts);
        if (!formatted) {
          console.error(`KO: prettier [${file}]`);
          return Promise.resolve(STATUS.failed);
        }
        return Promise.resolve(STATUS.ok);
      } catch (err) {
        return Promise.reject(err);
      }
    })
  );
};

async function prettierFormatter(files) {
  const filetypes = [
    ["css", "css"],
    ["html", "html"],
    ["json", "json"],
    ["md", "markdown"],
    ["toml", "toml"],
    ["yaml", "yaml"],
    ["yml", "yaml"]
  ];
  const prettierFormat = Promise.all(
    filetypes.map(config => {
      const [extname, parser] = config;
      if (checkMode) {
        return checkWithPrettier(filesWithExtension(files, extname), parser);
      }
      return formatWithPrettier(filesWithExtension(files, extname), parser);
    })
  );
  return prettierFormat;
}

async function eslintLinter(files) {
  const sources = eslintFiles(files);
  return new Promise(resolve => {
    let rulesMeta;
    const cli = new CLIEngine({
      useEslintrc: true,
      fix: !checkMode
    });
    const report = cli.executeOnFiles(sources);
    if (!checkMode) {
      CLIEngine.outputFixes(report);
    }
    const formatter = cli.getFormatter("stylish");
    const output = formatter(report.results, {
      get rulesMeta() {
        if (!rulesMeta) {
          rulesMeta = {};
          const rules = cli.getRules();
          for (let idx = 0; idx < rules.length; idx += 1) {
            const [ruleId, rule] = rules[idx];
            rulesMeta[ruleId] = rule.meta;
          }
        }
        return rulesMeta;
      }
    });
    if (output) {
      console.log(output);
    }
    if (report.errorCount === 0 && report.warningCount === 0) {
      resolve(STATUS.ok);
    } else {
      resolve(STATUS.failed);
    }
  });
}

async function shellLinter(files) {
  const sources = shellFiles(files);
  const shfmt = Promise.all(
    sources.map(async file => {
      try {
        if (checkMode) {
          const code = await execAsync("shfmt", [
            "-i",
            "2",
            "-ci",
            "-s",
            "-d",
            file
          ]);
          if (code === 0) {
            return Promise.resolve(STATUS.ok);
          }
          console.error(`KO: shfmt [${file}]`);
          return Promise.resolve(STATUS.failed);
        }
        const code = await execAsync("shfmt", [
          "-i",
          "2",
          "-ci",
          "-s",
          "-w",
          file
        ]);
        if (code === 0) {
          return Promise.resolve(STATUS.ok);
        }
        console.error(`KO: shfmt [${file}]`);
        return Promise.resolve(STATUS.failed);
      } catch (err) {
        return Promise.reject(err);
      }
    })
  );
  const shellcheck = Promise.all(
    sources.map(async file => {
      try {
        const code = await execAsync("shellcheck", [file]);
        if (code === 0) {
          return Promise.resolve(STATUS.ok);
        }
        console.error(`KO: shellcheck [${file}]`);
        return Promise.resolve(STATUS.failed);
      } catch (err) {
        return Promise.reject(err);
      }
    })
  );
  return Promise.all([shfmt, shellcheck]);
}

async function rustFormatter() {
  try {
    if (checkMode) {
      const code = await execAsync("cargo", [
        "fmt",
        "--",
        "--check",
        "--color=auto"
      ]);
      if (code === 0) {
        return Promise.resolve(STATUS.ok);
      }
      console.error("KO: rustfmt");
      return Promise.resolve(STATUS.failed);
    }
    const code = await execAsync("cargo", ["fmt"]);
    if (code === 0) {
      return Promise.resolve(STATUS.ok);
    }
    console.error("KO: rustfmt");
    return Promise.resolve(STATUS.failed);
  } catch (err) {
    return Promise.reject(err);
  }
}

async function clippyLinter() {
  try {
    const code = await execAsync("cargo", ["clippy", "--", "-D", "warnings"]);
    if (code === 0) {
      return Promise.resolve(STATUS.ok);
    }
    console.error("KO: clippy");
    return Promise.resolve(STATUS.failed);
  } catch (err) {
    return Promise.reject(err);
  }
}

async function rustDocBuilder() {
  try {
    const toolchain = (await fs.readFile("rustdoc-toolchain"))
      .toString()
      .trim();
    const code = await execAsync("rustup", [
      "run",
      "--install",
      toolchain,
      "cargo",
      "doc",
      "--no-deps",
      "--all"
    ]);
    if (code === 0) {
      return Promise.resolve(STATUS.ok);
    }
    console.error("KO: cargo doc");
    return Promise.reject(code);
  } catch (err) {
    return Promise.reject(err);
  }
}

async function clangFormatter(files) {
  const sources = cFiles(files);
  return Promise.all(
    sources.map(source => {
      return new Promise((resolve, reject) => {
        let formatted = "";
        const done = async err => {
          if (err) {
            reject(err);
          } else {
            try {
              const contents = await fs.readFile(source);
              if (formatted.toString() === contents.toString()) {
                resolve(STATUS.ok);
              } else if (checkMode) {
                console.error(`KO: clang-format [${source}]`);
                resolve(STATUS.failed);
              } else {
                await fs.writeFile(source, formatted.toString());
              }
            } catch (error) {
              reject(error);
            }
          }
        };
        const formatter = spawnClangFormat([source], done, [
          "ignore",
          "pipe",
          process.stderr
        ]);
        formatter.stdout.on("data", data => {
          formatted += data.toString();
        });
      });
    })
  );
}

async function rubyLinter(files) {
  try {
    const sources = rubyFiles(files);
    if (checkMode) {
      const code = await execAsync("bundle", ["exec", "rubocop", ...sources]);
      if (code === 0) {
        return Promise.resolve(STATUS.ok);
      }
      console.error("KO: rubocop");
      return Promise.resolve(STATUS.failed);
    }
    const code = await execAsync("bundle", [
      "exec",
      "rubocop",
      "-a",
      ...sources
    ]);
    if (code === 0) {
      return Promise.resolve(STATUS.ok);
    }
    console.error("KO: rubocop");
    return Promise.resolve(STATUS.failed);
  } catch (err) {
    return Promise.reject(err);
  }
}

(async function runner() {
  const timer = setInterval(() => {}, 100);
  try {
    const files = await walk(path.resolve(__dirname, ".."));
    const lintState = await Promise.all([
      prettierFormatter(files),
      eslintLinter(files),
      shellLinter(files),
      rustFormatter(),
      clippyLinter(),
      rustDocBuilder(),
      clangFormatter(files),
      rubyLinter(files)
    ]);
    const failures = lintState
      .flat(Infinity)
      .filter(status => status === STATUS.failed);
    if (failures.length > 0) {
      process.exit(1);
    }
  } catch (err) {
    console.error("Error: Unhandled exception");
    console.error(err);
    process.exit(1);
  } finally {
    timer.unref();
  }
})();
