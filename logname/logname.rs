#![crate_id(name="logname", version="1.0.0", author="Benoit Benedetti")]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Benoit Benedetti <benoit.benedetti@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

/* last synced with: logname (GNU coreutils) 8.22 */

#![allow(non_camel_case_types)]

#![feature(macro_rules)]

extern crate getopts;
extern crate libc;

use std::io::print;
use std::os;
use std::str;
use libc::c_char;

#[path = "../common/util.rs"] mod util;

extern {
    // POSIX requires using getlogin (or equivalent code)
    pub fn getlogin() -> *libc::c_char;
}

unsafe fn get_userlogin() -> String {
    let login: *libc::c_char = getlogin();

    str::raw::from_c_str(login)
}

static NAME: &'static str = "logname";
static VERSION: &'static str = "1.0.0";

fn version() {
    println!("{} {}", NAME, VERSION);
}

#[allow(dead_code)]
fn main() { uumain(os::args()); }

pub fn uumain(args: Vec<String>) {
    let program = args.get(0).clone();

    //
    // Argument parsing
    //
    let opts = [
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => crash!(1, "Invalid options\n{}", f.to_err_msg())
    };

    if matches.opt_present("help") {
        version();
        println!("");
        println!("Usage:");
        println!("  {:s}", program);
        println!("");
        print(getopts::usage("print user's login name", opts).as_slice());
        return;
    }
    if matches.opt_present("version") {
        version();
        return;
    }

    exec();
}

fn exec() {
    unsafe {
        let userlogin = get_userlogin();
        println!("{:s}", userlogin);
    }
}
