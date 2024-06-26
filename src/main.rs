pub mod filesystem;

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

    let mountpoint = "/media/test";
    let mut options = vec![
        MountOption::RO,
        MountOption::FSName("KFS".to_string())
        //MountOption::AutoUnmount
    ];
    fuser::mount2(KFS::new(files), mountpoint, &options).unwrap();
}
