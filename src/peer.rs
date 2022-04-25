use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::net::TcpStream;
use std::sync::Arc;

pub struct Peer {
    pub stream: TcpStream,
}

pub fn read(db: Arc<crate::db::Db>, stream: TcpStream) {
    let tx = db.db.begin_rw_txn().unwrap();
    let peer = Peer { stream: stream };
    peer.notice();
    for line in peer.feed_lines(){
        let parsed = json::parse(&line.unwrap()).unwrap();
        println!("next line {}", parsed);
        println!("key {}", parsed["key"]);
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
