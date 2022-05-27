use std::collections::HashMap;

const CHUNK_COUNT: usize = 32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct StorageChunk {
    occupied: bool,
}

pub struct Filesystem {
    id_ctr: usize,
    metadata: Metadata,
    storage: [StorageChunk; CHUNK_COUNT],
}

pub struct Metadata {
    files: HashMap<u32, File>,
}

impl Metadata {
    fn new() -> Self {
        Self { files: HashMap::new() }
    }
}

pub struct File {
    name: String,
    id: u32,
    chunk_ids: Vec<u32>,
}

impl File {
    fn new(name: String, id: u32) -> Self {
        Self {
            name,
            id,
            chunk_ids: Vec::new(),
        }
    }

    fn assign_chunk(&mut self, chunk_id: u32) {
        self.chunk_ids.push(chunk_id);
    }
}

impl Filesystem {
    fn new() -> Self {
        Self {
            id_ctr: 0,
            metadata: Metadata::new(),
            storage: [StorageChunk {occupied: false} ;CHUNK_COUNT],
        }
    }

    fn add_file(&mut self, name: String) {
        
    }
}
