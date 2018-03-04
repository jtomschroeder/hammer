extern crate iron;
extern crate mount;
extern crate staticfile;

use std::path::Path;
use std::net::ToSocketAddrs;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

pub fn serve<P, A>(path: P, address: A)
where
    P: AsRef<Path>,
    A: ToSocketAddrs,
{
    let mut mount = Mount::new();
    mount.mount("/", Static::new(path.as_ref()));

    Iron::new(mount).http(address).unwrap();
}
