use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::net::TcpStream;
use std::sync::Arc;

use lmdb::Cursor;
use lmdb::Transaction;

use serde_json;

use crate::nouns::*;

pub struct Peer {
    pub stream: TcpStream,
}

pub fn read(db: Arc<crate::db::Db>, stream: TcpStream) {
    let peer = Peer { stream: stream };
    peer.notice();
    for line in peer.feed_lines() {
        let command: command::Command = serde_json::from_str(&line.unwrap()).unwrap();
        println!("{}", serde_json::to_string(&command).unwrap());
        do_command(&db, command);
        dump(&db);
    }
}

pub fn do_command(db: &crate::db::Db, command: command::Command) {
    match command.verb.as_str() {
        "write" => write_op(db, command.noun),
        _ => (),
    }
}

pub fn write_op(db: &crate::db::Db, noun: Nouns) {
    let mut tx = db.env.begin_rw_txn().unwrap();
    let schema = db.schemas.iter().find(|&s| s.noun == to_string(&noun));
    if let Some(s) = schema {
        println!("schema found for {}", s.noun);
    }
    let index = "abc123";
    let result = tx.get(db.db, &index);
    match result {
        Err(_) => match &noun {
            Nouns::Location(loc) => {
                println!("writing Location {:?}", index);
                tx.put(db.db, &index, &loc.id, lmdb::WriteFlags::empty())
                    .unwrap()
            }
        },
        Ok(v) => println!("found {:?}: {:?}", index, String::from_utf8_lossy(v)),
    }
    tx.commit().unwrap();
}

pub fn dump(db: &crate::db::Db) {
    println!("---db dump---");
    let ro = db.env.begin_ro_txn().unwrap();
    {
        let mut c = ro.open_ro_cursor(db.db).unwrap();
        let mut count = 0;
        for kv in c.iter() {
            count += count;
            let k = String::from_utf8_lossy(kv.0);
            let v = String::from_utf8_lossy(kv.1);
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
