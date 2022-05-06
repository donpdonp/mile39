use lmdb::Cursor;
use lmdb::DatabaseFlags;
use lmdb::Environment;
use lmdb::Transaction;
use std::path::Path;

use crate::nouns;
use crate::nouns::*;
use crate::schema;

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

    pub fn write(&self, noun: &Nouns) -> String {
        let value = serde_json::to_value(noun).unwrap();
        let (noun_name, noun_value) = nouns::name_value(&value);
        let schema = self.schemas.get(&noun_name);
        if let Some(sch) = schema {
            for index in sch.indexes.iter() {
                let idx_db_name = self.schemas.db_name(&noun_name, &index.name);
                let index_db = self
                    .env
                    .create_db(Some(&idx_db_name), DatabaseFlags::empty())
                    .unwrap();
                let mut tx = self.env.begin_rw_txn().unwrap();
                let key = index.get_key(&noun_value);
                let result = tx.get(index_db, &key);
                match result {
                    Err(_) => match noun {
                        Nouns::Location(loc) => {
                            println!(
                                "writing {} key:{} value: {}",
                                idx_db_name,
                                String::from_utf8_lossy(&key),
                                &loc.id
                            );
                            tx.put(index_db, &key, &loc.id, lmdb::WriteFlags::empty())
                                .unwrap()
                        }
                    },
                    Ok(v) => println!(
                        "exists: {} {:?}: {:?}",
                        idx_db_name,
                        String::from_utf8_lossy(&key),
                        String::from_utf8_lossy(v)
                    ),
                }
                tx.commit().unwrap();
                self.dump(&index.name);
            }
        }
        noun_value.get("id").unwrap().as_str().unwrap().to_string()
    }

    pub fn dump(&self, name: &str) {
        println!("---db dump {} ---", name);
        let ddb = self.env.open_db(Some(&name)).unwrap();
        let ro = self.env.begin_ro_txn().unwrap();
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
}
