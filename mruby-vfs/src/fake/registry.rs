use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use super::node::{Dir, File, Node};

#[derive(Debug, Clone, Default)]
pub struct Registry {
    cwd: PathBuf,
    files: HashMap<PathBuf, Node>,
}

impl Registry {
    pub fn new() -> Self {
        let cwd = PathBuf::from("/");
        let mut files = HashMap::new();

        files.insert(cwd.clone(), Node::Dir(Dir::new()));

        Registry {
            cwd: cwd,
            files: files,
        }
    }

    pub fn current_dir(&self) -> Result<PathBuf> {
        self.get_dir(&self.cwd).map(|_| self.cwd.clone())
    }

    pub fn set_current_dir(&mut self, cwd: PathBuf) -> Result<()> {
        match self.get_dir(&cwd) {
            Ok(_) => {
                self.cwd = cwd;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn is_dir(&self, path: &Path) -> bool {
        self.files.get(path).map(Node::is_dir).unwrap_or(false)
    }

    pub fn is_file(&self, path: &Path) -> bool {
        self.files.get(path).map(Node::is_file).unwrap_or(false)
    }

    pub fn create_dir(&mut self, path: &Path) -> Result<()> {
        self.insert(path.to_path_buf(), Node::Dir(Dir::new()))
    }

    pub fn create_dir_all(&mut self, path: &Path) -> Result<()> {
        // Based on std::fs::DirBuilder::create_dir_all
        if path == Path::new("") {
            return Ok(());
        }

        match self.create_dir(path) {
            Ok(_) => return Ok(()),
            Err(ref e) if e.kind() == ErrorKind::NotFound => {}
            Err(_) if self.is_dir(path) => return Ok(()),
            Err(e) => return Err(e),
        }

        match path.parent() {
            Some(p) => self.create_dir_all(p)?,
            None => return Err(create_error(ErrorKind::Other)),
        }

        self.create_dir_all(path)
    }

    pub fn remove_dir(&mut self, path: &Path) -> Result<()> {
        match self.get_dir(path) {
            Ok(_) if self.descendants(path).is_empty() => {}
            Ok(_) => return Err(create_error(ErrorKind::Other)),
            Err(e) => return Err(e),
        };

        self.remove(path).and(Ok(()))
    }

    pub fn remove_dir_all(&mut self, path: &Path) -> Result<()> {
        self.get_dir_mut(path)?;

        let descendants = self.descendants(path);

        for child in descendants {
            self.remove(&child)?;
        }

        self.remove(path).and(Ok(()))
    }

    pub fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>> {
        self.get_dir(path)?;

        Ok(self.children(path))
    }

    pub fn create_file(&mut self, path: &Path, buf: &[u8]) -> Result<()> {
        let file = File::new(buf.to_vec());

        self.insert(path.to_path_buf(), Node::File(file))
    }

    pub fn write_file(&mut self, path: &Path, buf: &[u8]) -> Result<()> {
        self.get_file_mut(path)
            .map(|ref mut f| f.contents = buf.to_vec())
            .or_else(|e| {
                if e.kind() == ErrorKind::NotFound {
                    self.create_file(path, buf)
                } else {
                    Err(e)
                }
            })
    }

    pub fn overwrite_file(&mut self, path: &Path, buf: &[u8]) -> Result<()> {
        self.get_file_mut(path)
            .map(|ref mut f| f.contents = buf.to_vec())
    }

    pub fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        match self.get_file(path) {
            Ok(f) if f.mode & 0o444 != 0 => Ok(f.contents.clone()),
            Ok(_) => Err(create_error(ErrorKind::PermissionDenied)),
            Err(err) => Err(err),
        }
    }

    pub fn read_file_to_string(&self, path: &Path) -> Result<String> {
        match self.read_file(path) {
            Ok(vec) => String::from_utf8(vec).map_err(|_| create_error(ErrorKind::InvalidData)),
            Err(err) => Err(err),
        }
    }

    pub fn read_file_into(&self, path: &Path, buf: &mut Vec<u8>) -> Result<usize> {
        match self.get_file(path) {
            Ok(f) if f.mode & 0o444 != 0 => {
                buf.extend(&f.contents);
                Ok(f.contents.len())
            }
            Ok(_) => Err(create_error(ErrorKind::PermissionDenied)),
            Err(err) => Err(err),
        }
    }

    pub fn remove_file(&mut self, path: &Path) -> Result<()> {
        match self.get_file(path) {
            Ok(_) => self.remove(path).and(Ok(())),
            Err(e) => Err(e),
        }
    }

    pub fn copy_file(&mut self, from: &Path, to: &Path) -> Result<()> {
        match self.read_file(from) {
            Ok(ref buf) => self.write_file(to, buf),
            Err(ref err) if err.kind() == ErrorKind::NotFound || err.kind() == ErrorKind::Other => {
                Err(create_error(ErrorKind::InvalidInput))
            }
            Err(err) => Err(err),
        }
    }

    pub fn rename(&mut self, from: &Path, to: &Path) -> Result<()> {
        match (self.files.get(from), self.files.get(to)) {
            (Some(&Node::File(_)), Some(&Node::File(_))) => {
                self.remove_file(to)?;
                self.rename_path(from, to.to_path_buf())
            }
            (Some(&Node::File(_)), None) => self.rename_path(from, to.to_path_buf()),
            (Some(&Node::Dir(_)), Some(&Node::Dir(_))) if self.descendants(to).is_empty() => {
                self.remove(to)?;
                self.move_dir(from, to)
            }
            (Some(&Node::File(_)), Some(&Node::Dir(_)))
            | (Some(&Node::Dir(_)), Some(&Node::File(_)))
            | (Some(&Node::Dir(_)), Some(&Node::Dir(_))) => Err(create_error(ErrorKind::Other)),
            (Some(&Node::Dir(_)), None) => self.move_dir(from, to),
            (None, _) => Err(create_error(ErrorKind::NotFound)),
        }
    }

    pub fn readonly(&self, path: &Path) -> Result<bool> {
        match self.files.get(path) {
            Some(&Node::File(ref f)) => Ok(f.mode & 0o222 == 0),
            Some(&Node::Dir(ref d)) => Ok(d.mode & 0o222 == 0),
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    pub fn set_readonly(&mut self, path: &Path, readonly: bool) -> Result<()> {
        match self.files.get_mut(path) {
            Some(&mut Node::File(ref mut f)) => {
                if readonly {
                    f.mode &= !0o222
                } else {
                    f.mode |= 0o222
                };

                Ok(())
            }
            Some(&mut Node::Dir(ref mut d)) => {
                if readonly {
                    d.mode &= !0o222
                } else {
                    d.mode |= 0o222
                };

                Ok(())
            }
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    pub fn mode(&self, path: &Path) -> Result<u32> {
        match self.files.get(path) {
            Some(&Node::File(ref f)) => Ok(f.mode),
            Some(&Node::Dir(ref d)) => Ok(d.mode),
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    pub fn set_mode(&mut self, path: &Path, mode: u32) -> Result<()> {
        match self.files.get_mut(path) {
            Some(&mut Node::File(ref mut f)) => {
                f.mode = mode;
                Ok(())
            }
            Some(&mut Node::Dir(ref mut d)) => {
                d.mode = mode;
                Ok(())
            }
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    pub fn len(&self, path: &Path) -> u64 {
        match self.files.get(path) {
            Some(&Node::File(ref f)) => f.contents.len() as u64,
            Some(&Node::Dir(_)) => 4096,
            None => 0,
        }
    }

    fn get_dir(&self, path: &Path) -> Result<&Dir> {
        match self.files.get(path) {
            Some(&Node::Dir(ref dir)) => Ok(dir),
            Some(_) => Err(create_error(ErrorKind::Other)),
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    fn get_dir_mut(&mut self, path: &Path) -> Result<&mut Dir> {
        match self.files.get_mut(path) {
            Some(&mut Node::Dir(ref mut dir)) => {
                if dir.mode & 0o222 == 0 {
                    Err(create_error(ErrorKind::PermissionDenied))
                } else {
                    Ok(dir)
                }
            }
            Some(_) => Err(create_error(ErrorKind::Other)),
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    fn get_file(&self, path: &Path) -> Result<&File> {
        match self.files.get(path) {
            Some(&Node::File(ref file)) => Ok(file),
            Some(_) => Err(create_error(ErrorKind::Other)),
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    fn get_file_mut(&mut self, path: &Path) -> Result<&mut File> {
        match self.files.get_mut(path) {
            Some(&mut Node::File(ref mut file)) => {
                if file.mode & 0o222 == 0 {
                    Err(create_error(ErrorKind::PermissionDenied))
                } else {
                    Ok(file)
                }
            }
            Some(_) => Err(create_error(ErrorKind::Other)),
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    fn insert(&mut self, path: PathBuf, file: Node) -> Result<()> {
        if self.files.contains_key(&path) {
            return Err(create_error(ErrorKind::AlreadyExists));
        } else if let Some(p) = path.parent() {
            self.get_dir_mut(p)?;
        }

        self.files.insert(path, file);

        Ok(())
    }

    fn remove(&mut self, path: &Path) -> Result<Node> {
        match self.files.remove(path) {
            Some(f) => Ok(f),
            None => Err(create_error(ErrorKind::NotFound)),
        }
    }

    fn descendants(&self, path: &Path) -> Vec<PathBuf> {
        self.files
            .keys()
            .filter(|p| p.starts_with(path) && *p != path)
            .map(|p| p.to_path_buf())
            .collect()
    }

    fn children(&self, path: &Path) -> Vec<PathBuf> {
        self.files
            .keys()
            .filter(|p| p.parent().map(|parent| parent == path).unwrap_or(false))
            .map(|p| p.to_path_buf())
            .collect()
    }

    fn rename_path(&mut self, from: &Path, to: PathBuf) -> Result<()> {
        let file = self.remove(from)?;
        self.insert(to, file)
    }

    fn move_dir(&mut self, from: &Path, to: &Path) -> Result<()> {
        self.rename_path(from, to.to_path_buf())?;

        for child in self.children(from) {
            let stem = child.strip_prefix(from).unwrap_or(&child);
            let new_path = to.join(stem);

            self.rename(&child, &new_path)?;
        }

        Ok(())
    }
}

fn create_error(kind: ErrorKind) -> Error {
    // Based on private std::io::ErrorKind::as_str()
    let description = match kind {
        ErrorKind::NotFound => "entity not found",
        ErrorKind::PermissionDenied => "permission denied",
        ErrorKind::ConnectionRefused => "connection refused",
        ErrorKind::ConnectionReset => "connection reset",
        ErrorKind::ConnectionAborted => "connection aborted",
        ErrorKind::NotConnected => "not connected",
        ErrorKind::AddrInUse => "address in use",
        ErrorKind::AddrNotAvailable => "address not available",
        ErrorKind::BrokenPipe => "broken pipe",
        ErrorKind::AlreadyExists => "entity already exists",
        ErrorKind::WouldBlock => "operation would block",
        ErrorKind::InvalidInput => "invalid input parameter",
        ErrorKind::InvalidData => "invalid data",
        ErrorKind::TimedOut => "timed out",
        ErrorKind::WriteZero => "write zero",
        ErrorKind::Interrupted => "operation interrupted",
        ErrorKind::Other => "other os error",
        ErrorKind::UnexpectedEof => "unexpected end of file",
        _ => "other",
    };

    Error::new(kind, description)
}
