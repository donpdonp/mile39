use lmdb::{DatabaseFlags, Environment};
use std::path::Path;
use crate::schema;

#[derive(Debug)]
pub struct Db {
    pub env: Environment,
    pub schemas: schema::Schemas,
}

pub fn open() -> Db {
    let mut builder = lmdb::Environment::new();
    let wide_builder = builder.set_max_dbs(100);
    let env = wide_builder.open(Path::new("lmdb-data")).unwrap();
    let schemas = schema::from_file("schema.json");
    return Db { env: env, schemas: schemas };
}
