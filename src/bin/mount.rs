fn main() {
    match blockfs::mount() {
        Ok(()) => {}
        Err(x) => {
            println!("{}", x);
            std::process::exit(1);
        }
    }
}
