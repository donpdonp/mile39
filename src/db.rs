use crate::schema;
use lmdb::{Database, Environment};
use std::path::Path;

#[derive(Debug)]
pub struct Db {
    pub env: Environment,
    pub db: Database,
    pub schemas: schema::Schemas,
}

pub fn open() -> Db {
    let builder = lmdb::Environment::new();
    let env = builder.open(Path::new("lmdb-data")).unwrap();
    let db = env.open_db(None).unwrap();
    let schemas = schema::from_file("schema.json");
    return Db {
        env: env,
        db: db,
        schemas: schemas,
    };
}
