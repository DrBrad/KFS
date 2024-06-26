use std::any::Any;
use fuser::FileType;

pub trait File {

    fn get_name(&self) -> String;

    fn get_type(&self) -> FileType;

    fn get_size(&self) -> u64;

    fn as_any(&self) -> &dyn Any;
}