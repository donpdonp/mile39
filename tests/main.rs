use crate::nouns::*;
use mile39::*;
use std::sync::Arc;

#[test]
fn write() {
    let db = mile39::db::open();
    let peer = peer::new(Arc::new(db));
    let cmd = r#"
        {"verb":"write", 
         "noun":{"Location":{"id":"ab13", 
                             "lat":1, 
                             "lng":2, 
                             "date":"2022-05-03", 
                             "user_id":"1234-5679"}}
        }"#;
    let result = peer.command(cmd).unwrap();
    assert_eq!("ok", result.msg);

    let cmd = r#"
        {"verb":"read", 
         "id":"ab13" 
        }"#;
    let result = peer.command(cmd).unwrap();
    assert_eq!("ok", result.msg);
    let noun = result.noun.unwrap();
    match noun {
        Nouns::Location(loc) => {
            assert_eq!("boo", loc.date)
        }
    }
}
