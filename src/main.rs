pub mod filesystem;

use std::collections::{HashMap, HashSet};
use std::time::UNIX_EPOCH;
use filesystem::kfs::KFS;
use fuser::{FileAttr, FileType, MountOption};
use crate::filesystem::inter::file::File;
use crate::filesystem::inter::node::{Data, Node};
use crate::filesystem::kdirectory::KDirectory;
use crate::filesystem::kfile::KFile;

fn main() {
    /*
    let mut files: Vec<Box<dyn File>> = Vec::new();
    files.push(Box::new(KFile::new("hello_world.txt", 100)));

    let mut dir = KDirectory::new("test");
    dir.add_file(Box::new(KFile::new("new.txt", 100)));
    files.push(Box::new(dir));
    */


    let mut files = HashMap::new();
    files.insert(2, Node {
        data: Data {
            name: "hello_world.txt".to_string(),
            kind: FileType::RegularFile,
            size: 3
        },
        children: None,
        parent: 1
    });

    files.insert(3, Node {
        data: Data {
            name: "new.txt".to_string(),
            kind: FileType::RegularFile,
            size: 3
        },
        children: None,
        parent: 1
    });


    let mut children = HashSet::new();
    children.insert(5 as u64);

    files.insert(4, Node {
        data: Data {
            name: "test".to_string(),
            kind: FileType::Directory,
            size: 0
        },
        children: Some(children),
        parent: 1
    });



    files.insert(5, Node {
        data: Data {
            name: "asd".to_string(),
            kind: FileType::Directory,
            size: 0
        },
        children: Some(HashSet::new()),
        parent: 4
    });


    /*
    for i in 0..files.len() {
        let file = files.get(i).unwrap();
        println!("{} {}", file.get_size(), file.get_name());
    }
    */

    let mountpoint = "/media/test2";
    let mut options = vec![
        MountOption::RW,
        MountOption::FSName("KFS".to_string())
        //MountOption::AutoUnmount
    ];
    fuser::mount2(KFS::new(files), mountpoint, &options).unwrap();
}
