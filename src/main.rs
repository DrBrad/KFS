extern crate bencode;
pub mod filesystem;

use bencode::variables::bencode_array::{AddArray, BencodeArray};
use bencode::variables::bencode_object::{BencodeObject, PutObject};
use bencode::variables::inter::bencode_variable::BencodeVariable;
use filesystem::kfs::KFS;
use fuser::MountOption;

fn main() {
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


    let mountpoint = "/media/test2";
    let mut options = vec![MountOption::RO, MountOption::FSName("KFS".to_string())];
    fuser::mount2(KFS::new(ben), mountpoint, &options).unwrap();
}
