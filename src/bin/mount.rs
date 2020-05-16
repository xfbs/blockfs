use fuse::mount;
use blockfs::BlockFS;
use std::path::PathBuf;

fn main() {
    let blockfs = BlockFS {};
    let path = PathBuf::from("./blockfs");
    mount(blockfs, &path, &[]);
}
