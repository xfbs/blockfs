#[macro_use]
extern crate diesel_migrations;

use clap::{App, Arg};
use std::path::PathBuf;

pub mod blockfs;
pub use blockfs::BlockFS;

pub fn mount() -> std::io::Result<()> {
    let app = App::new("mount")
        .arg(
            Arg::with_name("option")
                .short("o")
                .long("option")
                .help("option passed to mount call"),
        )
        .arg(
            Arg::with_name("path")
                .index(1)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mountpoint")
                .index(2)
                .required(true)
                .takes_value(true),
        );

    let matches = app.get_matches();
    let path = matches
        .value_of_os("path")
        .map(|p| PathBuf::from(p))
        .unwrap();
    let blockfs = BlockFS::open(&path);
    let mountpoint = matches
        .value_of_os("mountpoint")
        .map(|p| PathBuf::from(p))
        .unwrap();
    fuse::mount(blockfs, &mountpoint, &[])
}

pub fn create() -> std::io::Result<()> {
    let app = App::new("create")
        .arg(
            Arg::with_name("blocksize")
                .short("b")
                .long("blocksize")
                .help("size of blocks"),
        )
        .arg(
            Arg::with_name("path")
                .index(1)
                .required(true)
                .takes_value(true),
        );

    let matches = app.get_matches();
    let path = matches
        .value_of_os("path")
        .map(|p| PathBuf::from(p))
        .unwrap();
    BlockFS::create(&path);

    Ok(())
}
