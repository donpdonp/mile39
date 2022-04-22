mod db;
mod net;
mod peer;
mod pool;


fn main() {
    let _db = db::open();
    let addr = "127.0.0.1:8888";
    let net = net::setup(addr);
    let mut pool = pool::new();
    println!("listening {}", addr);

    for stream in net.listener.incoming() {
        match stream {
            Err(_) => println!("err"),
            Ok(stream) => pool.push(||{peer::setup(stream)}),
        }
        println!("threadpool {}", pool.len())
    }
}

