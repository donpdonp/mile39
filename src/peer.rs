use std::sync::Arc;

use lmdb::Cursor;
use lmdb::DatabaseFlags;
use lmdb::Transaction;

use serde_json;

use crate::db;
use crate::nouns;
use crate::nouns::*;

pub struct Peer {
    pub user_id: Option<String>,
    pub db: Arc<db::Db>,
}

pub fn new(db: Arc<db::Db>) -> Peer {
    Peer {
        user_id: None,
        db: db,
    }
}

pub fn do_command(db: &crate::db::Db, command: command::Command) {
    match command.verb.as_str() {
        "write" => write_op(db, &command.noun),
        _ => (),
    }
}

pub fn write_op(db: &crate::db::Db, noun: &Nouns) {
    let value = serde_json::to_value(noun).unwrap();
    let (noun_name, noun_value) = nouns::name_value(&value);
    let schema = db.schemas.get(&noun_name);
    if let Some(sch) = schema {
        for index in sch.indexes.iter() {
            println!("before");
            let index_db = db
                .env
                .create_db(Some(&index.name), DatabaseFlags::empty())
                .unwrap();
            let mut tx = db.env.begin_rw_txn().unwrap();
            println!("after");
            let key = index.get_key(&noun_value);
            println!(
                "schema found for {} {} {}",
                noun_name,
                &index.name,
                String::from_utf8_lossy(&key)
            );
            let result = tx.get(index_db, &key);
            match result {
                Err(_) => match noun {
                    Nouns::Location(loc) => {
                        println!(
                            "writing {} key:{}",
                            noun_name,
                            String::from_utf8_lossy(&key)
                        );
                        tx.put(index_db, &key, &loc.id, lmdb::WriteFlags::empty())
                            .unwrap()
                    }
                },
                Ok(v) => println!(
                    "{} {:?}: {:?}",
                    index.name,
                    String::from_utf8_lossy(&key),
                    String::from_utf8_lossy(v)
                ),
            }
            tx.commit().unwrap();
            dump(&db, &index.name);
        }
    }
}

pub fn dump(db: &crate::db::Db, name: &str) {
    println!("---db dump {} ---", name);
    let ddb = db.env.open_db(Some(&name)).unwrap();
    let ro = db.env.begin_ro_txn().unwrap();
    {
        let mut c = ro.open_ro_cursor(ddb).unwrap();
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
    pub fn read(&self, line: &str) {
        let command: command::Command = serde_json::from_str(&line).unwrap();
        println!("{}", serde_json::to_string(&command).unwrap());
        do_command(&self.db, command);
    }
}
