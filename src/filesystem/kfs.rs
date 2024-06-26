use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use fuser::{FileAttr, Filesystem, FileType, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, ReplyStatfs, Request};
use crate::filesystem::inter::file::File;

const TTL: Duration = Duration::from_secs(1); // 1 second

pub struct KFS {
    files: Vec<Box<dyn File>>
}

impl KFS {

    pub fn new(files: Vec<Box<dyn File>>) -> Self {
        Self {
            files
        }
    }
}

impl Filesystem for KFS {

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent == 1 {
            println!("{}", name.to_str().unwrap());

            for i in 0..self.files.len() {
                if self.files.get(i).unwrap().get_name().as_str() == name.to_str().unwrap() {
                    reply.entry(&TTL, &FileAttr {
                        ino: (i as u64)+2,
                        size: self.files.get(i).unwrap().get_size(),
                        blocks: 1,
                        atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                        mtime: UNIX_EPOCH,
                        ctime: UNIX_EPOCH,
                        crtime: UNIX_EPOCH,
                        kind: self.files.get(i).unwrap().get_type(),
                        perm: 0o777,
                        nlink: 1,
                        uid: 501,
                        gid: 20,
                        rdev: 0,
                        flags: 0,
                        blksize: 512
                    }, 0);

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

        reply.attr(&TTL, &FileAttr {
            ino,
            size: self.files.get((ino as usize)-2).unwrap().get_size(),
            blocks: 1,
            atime: UNIX_EPOCH, // 1970-01-01 00:00:00
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: self.files.get((ino as usize)-2).unwrap().get_type(),
            perm: 0o777,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
            blksize: 512
        });
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, _flags: i32, _lock: Option<u64>, reply: ReplyData) {
        if ino == 1 {
            reply.error(2);
            return;
        }

        match self.files.get((ino as usize)-2).unwrap().get_type() {
            FileType::RegularFile => {
                reply.data(&"HELLO WORLD".as_bytes()[offset as usize..]);
            },
            _ => reply.error(2)
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        if ino != 1 {
            reply.error(2);
            return;
        }

        let default = [ ".", ".." ];

        for i in (offset as usize)..self.files.len()+2 {
            if i < 2 {
                reply.add(1, (i as i64)+1, FileType::Directory, default[i]);
                continue;
            }

            reply.add(i as u64, (i as i64)+1, self.files.get(i-2).unwrap().get_type(), self.files.get(i-2).unwrap().get_name());
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
            0       // filesystem ID
        );
    }
}
