use std::env;
use std::path::PathBuf;
use std::process::{Command, Output};

use bstr::{BString, ByteSlice};
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct CommandOutput {
    call_args: Vec<String>,
    status: i32,
    stdout: BString,
    stderr: BString,
}

impl CommandOutput {
    fn new() -> Self {
        Self {
            call_args: vec![],
            status: -1,
            stdout: BString::from(""),
            stderr: BString::from(""),
        }
    }

    fn with_args(&mut self, call_args: &[&str]) -> &mut Self {
        self.call_args
            .append(&mut (*call_args).iter().map(|x| x.to_string()).collect());
        self
    }

    fn with_command_output(&mut self, output: Output) -> &mut Self {
        self.status = output.status.code().unwrap_or(-1);
        self.stdout = BString::from(output.stdout);
        self.stderr = BString::from(output.stderr);
        self
    }

    fn build(&self) -> Self {
        CommandOutput {
            call_args: self.call_args.clone(),
            status: self.status,
            stdout: self.stdout.clone(),
            stderr: self.stderr.clone(),
        }
    }
}

impl Serialize for CommandOutput {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("parameters", 4)?;
        s.serialize_field("call_args", &self.call_args)?;
        s.serialize_field("status", &self.status)?;
        let stdout = self
            .stdout
            .lines()
            .map(|line| format!("{:?}", line.as_bstr()))
            .collect::<Vec<String>>()
            .join("\n");
        s.serialize_field("stdout", &stdout)?;
        let stderr = self
            .stderr
            .lines()
            .map(|line| format!("{:?}", line.as_bstr()))
            .collect::<Vec<String>>()
            .join("\n");
        s.serialize_field("stderr", &stderr)?;
        s.end()
    }
}

fn binary_name(name: &str) -> String {
    if cfg!(windows) {
        format!("{}.exe", name)
    } else {
        String::from(name)
    }
}

fn binary_path(name: &str) -> Result<PathBuf, String> {
    let executable = binary_name(name);
    let manifest_path =
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable not set by cargo");
    let path = PathBuf::from(manifest_path)
        .join("..")
        .join("target")
        .join("debug")
        .join(&executable);

    match path.exists() {
        true => Ok(path),
        false => Err(format!("Can't find binary {} in ./target/debug/", executable)),
    }
}

pub fn run(binary_name: &str, call_args: &[&str]) -> Result<CommandOutput, String> {
    let binary = binary_path(binary_name)?;

    let output = Command::new(binary)
        .args(call_args.iter())
        .output()
        .unwrap_or_else(|_| panic!("Failed to run ruby app {}", binary_name));

    Ok(CommandOutput::new()
        .with_args(call_args)
        .with_command_output(output)
        .build())
}
