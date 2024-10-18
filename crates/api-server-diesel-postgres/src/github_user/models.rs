use diesel::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::github_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct GitHub_User {
    pub id: Uuid,
    pub gh_user_id: String,
    pub gh_user_name: String,
    pub created_at: DateTime<Utc>,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::github_users)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct New_GitHub_User {
    pub gh_user_id: String,
    pub gh_user_name: String,
}

impl New_GitHub_User {
    pub fn is_valid(&self) -> bool {
        !self.gh_user_id.is_empty() && !self.gh_user_name.is_empty()
    }
}