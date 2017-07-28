extern crate percent_encoding;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate docopt;
extern crate url;
use docopt::Docopt;
use url::percent_encoding::*;
use std::io::Write;

const USAGE: &'static str = "\
Usage:
 ende_url [options]

encode/decode url encode input.

Options:
 -d, --decode     decode mode
 -h, --help       display this help and exit
";

fn main() {
    let argv = std::env::args();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.argv(argv).deserialize())
        .unwrap_or_else(|e| e.exit());
    let stdin = std::io::stdin();
    loop {
        let mut buf = String::new();
        match stdin.read_line(&mut buf) {
            Ok(size) if size == 0 => break,
            Ok(_) => {
                if args.flag_decode {
                    std::io::stdout()
                        .write(percent_decode(buf.as_bytes()).decode_utf8().unwrap().as_bytes())
                        .unwrap();
                } else {
                    std::io::stdout()
                        .write(percent_encode(buf.as_bytes(), DEFAULT_ENCODE_SET)
                            .to_string()
                            .as_bytes())
                        .unwrap();
                }
            }
            Err(t) => panic!(t),
        }
    }
}
#[derive(Debug)]
#[derive(Deserialize)]
struct Args {
    flag_decode: bool,
}
