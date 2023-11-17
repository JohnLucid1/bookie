#[derive(Debug)]
pub struct User {
    pub user_id: Option<i64>, // NOTE: by default none
    pub books_created: i32,
    pub chat_id: i64,
    pub is_admin: bool,
}

impl User {
    pub fn new(chat_id: i64) -> User {
        User {
            user_id: None,
            books_created: 0,
            chat_id,
            is_admin: false,
        }
    }
}
