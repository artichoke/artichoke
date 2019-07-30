use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Heap {
    memory: HashMap<u32, Vec<u8>>,
    next_free: u32,
}

impl Heap {
    pub fn allocate(&mut self, s: String) -> u32 {
        let ptr = self.next_free;
        self.next_free += 1;
        self.memory.insert(ptr, s.into_bytes());
        ptr
    }

    pub fn free(&mut self, ptr: u32) {
        self.memory.remove(&ptr);
    }

    pub fn string(&self, ptr: u32) -> &[u8] {
        self.memory.get(&ptr).map(Vec::as_slice).unwrap_or_default()
    }

    pub fn string_getlen(&self, ptr: u32) -> u32 {
        if let Some(s) = self.memory.get(&ptr) {
            s.len() as u32
        } else {
            0
        }
    }

    pub fn string_getch(&self, ptr: u32, idx: u32) -> u8 {
        if let Some(s) = self.memory.get(&ptr) {
            s[idx as usize]
        } else {
            0
        }
    }

    pub fn string_putch(&mut self, ptr: u32, ch: u8) {
        if let Some(s) = self.memory.get_mut(&ptr) {
            s.push(ch);
        }
    }
}
