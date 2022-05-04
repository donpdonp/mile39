use crate::schema;
use lmdb::Environment;
use std::path::Path;

#[derive(Debug)]
pub struct Db {
    pub env: Environment,
    pub schemas: schema::Schemas,
}

pub fn open() -> Db {
    let env = lmdb::Environment::new()
        .set_max_dbs(100)
        .open(Path::new("lmdb-data"))
        .unwrap();
    let schemas = schema::from_file("schema.json");
    return Db {
        env: env,
        schemas: schemas,
    };
}
