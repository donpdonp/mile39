use std::fs;
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

type PeerResult = Result<command::Response, String>;

pub fn new(db: Arc<db::Db>) -> Peer {
    Peer {
        user_id: None,
        db: db,
    }
}

impl Peer {
    pub fn command(&self, line: &str) -> PeerResult {
        let command: command::Commands = serde_json::from_str(&line).unwrap();
        println!("{}", serde_json::to_string(&command).unwrap());
        self.do_command(command)
    }

    pub fn do_command(&self, command: command::Commands) -> PeerResult {
        match command {
            command::Commands::Read(read) => read_op(&self.db, &read.params),
            command::Commands::Write(write) => write_op(&self.db, write.params),
        }
    }
}

pub fn read_op(db: &crate::db::Db, query: &command::QueryById) -> PeerResult {
    let path = db.file_from_id(&query.id);
    match File::open(&path) {
        Ok(reader) => {
            let noun: nouns::Nouns = serde_json::from_reader(reader).unwrap();
            Ok(command::Response {
                msg: "ok".to_string(),
                noun: Some(noun),
            })
        }
        Err(e) => {
            println!("read_op: {} {}", path, e);
            Err(e.to_string())
        }
    }
}

pub fn write_op(db: &crate::db::Db, location: location::Location) -> PeerResult {
    let wrapped_location = &Nouns::Location(location);
    let id = db.write(&wrapped_location);
    let path = db.file_from_id(&id);
    fs::write(path, serde_json::to_string(&wrapped_location).unwrap()).unwrap();
    Ok(command::Response {
        msg: "ok".to_string(),
        noun: None,
    })
}
