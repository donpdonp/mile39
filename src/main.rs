use mile39::db;
use mile39::net;
use mile39::peer;
use mile39::pool;

use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
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
            Ok(mut stream) => {
                println!(
                    "connected from {} to {}",
                    stream.peer_addr().unwrap(),
                    stream.local_addr().unwrap()
                );
                pool.push(move || {
                    let peer = peer::new(dbc);
                    let reader = BufReader::new(stream.try_clone().unwrap());
                    for line in reader.lines() {
                        let _result = peer.command(&line.unwrap()).unwrap();
                        stream.write(b"ok").unwrap();
                    }
                })
            }
        }
        println!("threadpool size {}", pool.len())
    }
}
