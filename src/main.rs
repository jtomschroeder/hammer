
extern crate iron;
extern crate staticfile;
extern crate mount;

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    println!("Serving '.' @ http://localhost:8000/");

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(".")));

    Iron::new(mount).http("127.0.0.1:8000").unwrap();
}
