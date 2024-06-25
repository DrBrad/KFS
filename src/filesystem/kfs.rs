use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use bencode::variables::bencode_array::{AddArray, BencodeArray};
use fuser::{FileAttr, Filesystem, FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, ReplyStatfs, Request};

const TTL: Duration = Duration::from_secs(1); // 1 second

pub struct KFS {
    files: BencodeArray
}

impl KFS {

    pub fn new(mut files: BencodeArray) -> Self {
        Self {
            files
        }
    }
}

impl Filesystem for KFS {

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 {
            println!("{}", name.to_str().unwrap());

            for i in 0..self.files.size() {
                if self.files.get_object(i).unwrap().get_string("name").unwrap() == name.to_str().unwrap() {
                    match self.files.get_object(i).unwrap().get_string("type").unwrap() {
                        "file" => {
                            reply.entry(&TTL, &FileAttr {
                                ino: (i as u64)+2,
                                size: self.files.get_object(i).unwrap().get_number("size").unwrap(),
                                blocks: 1,
                                atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                                mtime: UNIX_EPOCH,
                                ctime: UNIX_EPOCH,
                                crtime: UNIX_EPOCH,
                                kind: FileType::RegularFile,
                                perm: 0o777,
                                nlink: 1,
                                uid: 501,
                                gid: 20,
                                rdev: 0,
                                flags: 0,
                                blksize: 512
                            }, 0);
                        },
                        "directory" => {
                            reply.entry(&TTL, &FileAttr {
                                ino: (i as u64)+2,
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
                                blksize: 512
                            }, 0);
                        },
                        _ => reply.error(2)
                    }
                    return;
                }
            }
        }

        reply.error(2); // Return error for unknown parent directory
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        println!("{}", ino);

        if ino == 1 {
            reply.attr(&TTL, &FileAttr {
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
                blksize: 512
            });
            return;
        }

        let file = self.files.get_object((ino as usize)-2).unwrap();
        match file.get_string("type").unwrap() {
            "file" => {
                reply.attr(&TTL, &FileAttr {
                    ino,
                    size: file.get_number("size").unwrap(),
                    blocks: 1,
                    atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                    mtime: UNIX_EPOCH,
                    ctime: UNIX_EPOCH,
                    crtime: UNIX_EPOCH,
                    kind: FileType::RegularFile,
                    perm: 0o777,
                    nlink: 1,
                    uid: 501,
                    gid: 20,
                    rdev: 0,
                    flags: 0,
                    blksize: 512
                });
            },
            "directory" => {
                reply.attr(&TTL, &FileAttr {
                    ino,
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
                    blksize: 512
                });
            },
            _ => reply.error(2)
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, _flags: i32, _lock: Option<u64>, reply: ReplyData) {
        //if ino == 2 {
            //reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        //} else {
            reply.error(2);
        //}
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != 1 {
            reply.error(2);
            return;
        }

        let default = [ ".", ".." ];

        for i in (offset as usize)..self.files.size()+2 {
            if i < 2 {
                reply.add(1, (i as i64)+1, FileType::Directory, default[i]);
                continue;
            }

            match self.files.get_object(i-2).unwrap().get_string("type").unwrap() {
                "file" => {
                    reply.add(i as u64, (i as i64)+1, FileType::RegularFile, self.files.get_object(i-2).unwrap().get_string("name").unwrap());
                },
                "directory" => {
                    reply.add(i as u64, (i as i64)+1, FileType::Directory, self.files.get_object(i-2).unwrap().get_string("name").unwrap());
                },
                _ => {
                    reply.error(2);
                    return;
                }
            }
        }

        reply.ok();
    }

    fn statfs(&mut self, _req: &Request, _ino: u64, reply: ReplyStatfs) {
        // Example values for total blocks, free blocks, available blocks, etc.
        reply.statfs(
            1000000, // total blocks
            500000,  // free blocks
            500000,  // available blocks
            1000000, // total inodes
            999995,  // free inodes
            512,     // block size
            255,     // maximum name length
            0,       // filesystem ID
        );
    }
}
