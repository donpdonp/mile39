use std::path::Path;
use lmdb::Database;

pub fn open() -> Result<Database, lmdb::Error> {
    let builder = lmdb::Environment::new();
    let env = builder.open(Path::new("lmdb-data")).unwrap();
    let db = env.open_db(None);
    db
}
