mod db;
mod net;
mod nouns;
mod peer;
mod pool;
mod schema;

use std::sync::Arc;

fn main() {
    let db = Arc::new(db::open());
    let schemas = schema::from_file("schema.json");

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
        println!("threadpool size {}", pool.len())
    }
}
