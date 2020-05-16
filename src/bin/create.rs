fn main() {
    match blockfs::create() {
        Ok(()) => {},
        Err(x) => {
            println!("{}", x);
            std::process::exit(1);
        }
    }
}
