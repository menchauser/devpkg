#[macro_use]
extern crate log;
extern crate env_logger;
extern crate getopts;

use getopts::Options;
use std::env;
use std::io;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};

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

const DB_FILE: &'static str = "/usr/local/.devpkg/db";

struct Database {
    path: String,
}

impl Database {
    fn new() -> Database {
        Database { path: String::from(DB_FILE) }
    }

    fn open(&self) -> io::Result<File> {
        File::open(&self.path)
    }

    fn init(&self) -> io::Result<()> {
        debug!("Creating new database file");
        // TODO: check R and X permissions
        let db_file_path = Path::new(&self.path);
        let db_dir_path = db_file_path.parent().unwrap();

        if !db_dir_path.exists() {
            debug!("Directory {:?} does not exist. Creating new", &db_dir_path);
            try!(fs::create_dir_all(db_dir_path));
        }

        // TODO: check W permission
        if !db_file_path.exists() {
            debug!("File {:?} does not exist. Creating new", &db_file_path);
            try!(File::create(DB_FILE));
        }

        Ok(())
    }

    fn load(&self) -> io::Result<String> {
        debug!("Loading data from package database");
        let mut f = try!(self.open());
        let mut data = String::new();
        try!(f.read_to_string(&mut data));

        Ok(data)
    }

    fn find(&self, url: &str) -> io::Result<bool> {
        let data = try!(self.load());
        match self.load() {
            Ok(s) => return Ok(s.contains(url)),
            Err(e) => return Err(e),
        }
    }

    fn update(&self, url: &str) -> io::Result<()> {
        if self.find(url).unwrap_or(false) {
            info!("Already recorded as installed: {}", url);
        }

        let mut db = try!(self.open());
        try!(db.write_all(url.as_bytes()));

        Ok(())
    }

    fn list<W: Write>(&self, w: &mut W) -> io::Result<()> {
        let data = try!(self.load());
        try!(w.write_all(data.as_bytes()));
        Ok(())
    }
}


fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

// TODO: how to pass reference to Options outside, that outlive function?
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

fn execute_command(command: Command) -> Result<(), Box<std::error::Error>> {
    println!("Executing command {:?}", command);
    match command {
        Command::None => println!("Unknown command"),
        Command::Init => {
            info!("Initializing database");
            try!(Database::new().init());
        }
        Command::List => {
            info!("Listing package source database");
            try!(Database::new().list(&mut io::stdout()));
        }
        Command::Install { url, configure_opts, make_opts, install_target } => {
            println!("Installing package from {}.", url)
        }
        Command::Fetch { url } => println!("Fetching package from {}.", url),
        Command::Build { url, .. } => println!("Building package from url {}.", url),
    }
    Ok(())
}

fn main() {
    env_logger::init().unwrap();

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

    match execute_command(command) {
        Ok(_) => debug!("Command successfully executed"),
        Err(e) => debug!("Error: {:?}", e),
    }

    println!("Bye-bye");
}
