mod db;
mod net;
mod nouns;
mod peer;
mod pool;
mod schema;

use std::io::BufRead;
use std::io::BufReader;
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
            Ok(stream) => pool.push(|| {
                let peer = peer::new(stream, dbc);
                peer.notice();
                for line in BufReader::new(&peer.stream).lines() {
                    peer.read(line.unwrap())
                }
            }),
        }
        println!("threadpool size {}", pool.len())
    }
}
