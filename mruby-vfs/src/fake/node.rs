#[derive(Debug, Clone)]
pub struct File {
    pub contents: Vec<u8>,
    pub mode: u32,
}

impl File {
    pub fn new(contents: Vec<u8>) -> Self {
        File {
            contents: contents,
            mode: 0o644,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Dir {
    pub mode: u32,
}

impl Dir {
    pub fn new() -> Self {
        Dir { mode: 0o644 }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    File(File),
    Dir(Dir),
}

impl Node {
    pub fn is_file(&self) -> bool {
        match *self {
            Node::File(_) => true,
            _ => false,
        }
    }

    pub fn is_dir(&self) -> bool {
        match *self {
            Node::Dir(_) => true,
            _ => false,
        }
    }
}
