extern crate iron;
extern crate staticfile;
extern crate docopt;

use iron::Iron;
use staticfile::Static;
use docopt::Docopt;

const DEFAULT_PORT: &'static str = "8080";
const USAGE: &'static str = "
Serve files in the current directory via HTTP.

Usage: serve [<port>]
       serve (-h | --help)

Options:
    -h, --help  Show this screen";

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let port = if args.get_str("<port>") != "" {
        args.get_str("<port>").to_string()
    } else {
        DEFAULT_PORT.to_string()
    };

    let addr = format!("localhost:{}", port);

    match Iron::new(Static::new(".")).http(&*addr) {
        Ok(_) => {
            println!("Listening on port {}", port);
        },
        Err(_) => {
            println!("Could not start server on port {}. \
                      Make sure that you have the required permissions \
                      and that the port is not already in use.", port);
        },
    }
}
