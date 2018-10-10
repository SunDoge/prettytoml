extern crate prettytoml;

use prettytoml::prettify;

fn main() {
    println!("Hello, world!");
    let formatted = prettify("");
    println!("formatted: \n{}", &formatted);
}
