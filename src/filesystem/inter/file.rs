use crate::filesystem::inter::file_type::FileType;

pub trait File {

    fn get_name(&self) -> String;

    fn get_type(&self) -> FileType;

    fn get_size(&self) -> u64;
}