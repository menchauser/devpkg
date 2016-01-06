extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options] [URL]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("S", "", "set up repository for package installation");
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
