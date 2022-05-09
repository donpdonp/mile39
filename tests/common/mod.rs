use uuid::Uuid;

use crate::nouns::*;

pub fn setup() {
    println!("I am setup")
}

pub fn id_generate() -> String {
    Uuid::new_v4().to_string()
}

pub fn random_locations(count: u32) -> Vec<location::Location> {
    let mut bag = vec![];
    for _ in 0..count {
        let location = location::Location {
            id: id_generate(),
            lat: 1.0,
            lng: 2.0,
            date: "2022-05-02".to_owned(),
            user_id: "Abc".to_owned(),
        };
        bag.push(location);
    }
    return bag;
}
