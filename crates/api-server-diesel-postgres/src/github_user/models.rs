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
    pub github_id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}


#[derive(Insertable)]
#[diesel(table_name = crate::schema::github_users)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct New_GitHub_User {
    pub github_id: String,
    pub username: String,
}

impl New_GitHub_User {
    pub fn is_valid(&self) -> bool {
        !self.github_id.is_empty() && !self.username.is_empty()
    }
}