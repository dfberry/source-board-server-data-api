// Import necessary Diesel crates
use diesel::prelude::*;
use diesel::sql_types::{Text, Timestamp};
use diesel::table;

#[derive(Queryable, Insertable)]
#[table_name = "github_users"]
struct User {
    id: String,
    github_id: String,
    username: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[table_name = "github_user_sessions"]
struct Session {
    id: String,
    user_id: String,
    expires_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable)]
#[table_name = "github_user_tokens"]
struct Token {
    id: String,
    user_id: String,
    encrypted_access_token: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "app_user_watch_repos"]
struct UserWatchRepo {
    id: String,
    user_id: String,
    repo_name: String,
    created_at: chrono::NaiveDateTime,
}