extern crate bencode;
pub mod filesystem;

use bencode::variables::bencode_array::{AddArray, BencodeArray};
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use bencode::variables::inter::bencode_variable::BencodeVariable;
use filesystem::kfs::KFS;
use fuser::MountOption;
use crate::filesystem::inter::file::File;
use crate::filesystem::kdirectory::KDirectory;
use crate::filesystem::kfile::KFile;

fn main() {
    let mut files: Vec<Box<dyn File>> = Vec::new();
    files.push(Box::new(KFile::new("hello_world.txt", 100)));
    files.push(Box::new(KDirectory::new("test")));

    for i in 0..files.len() {
        let file = files.get(i).unwrap();
        println!("{} {}", file.get_size(), file.get_name());
    }

    /*
    let mut ben = BencodeArray::new();

    let mut file = BencodeObject::new();
    file.put("name", "hello_world.txt");
    file.put("type", "file");
    file.put("size", 14);
    ben.add(file);

    let mut dir = BencodeObject::new();
    dir.put("name", "test");
    dir.put("type", "directory");
    ben.add(dir);
    */


    let mountpoint = "/media/test";
    let mut options = vec![
        MountOption::RO,
        MountOption::FSName("KFS".to_string())
        //MountOption::AutoUnmount
    ];
    fuser::mount2(KFS::new(files), mountpoint, &options).unwrap();
}
