use mile39::db;
use mile39::net;
use mile39::peer;
use mile39::pool;

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
            Ok(stream) => {
                println!(
                    "connected from {} to {}",
                    stream.peer_addr().unwrap(),
                    stream.local_addr().unwrap()
                );
                pool.push(|| {
                    let peer = peer::new(dbc);
                    for line in BufReader::new(stream).lines() {
                        peer.command(&line.unwrap()).unwrap();
                    }
                })
            }
        }
        println!("threadpool size {}", pool.len())
    }
}
