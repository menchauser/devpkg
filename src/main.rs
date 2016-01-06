extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("S", "setup", "set up repository for package installation");
    opts.optflag("L", "list", "list installed packages");
    opts.optopt("I", "install", "install package from source URL", "URL");
    opts.optopt("F", "fetch", "fetch remote source package", "URL");
    opts.optopt("B", "build", "build fetched package", "URL");
    opts.optopt("c", "", "configure options", "OPTIONS");
    opts.optopt("m", "", "make options", "OPTIONS");
    opts.optopt("i", "", "install target", "TARGET");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    println!("Bye-bye");
}
