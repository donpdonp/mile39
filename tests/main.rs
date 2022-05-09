use crate::nouns::*;
use mile39::*;
use std::sync::Arc;

mod common;

#[test]
fn write() {
    common::setup();
    let db = mile39::db::open();
    let peer = peer::new(Arc::new(db));
    let location = location::Location {
        id: common::id_generate(),
        lat: 1.0,
        lng: 2.0,
        date: "2022-05-02".to_owned(),
        user_id: "Abc".to_owned(),
    };
    let cmd = command::Command {
        verb: "write".to_owned(),
        noun: Some(Nouns::Location(location)),
        id: None
    };
    let json = serde_json::to_string(&cmd).unwrap();
    let result = peer.command(&json).unwrap();
    assert_eq!("ok", result.msg);

    let cmd = command::Command {
        verb: "read".to_owned(),
        noun: None,
        id: Some("ab13".to_owned())
    };
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
