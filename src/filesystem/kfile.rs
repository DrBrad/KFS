use std::any::Any;
use fuser::FileType;
use crate::filesystem::inter::file::File;

pub struct KFile {
    name: String,
    size: u64
}

impl KFile {

    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_string(),
            size
        }
    }
}

impl File for KFile {

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type(&self) -> FileType {
        FileType::RegularFile
    }

    fn get_size(&self) -> u64 {
        self.size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
