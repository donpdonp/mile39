use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::net::TcpStream;
use std::sync::Arc;

use lmdb::Transaction;

pub struct Peer {
    pub stream: TcpStream,
}

pub fn read(db: Arc<crate::db::Db>, stream: TcpStream) {
    let peer = Peer { stream: stream };
    peer.notice();
    for line in peer.feed_lines(){
        let mut parsed = json::parse(&line.unwrap()).unwrap();
        let key = parsed["key"].take_string().unwrap();
        let mut tx = db.env.begin_rw_txn().unwrap();
        let result = tx.get(db.db, &key);
        match result {
            Err(_) => println!("{:?}: not found", key),
            Ok(v) => println!("{:?}: {:?}", key, v)
        }
        //let c = tx.open_rw_cursor(db.db);
    }
}

impl Peer {
    pub fn notice(&self) {
        println!(
            "connected from {} to {}",
            self.stream.peer_addr().unwrap(),
            self.stream.local_addr().unwrap()
        )
    }
    pub fn feed_lines(self) -> Lines<BufReader<TcpStream>> {
        BufReader::new(self.stream).lines()
    }
}
