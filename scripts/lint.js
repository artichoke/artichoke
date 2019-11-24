#!/usr/bin/env node
/* eslint-disable no-console */

const child = require("child_process");
const fs = require("fs");
const path = require("path");

require("array-flat-polyfill");
const prettier = require("prettier");
const shebangCommand = require("shebang-command");
const { spawnClangFormat } = require("clang-format");
const { CLIEngine } = require("eslint");

const IGNORE_DIRECTORIES = Object.freeze([
  ".git",
  "node_modules",
  "target",
  "vendor"
]);
const args = process.argv.slice(2);
const checkMode = args.includes("--check");

const walk = dir => {
  return new Promise((resolve, reject) => {
    fs.readdir(dir, (error, files) => {
      if (error) {
        reject(error);
      } else {
        const children = files
          .filter(file => !IGNORE_DIRECTORIES.includes(file))
          .map(file => {
            return new Promise((pathResolve, pathReject) => {
              const filepath = path.join(dir, file);
              fs.stat(filepath, (statError, stats) => {
                if (error) {
                  pathReject(error);
                } else if (stats.isDirectory()) {
                  walk(filepath).then(pathResolve);
                } else if (stats.isFile()) {
                  pathResolve(filepath);
                } else {
                  pathReject(filepath);
                }
              });
            });
          });
        Promise.all(children).then(dirEntries =>
          resolve(dirEntries.flat(Infinity).filter(entry => entry != null))
        );
      }
    });
  });
};

const execAsync = (cmd, spawnArgs, callback) => {
  const opts = {
    encoding: "utf8",
    stdio: "inherit"
  };

  const subprocess = child.spawn(cmd, spawnArgs, opts);
  subprocess.on("close", code => callback(null, code));
  subprocess.on("error", callback);
  return subprocess;
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
    files.map(file => {
      return new Promise((resolve, reject) => {
        fs.readFile(file, (readErr, contents) => {
          if (readErr) {
            reject(readErr);
          } else {
            const formatted = prettier.format(contents.toString(), opts);
            if (formatted === contents.toString()) {
              resolve(true);
            } else {
              fs.writeFile(file, formatted, writeErr => {
                if (writeErr) {
                  reject(writeErr);
                } else {
                  resolve(true);
                }
              });
            }
          }
        });
      });
    })
  );
};

const checkWithPrettier = (files, parser) => {
  const opts = { parser };
  if (parser === "markdown") {
    opts.proseWrap = "always";
  }
  return Promise.all(
    files.map(file => {
      return new Promise((resolve, reject) => {
        fs.readFile(file, (readErr, contents) => {
          if (readErr) {
            reject(readErr);
          } else {
            const formatted = prettier.check(contents.toString(), opts);
            if (!formatted) {
              console.error(`KO: ${file}`);
              resolve(false);
            } else {
              resolve(true);
            }
          }
        });
      });
    })
  );
};

async function prettierFormatter(files) {
  const filetypes = [
    ["css", "css"],
    ["html", "html"],
    ["json", "json"],
    ["md", "markdown"],
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
    resolve(true);
  });
}

async function shellLinter(files) {
  const sources = shellFiles(files);
  const shfmt = Promise.all(
    sources.map(file => {
      if (checkMode) {
        return new Promise((resolve, reject) => {
          execAsync(
            "shfmt",
            ["-i", "2", "-ci", "-s", "-d", file],
            (err, code) => {
              if (err) {
                reject(err);
              } else if (code === 0) {
                resolve(true);
              } else {
                console.error(`KO: ${file}`);
                resolve(false);
              }
            }
          );
        });
      }
      return new Promise((resolve, reject) => {
        execAsync(
          "shfmt",
          ["-i", "2", "-ci", "-s", "-w", file],
          (err, code) => {
            if (err) {
              reject(err);
            } else if (code === 0) {
              resolve(true);
            } else {
              console.error(`KO: ${file}`);
              resolve(false);
            }
          }
        );
      });
    })
  );
  const shellcheck = Promise.all(
    sources.map(file => {
      return new Promise((resolve, reject) => {
        execAsync("shellcheck", [file], (err, code) => {
          if (err) {
            reject(err);
          } else if (code === 0) {
            resolve(true);
          } else {
            console.error(`KO: ${file}`);
            resolve(false);
          }
        });
      });
    })
  );
  return Promise.all([shfmt, shellcheck]);
}

async function rustFormatter() {
  return new Promise((resolve, reject) => {
    if (checkMode) {
      execAsync(
        "cargo",
        ["fmt", "--", "--check", "--color=auto"],
        (err, code) => {
          if (err) {
            reject(err);
          } else if (code === 0) {
            resolve(true);
          } else {
            console.error("KO: cargo fmt");
            resolve(false);
          }
        }
      );
    } else {
      execAsync("cargo", ["fmt"], (err, code) => {
        if (err) {
          reject(err);
        } else if (code === 0) {
          resolve(true);
        } else {
          console.error("KO: cargo fmt");
          resolve(false);
        }
      });
    }
  });
}

async function clippyLinter() {
  return new Promise((resolve, reject) => {
    execAsync("cargo", ["clippy", "--", "-D", "warnings"], (err, code) => {
      if (err) {
        reject(err);
      } else if (code === 0) {
        resolve(true);
      } else {
        console.error("KO: cargo clippy");
        resolve(false);
      }
    });
  });
}

async function rustDocBuilder() {
  return new Promise((resolve, reject) => {
    execAsync("cargo", ["doc", "--no-deps", "--all"], (err, code) => {
      if (err) {
        reject(err);
      } else if (code === 0) {
        resolve(true);
      } else {
        console.error("KO: cargo doc");
        resolve(false);
      }
    });
  });
}

async function clangFormatter(files) {
  const sources = cFiles(files);
  return Promise.all(
    sources.map(source => {
      return new Promise((resolve, reject) => {
        let formatted = "";
        const done = err => {
          if (err) {
            reject(err);
          } else {
            fs.readFile(source, (readErr, contents) => {
              if (readErr) {
                reject(readErr);
              } else {
                const formattedContents = formatted.toString();
                if (formattedContents === contents.toString()) {
                  resolve(true);
                } else if (checkMode) {
                  console.error(`KO: ${source}`);
                  resolve(false);
                } else {
                  fs.writeFile(source, formatted.toString(), writeErr => {
                    if (writeErr) {
                      reject(writeErr);
                    } else {
                      resolve(true);
                    }
                  });
                }
              }
            });
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
  const sources = rubyFiles(files);
  return new Promise((resolve, reject) => {
    if (checkMode) {
      execAsync("bundle", ["exec", "rubocop", ...sources], (err, code) => {
        if (err) {
          reject(err);
        } else if (code === 0) {
          resolve(true);
        } else {
          console.error("KO: Ruby");
          resolve(false);
        }
      });
    } else {
      execAsync(
        "bundle",
        ["exec", "rubocop", "-a", ...sources],
        (err, code) => {
          if (err) {
            reject(err);
          } else if (code === 0) {
            resolve(true);
          } else {
            console.error("KO: Ruby");
            resolve(false);
          }
        }
      );
    }
  });
}

(async function runner() {
  const timer = setInterval(() => {}, 100);
  let failed = false;
  try {
    const files = await walk(path.resolve(__dirname, ".."));
    const jobs = [
      prettierFormatter(files),
      eslintLinter(files),
      shellLinter(files),
      rustFormatter(),
      clippyLinter(),
      rustDocBuilder(),
      clangFormatter(files),
      rubyLinter(files)
    ].map(p =>
      p.catch(err => {
        console.error("Error: Unhandled exception");
        if (err) {
          console.error(err);
        }
        failed = true;
        return err;
      })
    );
    await Promise.all(jobs)
      .then(returnCodes => {
        const failures = returnCodes
          .flat(Infinity)
          .filter(status => status === false);
        if (failures.length > 0) {
          failed = true;
        }
      })
      .catch(err => {
        console.error("Error: Unhandled exception");
        if (err) {
          console.error(err);
        }
        failed = true;
      });
  } catch (err) {
    console.error(err);
    failed = true;
  }
  timer.unref();
  if (failed) {
    process.exit(1);
  }
})();
