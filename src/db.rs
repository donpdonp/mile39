use std::path::Path;
use lmdb::Environment;

#[derive(Debug)]
pub struct Db {
   pub db: Environment
}

pub fn open() -> Db {
    let builder = lmdb::Environment::new();
    let env = builder.open(Path::new("lmdb-data")).unwrap();
    let _ = env.open_db(None);
    return Db{db: env}
}
