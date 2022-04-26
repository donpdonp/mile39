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
            Ok(v) => println!("{:?}: {:?}", key, String::from_utf8_lossy(v)),
        }
        tx.commit().unwrap();
        dump(&db);
    }
}

pub fn dump(db: &crate::db::Db) {
    println!("---db dump---");
    let ro = db.env.begin_ro_txn().unwrap();
    {
        let mut c = ro.open_ro_cursor(db.db).unwrap();
        let mut count = 0;
        for ck in c.iter() {
            count += count;
            let k = String::from_utf8_lossy(ck.0);
            let v = String::from_utf8_lossy(ck.1);
            println!("{} {:?} {:?}", count, k, v);
        }
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
