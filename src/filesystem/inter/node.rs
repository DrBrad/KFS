use std::collections::HashSet;
use fuser::{FileAttr, FileType};

pub struct Node {
    pub data: Data,
    pub children: Option<HashSet<u64>>,
    pub parent: u64
}

pub struct Data {
    pub name: String,
    pub kind: FileType,
    pub size: u64
}