use fuser::FileType;
use crate::filesystem::inter::file::File;

pub struct KDirectory {
    name: String
}

impl KDirectory {

    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
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
}
