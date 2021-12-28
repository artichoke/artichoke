mod artichoke;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(dead_code)]
#[derive(Debug)]
struct CommandOutput<'a> {
    call_args: Vec<&'a str>,
    status: i32,
    stdout: BString,
    stderr: BString,
}

fn manifest_path() -> String {
    env::var_os("CARGO_MANIFEST_DIR").unwrap().into_string().unwrap()
}

fn binary_path<'a>(name: &'a str) -> Result<PathBuf, String> {
    let manifest_path = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable not set by cargo");
    let path = PathBuf::from(manifest_path).join("target").join("debug").join(name);
    match path.exists() {
        true => Ok(path),
        false => Err(format!("Can't find binary {} in ./target/debug/", name)),
    }
}

fn run<'a>(binary_name: &'a str, call_args: Vec<&'a str>) -> Result<CommandOutput<'a>, String> {
    let binary = binary_path(binary_name)?;

    let output = Command::new(binary)
        .args(call_args.clone())
        .output()
        .expect(format!("Failed to run ruby app {}", binary_name).as_str());

    let status = match output.status.code() {
        Some(code) => code,
        None => -1,
    };

    Ok(CommandOutput {
        call_args,
        status,
        stdout: std::str::from_utf8(&output.stdout).expect("Invalid Utf-8 string").to_string(),
        stderr: std::str::from_utf8(&output.stderr).expect("Invalid Utf-8 string").to_string(),
    })
}

