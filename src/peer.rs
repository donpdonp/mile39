use std::fs::File;
use std::sync::Arc;

use serde_json;

use crate::db;
use crate::nouns;
use crate::nouns::*;

pub struct Peer {
    pub user_id: Option<String>,
    pub db: Arc<db::Db>,
}

type PeerResult = Result<command::Response, &'static str>;

pub fn new(db: Arc<db::Db>) -> Peer {
    Peer {
        user_id: None,
        db: db,
    }
}

impl Peer {
    pub fn command(&self, line: &str) -> PeerResult {
        let command: command::Command = serde_json::from_str(&line).unwrap();
        println!("{}", serde_json::to_string(&command).unwrap());
        self.do_command(command)
    }
    pub fn do_command(&self, command: command::Command) -> PeerResult {
        match command.verb.as_str() {
            "write" => match &command.noun {
                Some(noun) => write_op(&self.db, noun),
                None => Err("write but no noun"),
            },
            "read" => match &command.id {
                Some(id) => read_op(&self.db, id),
                None => Err("read but no id"),
            },
            _ => Err("unknown command"),
        }
    }
}

pub fn read_op(db: &crate::db::Db, id: &String) -> PeerResult {
    let path = db.file_from_id(id);
    println!("read: {}", path);
    let reader = File::open(path).unwrap();
    let noun: nouns::Nouns = serde_json::from_reader(reader).unwrap();
    Ok(command::Response {
        msg: "ok".to_string(),
        noun: Some(noun),
    })
}
pub fn write_op(db: &crate::db::Db, noun: &Nouns) -> PeerResult {
    db.write(noun);
    Ok(command::Response {
        msg: "ok".to_string(),
        noun: None,
    })
}
