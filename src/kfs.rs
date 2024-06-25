use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use fuser::{FileAttr, Filesystem, FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request};

const TTL: Duration = Duration::from_secs(1); // 1 second

const HELLO_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

const HELLO_DIR_ATTR2: FileAttr = FileAttr {
    ino: 3, // Replace with appropriate inode number
    size: 0, // Directories generally have a size of 0
    blocks: 0,
    atime: UNIX_EPOCH, // Last access time (Unix epoch for simplicity)
    mtime: UNIX_EPOCH, // Last modification time (Unix epoch for simplicity)
    ctime: UNIX_EPOCH, // Last status change time (Unix epoch for simplicity)
    crtime: UNIX_EPOCH, // Creation time (Unix epoch for simplicity)
    kind: FileType::Directory, // Indicates it's a directory
    perm: 0o755, // Permissions: rwxr-xr-x (Owner: Read, Write, Execute; Group: Read, Execute; Others: Read, Execute)
    nlink: 2, // Number of hard links (typically 2 for a directory)
    uid: 501, // Replace with actual user ID
    gid: 20, // Replace with actual group ID
    rdev: 0,
    flags: 0,
    blksize: 512, // Block size for filesystem I/O operations
};

const HELLO_TXT_CONTENT: &str = "Hello World!\n";

const HELLO_TXT_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 13,
    blocks: 1,
    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
    mtime: UNIX_EPOCH,
    ctime: UNIX_EPOCH,
    crtime: UNIX_EPOCH,
    kind: FileType::RegularFile,
    perm: 0o644,
    nlink: 1,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
    blksize: 512,
};

pub struct KFS;

impl Filesystem for KFS {

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 {
            match name.to_str() {
                Some("hello.txt") => reply.entry(&TTL, &HELLO_TXT_ATTR, 0),
                Some("test") => reply.entry(&TTL, &HELLO_DIR_ATTR2, 0),
                _ => reply.error(2), // Return error if name is not recognized
            }
        } else {
            reply.error(2); // Return error for unknown parent directory
        }
        /*
        if parent == 1 && name.to_str() == Some("hello.txt") {
            reply.entry(&TTL, &HELLO_TXT_ATTR, 0);
        } else {
            reply.error(2);
        }
        */
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        match ino {
            1 => reply.attr(&TTL, &HELLO_DIR_ATTR),
            3 => reply.attr(&TTL, &HELLO_DIR_ATTR2),
            2 => reply.attr(&TTL, &HELLO_TXT_ATTR),
            _ => reply.error(2),
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, _flags: i32, _lock: Option<u64>, reply: ReplyData) {
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(2);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != 1 {
            reply.error(2);
            return;
        }

        let entries = vec![
            (1, FileType::Directory, "."),
            (1, FileType::Directory, ".."),
            (3, FileType::Directory, "test"),
            (2, FileType::RegularFile, "hello.txt"),
        ];

        for (i, &(inode, file_type, name)) in entries.iter().enumerate().skip(offset as usize) {
            if reply.add(inode, (i + 1) as i64, file_type, name) {
                break;
            }
        }

        /*
        for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
            // i + 1 means the index of the next entry
            if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
                break;
            }
        }
        */
        reply.ok();
    }
}
