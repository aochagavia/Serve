extern crate iron;
extern crate staticfile;

use iron::Iron;

use staticfile::Static;

use std::env;

fn main() {
    let default_port = "8080".to_string();
    let port = env::args().nth(1).unwrap_or(default_port);
    let addr = format!("localhost:{}", port);

    println!("Listening on port {}", port);
    Iron::new(Static::new(".")).http(&*addr).unwrap();
}
