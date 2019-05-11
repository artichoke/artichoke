#[derive(Debug, Clone)]
pub struct File<Metadata> {
    pub contents: Vec<u8>,
    pub mode: u32,
    pub metadata: Option<Metadata>,
}

impl<Metadata> File<Metadata> {
    pub fn new(contents: Vec<u8>) -> Self {
        File {
            contents,
            mode: 0o644,
            metadata: None,
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
pub enum Node<Metadata> {
    File(File<Metadata>),
    Dir(Dir),
}

impl<Metadata> Node<Metadata> {
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
