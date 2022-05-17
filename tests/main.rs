use crate::nouns::*;
use mile39::*;
use std::sync::Arc;

mod common;

#[test]
fn write_one_read_one() {
    let db = mile39::db::open();
    let peer = peer::new(Arc::new(db));
    let mut locations = common::random_locations(1);
    let location = locations.pop().unwrap();
    let location_id = location.id.to_owned();
    let cmd = command::Commands::Write(command::Write {
        id: "ab12".to_owned(),
        params: location,
    });
    let json = serde_json::to_string(&cmd).unwrap();
    let result = peer.command(&json).unwrap();
    assert_eq!("ok", result.msg);

    let cmd = command::Commands::Read(command::Read {
        id: "ab13".to_owned(),
        params: command::QueryById { id: location_id },
    });
    let json = serde_json::to_string(&cmd).unwrap();
    let result = peer.command(&json).unwrap();
    assert_eq!("ok", result.msg);
    let noun = result.noun.unwrap();
    match noun {
        Nouns::Location(loc) => {
            assert_eq!("2022-05-03", loc.date)
        }
    }
}

#[test]
fn write_many_read_by_id() {
    let db = mile39::db::open();
    let peer = peer::new(Arc::new(db));
    let locations = common::random_locations(10);
    println!("writing {} locations", locations.len());
    for location in &locations {
        let cmd = command::Commands::Write(command::Write {
            id: "ab12".to_owned(),
            params: location.clone(),
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let result = peer.command(&json).unwrap();
        assert_eq!("ok", result.msg);
    }
    for location in &locations {
        let cmd = command::Commands::Read(command::Read {
            id: "ab13".to_owned(),
            params: command::QueryById {
                id: location.id.to_owned(),
            },
        });
        let json = serde_json::to_string(&cmd).unwrap();
        let result = peer.command(&json).unwrap();
        assert_eq!("ok", result.msg);
    }
}
