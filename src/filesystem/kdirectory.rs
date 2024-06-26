use std::any::Any;
use fuser::FileType;
use crate::filesystem::inter::file::File;

pub struct KDirectory {
    name: String,
    files: Vec<Box<dyn File>>
}

impl KDirectory {

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new()
        }
    }

    pub fn get_files(&self) -> &Vec<Box<dyn File>> {
        &self.files
    }

    pub fn get_file(&self, index: usize) -> &Box<dyn File> {
        self.files.get(index).unwrap()
    }

    pub fn add_file(&mut self, file: Box<dyn File>) {
        self.files.push(file);
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }
}

impl File for KDirectory {

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> FileType {
        FileType::Directory
    }

    fn get_size(&self) -> u64 {
        0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
