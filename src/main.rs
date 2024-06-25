pub mod filesystem;
use filesystem::kfs::KFS;
use fuser::MountOption;

fn main() {
    let mountpoint = "/media/test";
    let mut options = vec![MountOption::RO, MountOption::FSName("KFS".to_string())];
    fuser::mount2(KFS, mountpoint, &options).unwrap();
}
