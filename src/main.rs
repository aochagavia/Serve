extern crate iron;
extern crate staticfile;

use iron::Iron;
use staticfile::Static;
use std::env;

const DEFAULT_PORT: &'static str = "8080";

fn main() {
    let port = env::args().nth(1).unwrap_or(DEFAULT_PORT.into());
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
