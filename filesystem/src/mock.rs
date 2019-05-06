use std::error::Error as StdError;
use std::ffi::OsString;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;

use pseudo::Mock;

use FileSystem;

#[derive(Debug, Clone, PartialEq)]
pub struct FakeError {
    kind: ErrorKind,
    description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirEntry {
    file_name: PathBuf,
    is_file: bool,
}

impl DirEntry {
    pub fn new<P: AsRef<Path>>(file_name: P, is_file: bool) -> Self {
        DirEntry {
            file_name: file_name.as_ref().to_path_buf(),
            is_file,
        }
    }
}

impl ::DirEntry for DirEntry {
    fn path(&self) -> PathBuf {
        self.file_name.clone()
    }

    fn file_name(&self) -> OsString {
        self.file_name.clone().into_os_string()
    }
}

#[derive(Debug)]
pub struct ReadDir(IntoIter<Result<DirEntry, Error>>);

impl ReadDir {
    pub fn new() -> Self {
        ReadDir(vec![].into_iter())
    }
}

impl Default for ReadDir {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for ReadDir {
    type Item = Result<DirEntry, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl ::ReadDir<DirEntry> for ReadDir {}

impl From<Error> for FakeError {
    fn from(err: Error) -> Self {
        FakeError {
            kind: err.kind(),
            description: err.description().to_string(),
        }
    }
}

impl From<FakeError> for Error {
    fn from(err: FakeError) -> Self {
        Error::new(err.kind, err.description)
    }
}

#[derive(Debug, Clone)]
pub struct MockFileSystem {
    pub current_dir: Mock<(), Result<PathBuf, FakeError>>,
    pub set_current_dir: Mock<PathBuf, Result<(), FakeError>>,

    pub is_dir: Mock<PathBuf, bool>,
    pub is_file: Mock<PathBuf, bool>,

    pub create_dir: Mock<PathBuf, Result<(), FakeError>>,
    pub create_dir_all: Mock<PathBuf, Result<(), FakeError>>,
    pub remove_dir: Mock<PathBuf, Result<(), FakeError>>,
    pub remove_dir_all: Mock<PathBuf, Result<(), FakeError>>,
    pub read_dir: Mock<PathBuf, Result<Vec<Result<DirEntry, FakeError>>, FakeError>>,

    pub write_file: Mock<(PathBuf, Vec<u8>), Result<(), FakeError>>,
    pub overwrite_file: Mock<(PathBuf, Vec<u8>), Result<(), FakeError>>,
    pub read_file: Mock<(PathBuf), Result<Vec<u8>, FakeError>>,
    pub read_file_to_string: Mock<(PathBuf), Result<String, FakeError>>,
    pub read_file_into: Mock<(PathBuf, Vec<u8>), Result<usize, FakeError>>,
    pub create_file: Mock<(PathBuf, Vec<u8>), Result<(), FakeError>>,
    pub remove_file: Mock<(PathBuf), Result<(), FakeError>>,
    pub copy_file: Mock<(PathBuf, PathBuf), Result<(), FakeError>>,

    pub rename: Mock<(PathBuf, PathBuf), Result<(), FakeError>>,

    pub readonly: Mock<(PathBuf), Result<bool, FakeError>>,
    pub set_readonly: Mock<(PathBuf, bool), Result<(), FakeError>>,

    pub len: Mock<(PathBuf), u64>,
}

impl MockFileSystem {
    pub fn new() -> Self {
        MockFileSystem {
            current_dir: Mock::new(Ok(PathBuf::new())),
            set_current_dir: Mock::new(Ok(())),

            is_dir: Mock::new(true),
            is_file: Mock::new(true),

            create_dir: Mock::new(Ok(())),
            create_dir_all: Mock::new(Ok(())),
            remove_dir: Mock::new(Ok(())),
            remove_dir_all: Mock::new(Ok(())),
            read_dir: Mock::new(Ok(vec![])),

            write_file: Mock::new(Ok(())),
            overwrite_file: Mock::new(Ok(())),
            read_file: Mock::new(Ok(vec![])),
            read_file_to_string: Mock::new(Ok(String::new())),
            read_file_into: Mock::new(Ok(0)),
            create_file: Mock::new(Ok(())),
            remove_file: Mock::new(Ok(())),
            copy_file: Mock::new(Ok(())),

            rename: Mock::new(Ok(())),

            readonly: Mock::new(Ok(false)),
            set_readonly: Mock::new(Ok(())),

            len: Mock::new(u64::default()),
        }
    }
}

impl Default for MockFileSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystem for MockFileSystem {
    type DirEntry = DirEntry;
    type ReadDir = ReadDir;

    fn current_dir(&self) -> Result<PathBuf, Error> {
        self.current_dir.call(()).map_err(Error::from)
    }

    fn set_current_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.set_current_dir
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.is_dir.call(path.as_ref().to_path_buf())
    }

    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.is_file.call(path.as_ref().to_path_buf())
    }

    fn create_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.create_dir
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.create_dir_all
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn remove_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.remove_dir
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn remove_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.remove_dir_all
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<Self::ReadDir, Error> {
        self.read_dir
            .call(path.as_ref().to_path_buf())
            .map(|entries| {
                let entries: Vec<Result<DirEntry, Error>> = entries
                    .into_iter()
                    .map(|e| e.map_err(Error::from))
                    .collect();

                ReadDir(entries.into_iter())
            })
            .map_err(Error::from)
    }

    fn write_file<P, B>(&self, path: P, buf: B) -> Result<(), Error>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>,
    {
        self.write_file
            .call((path.as_ref().to_path_buf(), buf.as_ref().to_vec()))
            .map_err(Error::from)
    }

    fn overwrite_file<P, B>(&self, path: P, buf: B) -> Result<(), Error>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>,
    {
        self.overwrite_file
            .call((path.as_ref().to_path_buf(), buf.as_ref().to_vec()))
            .map_err(Error::from)
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, Error> {
        self.read_file
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn read_file_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String, Error> {
        self.read_file_to_string
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn read_file_into<P, B>(&self, path: P, mut buf: B) -> Result<usize, Error>
    where
        P: AsRef<Path>,
        B: AsMut<Vec<u8>>,
    {
        self.read_file_into
            .call((path.as_ref().to_path_buf(), buf.as_mut().clone()))
            .map_err(Error::from)
    }

    fn create_file<P, B>(&self, path: P, buf: B) -> Result<(), Error>
    where
        P: AsRef<Path>,
        B: AsRef<[u8]>,
    {
        self.create_file
            .call((path.as_ref().to_path_buf(), buf.as_ref().to_vec()))
            .map_err(Error::from)
    }

    fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.remove_file
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn copy_file<P, Q>(&self, from: P, to: Q) -> Result<(), Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        self.copy_file
            .call((from.as_ref().to_path_buf(), to.as_ref().to_path_buf()))
            .map_err(Error::from)
    }

    fn rename<P, Q>(&self, from: P, to: Q) -> Result<(), Error>
    where
        P: AsRef<Path>,
        Q: AsRef<Path>,
    {
        self.rename
            .call((from.as_ref().to_path_buf(), to.as_ref().to_path_buf()))
            .map_err(Error::from)
    }

    fn readonly<P: AsRef<Path>>(&self, path: P) -> Result<bool, Error> {
        self.readonly
            .call(path.as_ref().to_path_buf())
            .map_err(Error::from)
    }

    fn set_readonly<P: AsRef<Path>>(&self, path: P, readonly: bool) -> Result<(), Error> {
        self.set_readonly
            .call((path.as_ref().to_path_buf(), readonly))
            .map_err(Error::from)
    }

    fn len<P: AsRef<Path>>(&self, path: P) -> u64 {
        self.len.call(path.as_ref().to_path_buf())
    }
}
