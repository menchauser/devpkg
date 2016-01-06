extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[derive(Debug)]
enum Command {
    None,
    Init,
    List,
    Install {
        url: String,
        configure_opts: Option<String>,
        make_opts: Option<String>,
        install_target: Option<String>,
    },
    Fetch {
        url: String,
    },
    Build {
        url: String,
        configure_opts: Option<String>,
        make_opts: Option<String>,
        install_target: Option<String>,
    },
}

// TODO: how to live w/o lifetime specifier?
fn init_options() -> Options {
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

    opts
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let program = args[0].clone();

    let opts = init_options();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let command = if matches.opt_present("S") {
        Command::Init
    } else if matches.opt_present("L") {
        Command::List
    } else if matches.opt_present("I") {
        let url = matches.opt_str("I").expect("You must at least give a URL.");
        Command::Install {
            url: url,
            configure_opts: matches.opt_str("c"),
            make_opts: matches.opt_str("m"),
            install_target: matches.opt_str("i"),
        }
    } else if matches.opt_present("F") {
        let url = matches.opt_str("F").expect("You must at least give a URL.");
        Command::Fetch { url: url }
    } else if matches.opt_present("B") {
        let url = matches.opt_str("F").expect("You must at least give a URL.");
        Command::Build {
            url: url,
            configure_opts: matches.opt_str("c"),
            make_opts: matches.opt_str("m"),
            install_target: matches.opt_str("i"),
        }
    } else {
        Command::None
    };

    println!("Executing command: {:?}", command);

    println!("Bye-bye");
}
