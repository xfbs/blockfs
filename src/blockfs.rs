use fuse::Filesystem;
use std::path::{Path, PathBuf};
use diesel::prelude::*;
use std::fs;

embed_migrations!("migrations");

pub struct BlockFS {
    pub path: PathBuf,
    pub db: SqliteConnection,
}

impl BlockFS {
    pub fn open(path: &Path) -> BlockFS {
        let db_path = path.join("blockfs.db");
        let db_path_str = db_path.to_str().unwrap();
        let connection = SqliteConnection::establish(db_path_str)
            .expect("should work");

        BlockFS {
            path: path.to_path_buf(),
            db: connection,
        }
    }

    pub fn create(path: &Path) -> BlockFS {
        let db_path = path.join("blockfs.db");
        let db_path_str = db_path.to_str().unwrap();
        let connection = SqliteConnection::establish(db_path_str)
            .expect("should work");
        embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
            .expect("should work");

        let blocks_path = path.join("blocks");
        fs::create_dir(blocks_path)
            .expect("should work");

        BlockFS {
            path: path.to_path_buf(),
            db: connection,
        }
    }
}

impl Filesystem for BlockFS {
}
