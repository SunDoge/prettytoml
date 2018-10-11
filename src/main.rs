extern crate prettytoml;

use prettytoml::prettify;

fn main() {
    println!("Hello, world!");
    let formatted = prettify(
        r#"[database]
        server = "192.168.1.1"
        ports = [ 8001, 8001, 8002 ]
        connection_max = 5000
        enabled = true"#,
    );
    println!("formatted: \n>{}<", &formatted);
}
