mod db;
mod net;
mod nouns;
mod peer;
mod pool;

use std::sync::Arc;

fn main() {
    let db = Arc::new(db::open());
    let addr = "127.0.0.1:8888";
    let net = net::setup(addr);
    let mut pool = pool::new();
    println!("listening {}", addr);

    for stream in net.listener.incoming() {
        let dbc = db.clone();
        match stream {
            Err(_) => println!("socket accept err"),
            Ok(stream) => pool.push(|| peer::read(dbc, stream)),
        }
        println!("threadpool {}", pool.len())
    }
}
