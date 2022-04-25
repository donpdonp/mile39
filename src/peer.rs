use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::net::TcpStream;
use std::sync::Arc;

use lmdb::Cursor;
use lmdb::Transaction;

pub struct Peer {
    pub stream: TcpStream,
}

pub fn read(db: Arc<crate::db::Db>, stream: TcpStream) {
    let peer = Peer { stream: stream };
    peer.notice();
    for line in peer.feed_lines() {
        let mut parsed = json::parse(&line.unwrap()).unwrap();
        let key = parsed["key"].take_string().unwrap();
        let mut tx = db.env.begin_rw_txn().unwrap();
        let result = tx.get(db.db, &key);
        match result {
            Err(_) => {
                let value = "value";
                println!("writing {:?} {:?}", key, value);
                tx.put(db.db, &key, &value, lmdb::WriteFlags::empty())
                    .unwrap();
            }
            Ok(v) => println!("{:?}: {:?}", key, v),
        }
        {
            let c = tx.open_rw_cursor(db.db);
            for k in c.iter() {
                println!(
                    "{:?}",
                    String::from_utf8_lossy(k.get(None, None, 0).unwrap().1)
                );
            }
        }
        tx.commit();
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
