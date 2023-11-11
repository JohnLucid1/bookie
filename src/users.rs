#[derive(Debug)]
pub struct User {
    pub user_id: Option<i64>,
    pub books_created: i32,
    pub chat_id: i64,
    pub is_admin: bool,
}

impl User {
}
