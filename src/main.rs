extern crate iron;
extern crate staticfile;
extern crate argparse;

use iron::Iron;
use staticfile::Static;
use argparse::{ArgumentParser, Store};

const DEFAULT_PORT: &'static str = "8080";

fn main() {
    let mut port = "".to_string();
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Serve files in the current directory via HTTP.");

        parser.refer(&mut port)
            .add_argument("port", Store,
            "the port to listen on. Defaults to 8080");

        parser.parse_args_or_exit();
    }

    let port = if port == "" {DEFAULT_PORT.to_string()} else {port};
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
