use crate::db_models::{Note, User};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct FetchUser;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Note>>")]
pub struct FetchUserArticles {
    pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Note>")]
pub struct CreateArticle {
    pub title: String,
    pub content: String,
    pub created_by: i32,
}
