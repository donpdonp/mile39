mod db;
mod net;

fn main() {
    println!("Hello, world!");
    let _db = db::open();
    let addr = "127.0.0.1:8888";
    let net = net::setup(addr);
    for stream in net.listener.incoming() {
        match stream {
            Err(_) => println!("err"),
            Ok(stream) => net.connect(stream)
        }
    }
}
