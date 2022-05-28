use std::collections::HashMap;

const CHUNK_COUNT: usize = 32;

enum FileType {
    File,
    Directory,
}

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
    files: HashMap<usize, File>,
}

impl Metadata {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }
}

pub struct File {
    name: String,
    id: usize,
    parent_id: Option<usize>,
    file_type: FileType,
    chunk_ids: Vec<usize>,
    subfiles: Option<Vec<usize>>,
}

impl File {
    fn new_file(name: String, id: usize, parent_id: Option<usize>) -> Self {
        Self {
            name,
            id,
            parent_id,
            file_type: FileType::File,
            chunk_ids: Vec::new(),
            subfiles: None,
        }
    }

    fn new_directory(name: String, id: usize, parent_id: Option<usize>) -> Self {
        Self {
            name,
            id,
            parent_id,
            file_type: FileType::Directory,
            chunk_ids: Vec::new(),
            subfiles: Some(Vec::new()),
        }
    }

    fn assign_chunk(&mut self, chunk_id: usize) {
        self.chunk_ids.push(chunk_id);
    }
}

impl Filesystem {
    fn new() -> Self {
        let mut ret = Self {
            id_ctr: 0,
            metadata: Metadata::new(),
            storage: [StorageChunk { occupied: false }; CHUNK_COUNT],
        };
        ret.add_directory(String::from("root"), None);
        ret
    }

    fn add_file(&mut self, name: String, parent_id: Option<usize>) {
        let newfile = File::new_file(name, self.id_ctr, parent_id);
        self.metadata.files.insert(self.id_ctr, newfile);
        self.id_ctr += 1;
    }

    fn add_directory(&mut self, name: String, parent_id: Option<usize>) {
        let newdirectory = File::new_directory(name, self.id_ctr, parent_id);
        self.metadata.files.insert(self.id_ctr, newdirectory);
        self.id_ctr += 1;
    }
}
