use diesel::prelude::*;
use diesel::pg::PgConnection;
use chrono::Utc;
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

use super::models::{GitHub_User, New_GitHub_User};
use crate::schema::github_users;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub fn create_user(conn: &mut PgConnection, new_user: &New_GitHub_User) -> Result<GitHub_User, diesel::result::Error> {

    diesel::insert_into(github_users::table)
        .values(new_user)
        .returning(GitHub_User::as_returning())
        .get_result(conn)

}
pub fn read_all_users(conn: &mut PgConnection,) -> Result<Vec<GitHub_User>, diesel::result::Error> {

    github_users::table.load::<GitHub_User>(conn)


}