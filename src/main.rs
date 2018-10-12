extern crate prettytoml;

use prettytoml::{prettify, prettify_from_file};

fn main() {
    println!("Hello, world!");
    let _ = prettify_from_file("Cargo.toml").expect("file not found");

    let formatted = prettify(
        r#"[database]
        server = "192.168.1.1"
        ports = [ 8001, 8001, 8002 ]
        connection_max = 5000
        enabled = true"#,
    );
    println!("formatted: \n>{}<", &formatted);
}
