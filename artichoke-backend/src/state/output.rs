use std::fmt;
use std::io::{self, Write};

pub trait Output: Send + Sync {
    fn as_debug(&self) -> &dyn fmt::Debug;

    fn write_stdout(&mut self, bytes: &[u8]) -> io::Result<()>;

    fn write_stderr(&mut self, bytes: &[u8]) -> io::Result<()>;

    fn print(&mut self, bytes: &[u8]) {
        let _ = self.write_stdout(bytes);
    }

    fn puts(&mut self, bytes: &[u8]) {
        let _ = self
            .write_stdout(bytes)
            .and_then(|_| self.write_stdout(b"\n"));
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Process;

impl Process {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Output for Process {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn write_stdout(&mut self, bytes: &[u8]) -> io::Result<()> {
        io::stdout().write_all(bytes)
    }

    fn write_stderr(&mut self, bytes: &[u8]) -> io::Result<()> {
        io::stderr().write_all(bytes)
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Captured {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

impl Captured {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.stdout.clear();
        self.stderr.clear();
    }

    #[must_use]
    pub fn stdout(&self) -> &[u8] {
        self.stdout.as_slice()
    }

    #[must_use]
    pub fn stderr(&self) -> &[u8] {
        self.stderr.as_slice()
    }
}

impl Output for Captured {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn write_stdout(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.stdout.write_all(bytes)
    }

    fn write_stderr(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.stderr.write_all(bytes)
    }
}

impl<'a> Output for &'a mut Captured {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn write_stdout(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.stdout.write_all(bytes)
    }

    fn write_stderr(&mut self, bytes: &[u8]) -> io::Result<()> {
        self.stderr.write_all(bytes)
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Null;

impl Null {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Output for Null {
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn write_stdout(&mut self, bytes: &[u8]) -> io::Result<()> {
        let _ = bytes;
        Ok(())
    }

    fn write_stderr(&mut self, bytes: &[u8]) -> io::Result<()> {
        let _ = bytes;
        Ok(())
    }
}
