use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::Utc;
use dotenvy::dotenv;
use std::env;

use super::models::*;
use super::schema::*;

// Insert a new user into the user_table
pub fn insertDbUser(conn: &mut PgConnection, id: &str, github_id: &str, username: &str) -> User {
    use crate::schema::github_users_table::dsl::*;
    let new_user = User {
        id: id.to_string(),
        github_id: github_id.to_string(),
        username: username.to_string(),
    };
    diesel::insert_into(github_users_table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}

// Retrieve a user by their GitHub ID from the user_table
pub fn getDbUserByGithubId(conn: &PgConnection, github_id: &str) -> QueryResult<User> {
    use crate::schema::github_users_table::dsl::*;
    github_users_table.filter(github_id.eq(github_id)).first(conn)
}

// Update a token in the token_table
pub fn updateDbToken(conn: &PgConnection, token_id: &str, new_encrypted_access_token: &str) -> QueryResult<usize> {
    use crate::schema::token_table::dsl::*;
    diesel::update(token_table.filter(id.eq(token_id)))
        .set(encrypted_access_token.eq(new_encrypted_access_token))
        .execute(conn)
}

// Insert a new token into the token_table
pub fn insertDbToken(conn: &mut PgConnection, id: &str, user_id: &str, encrypted_access_token: &str) -> Token {
    use crate::schema::github_user_tokens_table::dsl::*;
    let new_token = Token {
        id: id.to_string(),
        user_id: user_id.to_string(),
        encrypted_access_token: encrypted_access_token.to_string(),
    };
    diesel::insert_into(github_user_tokens_table)
        .values(&new_token)
        .get_result(conn)
        .expect("Error saving new token")
}

// Delete a token by user ID from the token_table
pub fn deleteDbTokenByDbUserId(conn: &PgConnection, user_id: &str) -> QueryResult<usize> {
    use crate::schema::github_user_tokens_table::dsl::*;
    diesel::delete(github_user_tokens_table.filter(user_id.eq(user_id))).execute(conn)
}

// Retrieve a token by user ID from the token_table
pub fn getDbTokenByDbUserId(conn: &PgConnection, user_id: &str) -> QueryResult<Token> {
    use crate::schema::github_user_tokens_table::dsl::*;
    github_user_tokens_table.filter(user_id.eq(user_id)).first(conn)
}