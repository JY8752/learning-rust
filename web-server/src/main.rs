// use web_server::server::single;
use web_server::server::multi;

fn main() {
    // single::listen("127.0.0.1", 7878).unwrap();
    multi::listen("127.0.0.1", 7878, 4).unwrap();
}
