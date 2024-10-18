use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::Utc;
use dotenvy::dotenv;
use std::env;

use super::models::GitHub_User;
use crate::schema::github_users;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_user(conn: &mut PgConnection, id: &str, github_id: &str, username: &str) -> GitHub_User {

    let new_user = GitHub_User {
        id: id.to_string(),
        github_id: github_id.to_string(),
        username: username.to_string(),
        created_at: Utc::now(),
    };

    diesel::insert_into(github_users::table)
        .values(&new_user)
        .returning(GitHub_User::as_returning())
        .get_result(conn)
        .expect("Error saving new GitHub_User")

}
pub fn read_all_users(conn: &mut PgConnection,) -> QueryResult<Vec<GitHub_User>> {

    github_users::table.load::<GitHub_User>(conn)

}