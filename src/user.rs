#[derive(Eq, Hash, PartialEq, Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
}
