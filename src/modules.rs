#[derive(Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub password: String,
    pub email: Option<String>,
}
