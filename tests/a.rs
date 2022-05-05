use mile39::*;
use std::sync::Arc;

#[test]
#[should_panic]
fn go() {
    let db = mile39::db::open();
    let peer = peer::new(Arc::new(db));
    peer.read("{}")
}
