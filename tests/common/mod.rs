use uuid::Uuid;

pub fn setup() {
    println!("I am setup")
}

pub fn id_generate() -> String {
    Uuid::new_v4().to_string()
}
