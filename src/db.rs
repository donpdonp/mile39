use crate::schema;
use lmdb::Environment;
use std::path::Path;

#[derive(Debug)]
pub struct Db {
    pub env: Environment,
    pub schemas: schema::Schemas,
    pub file_path: String,
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
        file_path: "jsonlake".to_owned(),
    };
}

impl Db {
    pub fn file_from_id(&self, id: &String) -> String {
        format!("{}/{}", self.file_path, id)
    }
}
