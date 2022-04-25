use std::path::Path;
use lmdb::{Environment,Database};

#[derive(Debug)]
pub struct Db {
   pub env: Environment,
   pub db: Database
}

pub fn open() -> Db {
    let builder = lmdb::Environment::new();
    let env = builder.open(Path::new("lmdb-data")).unwrap();
    let db = env.open_db(None).unwrap();
    return Db{env: env, db: db}
}
