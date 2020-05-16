use diesel::prelude::*;
use fuse::*;
use libc::*;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use time::Timespec;

embed_migrations!("migrations");

pub struct BlockFS {
    pub path: PathBuf,
    pub db: SqliteConnection,
}

impl BlockFS {
    pub fn open(path: &Path) -> BlockFS {
        let db_path = path.join("blockfs.db");
        let db_path_str = db_path.to_str().unwrap();
        let connection = SqliteConnection::establish(db_path_str).expect("should work");

        BlockFS {
            path: path.to_path_buf(),
            db: connection,
        }
    }

    pub fn create(path: &Path) -> BlockFS {
        let db_path = path.join("blockfs.db");
        let db_path_str = db_path.to_str().unwrap();
        let connection = SqliteConnection::establish(db_path_str).expect("should work");
        embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
            .expect("should work");

        let blocks_path = path.join("blocks");
        fs::create_dir(blocks_path).expect("should work");

        BlockFS {
            path: path.to_path_buf(),
            db: connection,
        }
    }
}

const TTL: Timespec = Timespec { sec: 1, nsec: 0 }; // 1 second

const CREATE_TIME: Timespec = Timespec {
    sec: 1381237736,
    nsec: 0,
}; // 2013-10-08 08:56

const ROOT_DIR_ATTR: FileAttr = FileAttr {
    ino: 1,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const FILES_DIR_ATTR: FileAttr = FileAttr {
    ino: 2,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const BLOCKS_DIR_ATTR: FileAttr = FileAttr {
    ino: 3,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const DATA_DIR_ATTR: FileAttr = FileAttr {
    ino: 4,
    size: 0,
    blocks: 0,
    atime: CREATE_TIME,
    mtime: CREATE_TIME,
    ctime: CREATE_TIME,
    crtime: CREATE_TIME,
    kind: FileType::Directory,
    perm: 0o755,
    nlink: 2,
    uid: 501,
    gid: 20,
    rdev: 0,
    flags: 0,
};

const HELLO_TXT_CONTENT: &'static str = "Hello World!\n";

impl Filesystem for BlockFS {
    fn init(&mut self, _req: &Request) -> Result<(), c_int> {
        println!("init");
        Ok(())
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        match ino {
            x if x == ROOT_DIR_ATTR.ino => {
                if offset == 0 {
                    reply.add(ROOT_DIR_ATTR.ino, 0, FileType::Directory, ".");
                    reply.add(ROOT_DIR_ATTR.ino, 1, FileType::Directory, "..");
                    reply.add(FILES_DIR_ATTR.ino, 2, FileType::Directory, "files");
                    reply.add(BLOCKS_DIR_ATTR.ino, 3, FileType::Directory, "blocks");
                    reply.add(DATA_DIR_ATTR.ino, 4, FileType::Directory, "data");
                }
                reply.ok();
            }
            x if x == FILES_DIR_ATTR.ino => {
                if offset == 0 {
                    reply.add(FILES_DIR_ATTR.ino, 0, FileType::Directory, ".");
                    reply.add(ROOT_DIR_ATTR.ino, 1, FileType::Directory, "..");
                }
                reply.ok();
            }
            x if x == BLOCKS_DIR_ATTR.ino => {
                if offset == 0 {
                    reply.add(BLOCKS_DIR_ATTR.ino, 0, FileType::Directory, ".");
                    reply.add(ROOT_DIR_ATTR.ino, 1, FileType::Directory, "..");
                }
                reply.ok();
            }
            x if x == DATA_DIR_ATTR.ino => {
                if offset == 0 {
                    reply.add(DATA_DIR_ATTR.ino, 0, FileType::Directory, ".");
                    reply.add(ROOT_DIR_ATTR.ino, 1, FileType::Directory, "..");
                }
                reply.ok();
            }
            _ => reply.error(ENOENT),
        }
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        println!("lookup parent: {} name: {}", parent, name.to_string_lossy());
        match parent {
            x if x == ROOT_DIR_ATTR.ino => match name.to_str() {
                Some("files") => reply.entry(&TTL, &FILES_DIR_ATTR, 0),
                Some("blocks") => reply.entry(&TTL, &BLOCKS_DIR_ATTR, 0),
                Some("data") => reply.entry(&TTL, &DATA_DIR_ATTR, 0),
                _ => reply.error(ENOENT),
            },
            _ => reply.error(ENOENT),
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match ino {
            x if x == ROOT_DIR_ATTR.ino => reply.attr(&TTL, &ROOT_DIR_ATTR),
            x if x == DATA_DIR_ATTR.ino  => reply.attr(&TTL, &DATA_DIR_ATTR),
            x if x == BLOCKS_DIR_ATTR.ino  => reply.attr(&TTL, &BLOCKS_DIR_ATTR),
            x if x == FILES_DIR_ATTR.ino  => reply.attr(&TTL, &FILES_DIR_ATTR),
            _ => reply.error(ENOENT),
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        reply: ReplyData,
    ) {
        if ino == 2 {
            reply.data(&HELLO_TXT_CONTENT.as_bytes()[offset as usize..]);
        } else {
            reply.error(ENOENT);
        }
    }
}
