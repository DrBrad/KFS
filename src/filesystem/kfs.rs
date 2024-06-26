use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};
use fuser::{FileAttr, Filesystem, FileType, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEntry, ReplyStatfs, ReplyWrite, Request};
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



    fn create(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, umask: u32, flags: i32, reply: ReplyCreate) {
        println!("CREATE FILE");
        /*
        let mut files = self.files.lock().unwrap();

        if parent != 1 {
            reply.error(ENOENT);
            return;
        }

        let ino = self.next_ino;
        self.next_ino += 1;

        let attr = FileAttr {
            ino,
            size: 0,
            blocks: 0,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind: FileType::RegularFile,
            perm: mode as u16,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
            blksize: BLOCK_SIZE as u32,
        };

        files.insert(ino, FileData { attr, data: Vec::new() });

        reply.created(&TTL, &attr, 0, ino, 0);
        */
    }

    fn write(&mut self, _req: &Request<'_>, ino: u64, fh: u64, offset: i64, data: &[u8], write_flags: u32, flags: i32, lock_owner: Option<u64>, reply: ReplyWrite) {
        println!("WRITE FILE");
        /*
        let mut files = self.files.lock().unwrap();

        if let Some(file_data) = files.get_mut(&ino) {
            let offset = offset as usize;
            if offset + data.len() > file_data.data.len() {
                file_data.data.resize(offset + data.len(), 0);
            }
            file_data.data[offset..offset + data.len()].copy_from_slice(data);
            file_data.attr.size = file_data.data.len() as u64;
            file_data.attr.mtime = SystemTime::now();
            file_data.attr.ctime = SystemTime::now();

            reply.written(data.len() as u32);
        } else {
            reply.error(ENOENT);
        }
        */
    }

    /*
    fn write(
        &mut self,
        _req: &Request,
        inode: u64,
        fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        #[allow(unused_variables)] flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        //EACCES: ::c_int = 13;
        //EBADF: ::c_int = 9;
        //reply.error(libc::EACCES);
        reply.error(9);
    }

    fn write_directory_content(&self, inode: Inode, entries: DirectoryDescriptor) {
        let path = Path::new(&self.data_dir)
            .join("contents")
            .join(inode.to_string());
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        bincode::serialize_into(file, &entries).unwrap();
    }
    */

    /*
    fn write_inode(&self, inode: &InodeAttributes) {
        let path = Path::new(&self.data_dir)
            .join("inodes")
            .join(inode.inode.to_string());
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap();
        bincode::serialize_into(file, inode).unwrap();
    }
    */

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
