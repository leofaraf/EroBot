use super::schema::*;

#[derive(Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub person_id: i32,
    pub photo_path: String,
    pub is_premium: bool
}

#[derive(Serialize, Queryable)]
pub struct Person {
    pub id: i32,
    pub title: i32,
    pub photo_path: String,
}