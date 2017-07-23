#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate uuid;
extern crate docopt;
extern crate time;

use uuid::{Uuid, UuidV1Context};
use docopt::Docopt;

const USAGE: &'static str = "\
Usage:
 rsuuidgen [options]

Create a new UUID value.

Options:
 -r, --random     generate random-based uuid
 -t, --time       (in-completion)generate time-based uuid
 -V, --version    output version information and exit
 -h, --help       display this help and exit
";

fn main() {
    let argv = std::env::args();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv).deserialize())
        .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("rsuuidgen {}", "0.1.0");
    } else if args.flag_time {
        let ctx = UuidV1Context::new(42);
        let now = time::now().to_timespec();
        println!("{}", Uuid::new_v1(&ctx, now.sec as u64, (now.nsec / 100000) as u32, &[1,2,3,4,5,6]).unwrap());
    } else if args.flag_random {
        println!("{}", Uuid::new_v4());
    } else {
        println!("{}", Uuid::new_v4());
    }
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Args {
    flag_random: bool,
    flag_time: bool,
    flag_version: bool
}
