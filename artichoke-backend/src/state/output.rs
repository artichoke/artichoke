use std::fmt;
use std::io::{self, Write};

use bstr::BString;

#[cfg(all(not(feature = "output-strategy-capture"), not(feature = "output-strategy-null")))]
pub type Strategy = Process;
#[cfg(all(feature = "output-strategy-capture", not(feature = "output-strategy-null")))]
pub type Strategy = Captured;
#[cfg(all(feature = "output-strategy-capture", feature = "output-strategy-null"))]
pub type Strategy = Null;

pub trait Output: Send + Sync + fmt::Debug {
    fn write_stdout<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()>;

    fn write_stderr<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()>;

    fn print<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        self.write_stdout(bytes)?;
        Ok(())
    }

    fn puts<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        self.write_stdout(bytes)?;
        self.write_stdout(b"\n")?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Process {
    _private: (),
}

impl Process {
    /// Constructs a new, default `Process` output strategy.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl Output for Process {
    fn write_stdout<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        io::stdout().write_all(bytes.as_ref())
    }

    fn write_stderr<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        io::stderr().write_all(bytes.as_ref())
    }
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Captured {
    stdout: BString,
    stderr: BString,
}

impl Captured {
    /// Constructs a new, default `Captured` output strategy.
    // This method cannot be const because of:
    // https://github.com/BurntSushi/bstr/issues/73
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
    fn write_stdout<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        self.stdout.extend_from_slice(bytes.as_ref());
        Ok(())
    }

    fn write_stderr<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        self.stderr.extend_from_slice(bytes.as_ref());
        Ok(())
    }
}

impl<'a> Output for &'a mut Captured {
    fn write_stdout<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        self.stdout.extend_from_slice(bytes.as_ref());
        Ok(())
    }

    fn write_stderr<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        self.stderr.extend_from_slice(bytes.as_ref());
        Ok(())
    }
}

#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Null {
    _private: (),
}

impl Null {
    /// Constructs a new, default `Null` output strategy.
    #[must_use]
    pub const fn new() -> Self {
        Self { _private: () }
    }
}

impl Output for Null {
    fn write_stdout<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        drop(bytes);
        Ok(())
    }

    fn write_stderr<T: AsRef<[u8]>>(&mut self, bytes: T) -> io::Result<()> {
        drop(bytes);
        Ok(())
    }
}
