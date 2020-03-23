#!/usr/bin/env node
/* eslint-disable no-console */

const fs = require("fs").promises;
const path = require("path");

require("array-flat-polyfill");
const { spawnClangFormat } = require("clang-format");

const STATUS = Object.freeze({
  ok: "ok",
  failed: "failed",
});

const IGNORE_DIRECTORIES = Object.freeze([
  ".git",
  "node_modules",
  "target",
  "vendor",
]);
const args = process.argv.slice(2);
const checkMode = args.includes("--check");

async function walk(dir) {
  try {
    const files = await fs.readdir(dir);
    const children = files
      .filter((file) => !IGNORE_DIRECTORIES.includes(file))
      .map(async (file) => {
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
    return Promise.all(children).then((dirEntries) =>
      dirEntries.flat(Infinity).filter((entry) => entry != null)
    );
  } catch (err) {
    return Promise.reject(err);
  }
}

const filesWithExtension = (files, ext) =>
  files.filter((file) => {
    const extname = path.extname(file);
    return [ext, `.${ext}`].includes(extname);
  });

const cFiles = (files) => [
  ...filesWithExtension(files, "c"),
  ...filesWithExtension(files, "h"),
];

async function clangFormatter(files) {
  const sources = cFiles(files);
  return Promise.all(
    sources.map((source) => {
      const relative = path.relative(path.resolve(__dirname, ".."), source);
      return new Promise((resolve, reject) => {
        let formatted = "";
        const done = async (err) => {
          if (err) {
            console.error(`KO: ${relative}`);
            reject(err);
          } else {
            try {
              const contents = await fs.readFile(source);
              if (formatted.toString() === contents.toString()) {
                console.info(`OK: ${relative}`);
                resolve(STATUS.ok);
              } else if (checkMode) {
                console.error(`KO: ${relative}`);
                resolve(STATUS.failed);
              } else {
                await fs.writeFile(source, formatted.toString());
                console.info(`OK: ${relative}`);
                resolve(STATUS.ok);
              }
            } catch (error) {
              console.error(`KO: ${relative}`);
              reject(error);
            }
          }
        };
        const formatter = spawnClangFormat([source], done, [
          "ignore",
          "pipe",
          process.stderr,
        ]);
        formatter.stdout.on("data", (data) => {
          formatted += data.toString();
        });
      });
    })
  );
}

(async function runner() {
  const timer = setInterval(() => {}, 100);
  try {
    const files = await walk(path.resolve(__dirname, ".."));
    const lintState = await clangFormatter(files);
    const failures = lintState
      .flat(Infinity)
      .filter((status) => status === STATUS.failed);
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
