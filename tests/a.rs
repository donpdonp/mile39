use mile39::*;
use std::sync::Arc;

#[test]
fn go() {
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
    peer.command(cmd).unwrap();
}
