extern crate hammer;

fn main() {
    println!("Serving '.' @ http://localhost:8000/");
    hammer::serve(".", "localhost:8000")
}
