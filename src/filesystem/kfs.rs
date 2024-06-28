use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, UNIX_EPOCH};
use fuser::{FileAttr, Filesystem, FileType, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEmpty, ReplyEntry, ReplyOpen, ReplyStatfs, ReplyWrite, Request};
use crate::filesystem::inter::node::{Data, Node};

const TTL: Duration = Duration::from_secs(1); // 1 second

pub struct KFS {
    files: Arc<Mutex<HashMap<u64, Node>>>,
    next_ino: u64
}

impl KFS {

    pub fn new(mut files: HashMap<u64, Node>) -> Self {
        /*
        let mut children = BTreeMap::new();

        for ino in files.keys() {
            if files.get(ino).unwrap().parent == 1 {
                children.insert(files.get(ino).as_ref().unwrap().data.name.clone(), ino.clone());
            }
        }

        files.insert(1, Node {
            data: Data {
                //name: ".".to_string(),
                kind: FileType::Directory,
                size: 0
            },
            children: Some(children),
            parent: 0
        });
        */

        let next_ino = (files.len() as u64)+1;

        Self {
            files: Arc::new(Mutex::new(files)),
            next_ino
        }
    }
}

impl Filesystem for KFS {

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        let files = self.files.lock().unwrap();
        //let children = self.files.lock().as_ref().unwrap().get(&parent).unwrap().children.as_ref().unwrap().clone();
        //let children = files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().clone();

        if let Some(ino) = files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().get(name.to_str().unwrap()) {
            reply.entry(&TTL, &FileAttr {
                ino: *ino,
                size: files.get(ino).as_ref().unwrap().data.size,
                blocks: 1,
                atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                mtime: UNIX_EPOCH,
                ctime: UNIX_EPOCH,
                crtime: UNIX_EPOCH,
                kind: files.get(ino).as_ref().unwrap().data.kind,
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


        reply.error(2); // Return error for unknown parent directory
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: ReplyAttr) {
        if let Some(child_node) = self.files.lock().as_ref().unwrap().get(&ino) {
            reply.attr(&TTL, &FileAttr {
                ino,
                size: child_node.data.size,
                blocks: 1,
                atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                mtime: UNIX_EPOCH,
                ctime: UNIX_EPOCH,
                crtime: UNIX_EPOCH,
                kind: child_node.data.kind,
                perm: 0o777,
                nlink: 1,
                uid: 501,
                gid: 20,
                rdev: 0,
                flags: 0,
                blksize: 512
            });
        }
    }

    fn read(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, _size: u32, _flags: i32, _lock: Option<u64>, reply: ReplyData) {
        /*
        if ino == 1 {
            reply.error(2);
            return;
        }
        */

        /*
        match self.files.get((ino as usize)-2).unwrap().get_type() {
            FileType::RegularFile => {
                reply.data(&"HELLO WORLD".as_bytes()[offset as usize..]);
            },
            _ => reply.error(2)
        }
        */

        reply.data(&"Hello World".as_bytes()[offset as usize..]);

        //reply.error(2);
    }

    fn readdir(&mut self, _req: &Request, ino: u64, _fh: u64, offset: i64, mut reply: ReplyDirectory) {
        let files = self.files.lock().unwrap();

        if offset == 0 {
            reply.add(1, 1, FileType::Directory, ".");
            reply.add(files.get(&ino).unwrap().parent, 2, FileType::Directory, "..");
        }

        let children = files.get(&ino).unwrap().children.as_ref().unwrap().clone();

        let mut i = offset;
        for (child_name, child_ino) in children.iter().skip(i as usize) {
            if let child_node = files.get(child_ino).unwrap() {
                reply.add(*child_ino, i+2, child_node.data.kind, child_name);
                i += 1;
            }
        }

        reply.ok();
    }

    fn mkdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, umask: u32, reply: ReplyEntry) {
        let mut files = self.files.lock().unwrap();

        if !files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().contains_key(name.to_str().unwrap()) {
            let ino = self.next_ino;
            self.next_ino += 1;

            files.insert(ino, Node {
                data: Data {
                    //name: name.to_str().unwrap().to_string(),
                    kind: FileType::Directory,
                    size: 0
                },
                children: Some(BTreeMap::new()),
                parent: parent
            });

            files.get_mut(&parent).as_mut().unwrap().children.as_mut().unwrap().insert(name.to_str().unwrap().to_string(), ino);

            reply.entry(&TTL, &FileAttr {
                ino,
                size: 0,
                blocks: 1,
                atime: UNIX_EPOCH, // 1970-01-01 00:00:00
                mtime: UNIX_EPOCH,
                ctime: UNIX_EPOCH,
                crtime: UNIX_EPOCH,
                kind: FileType::Directory,
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

        reply.error(17);
    }



    fn create(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, mode: u32, umask: u32, flags: i32, reply: ReplyCreate) {
        let mut files = self.files.lock().unwrap();

        if files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().contains_key(name.to_str().unwrap()) {
            reply.error(38);
            return;
        }

        let ino = self.next_ino;
        self.next_ino += 1;

        files.insert(ino, Node {
            data: Data {
                //name: name.to_str().unwrap().to_string(),
                kind: FileType::RegularFile,
                size: 0
            },
            children: None,
            parent: parent
        });

        files.get_mut(&parent).as_mut().unwrap().children.as_mut().unwrap().insert(name.to_str().unwrap().to_string(), ino);

        reply.created(&TTL, &FileAttr {
            ino,
            size: 0,
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
        }, 0, ino, 0);
    }

    fn write(&mut self, _req: &Request<'_>, ino: u64, fh: u64, offset: i64, data: &[u8], write_flags: u32, flags: i32, lock_owner: Option<u64>, reply: ReplyWrite) {
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

        let mut files = self.files.lock().unwrap();

        if !files.contains_key(&ino) {
            reply.error(38);
            return;
        }

        //let end_offset = offset as usize + data.len();
        //if (node.data.size as usize) < end_offset {
        //    node.data.resize(end_offset, 0);
        //}

        //node.data[offset as usize..end_offset].copy_from_slice(data);
        files.get_mut(&ino).unwrap().data.size = data.len() as u64;
        reply.written(data.len() as u32);

    }




    fn unlink(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let mut files = self.files.lock().unwrap();

        let ino = files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().get(name.to_str().unwrap()).unwrap().clone();
        files.get_mut(&parent).as_mut().unwrap().children.as_mut().unwrap().remove(name.to_str().unwrap());
        files.remove(&ino);
        reply.ok();

        //reply.error(38);
    }

    fn rmdir(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, reply: ReplyEmpty) {
        let mut files = self.files.lock().unwrap();
        //let children = files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().clone();

        let ino = files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().get(name.to_str().unwrap()).unwrap().clone();

        let children = files.get(&ino).as_ref().unwrap().children.as_ref().unwrap().clone();
        for (child_name, child_ino) in children.iter() {
            files.remove(child_ino);
        }

        files.get_mut(&parent).as_mut().unwrap().children.as_mut().unwrap().remove(name.to_str().unwrap());
        files.remove(&ino);
        reply.ok();

        //reply.error(38);
    }

    fn rename(&mut self, _req: &Request<'_>, parent: u64, name: &OsStr, newparent: u64, newname: &OsStr, flags: u32, reply: ReplyEmpty) {
        let mut files = self.files.lock().unwrap();

        let ino = files.get(&parent).as_ref().unwrap().children.as_ref().unwrap().get(name.to_str().unwrap()).unwrap().clone();
        files.get_mut(&parent).as_mut().unwrap().children.as_mut().unwrap().remove(name.to_str().unwrap());

        files.get_mut(&newparent).as_mut().unwrap().children.as_mut().unwrap().insert(newname.to_str().unwrap().to_string(), ino);

        reply.ok();
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
