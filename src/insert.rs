struct User {
    id: u32,
    username: String,
    email: String,
}

// insert into users(id, username, email) values('fze', 'efz', 'fezf');
pub fn insert (query: String) {
    println!("the insert query is: {}", query);
    let query_parts: Vec<&str> = query.split(" ").collect();
}