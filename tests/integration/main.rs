mod artichoke;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bstr::BString;

#[allow(dead_code)]
#[derive(Debug)]
struct CommandOutput<'a> {
    call_args: Vec<&'a str>,
    status: i32,
    stdout: BString,
    stderr: BString,
}

fn binary_name(name: &str) -> &str {
    if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        name
    }
}

fn binary_path(name: &str) -> Result<PathBuf, String> {
    let executable = binary_name(name);
    let manifest_path =
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable not set by cargo");
    let path = PathBuf::from(manifest_path).join("target").join("debug").join(executable);

    match path.exists() {
        true => Ok(path),
        false => Err(format!("Can't find binary {} in ./target/debug/", executable)),
    }
}

fn run<'a>(binary_name: &'a str, call_args: Vec<&'a str>) -> Result<CommandOutput<'a>, String> {
    let binary = binary_path(binary_name)?;

    let output = Command::new(binary)
        .args(call_args.clone())
        .output()
        .unwrap_or_else(|_| panic!("Failed to run ruby app {}", binary_name));

    let status = output.status.code().unwrap_or(-1);

    Ok(CommandOutput {
        call_args,
        status,
        stdout: BString::from(output.stdout),
        stderr: BString::from(output.stderr),
    })
}
