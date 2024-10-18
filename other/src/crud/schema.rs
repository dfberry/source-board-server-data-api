// Import necessary Diesel crates
use diesel::prelude::*;
use diesel::sql_types::{Text, Timestamp};
use diesel::table;

// Define the schema for each table using Diesel's DSL
table! {
    github_users_table (id) {
        id -> Text,
        github_id -> Text,
        username -> Text,
        created_at -> Timestamp
    }
}

table! {
    github_user_sessions (id) {
        id -> Text,
        user_id -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    github_user_tokens_table (id) {
        id -> Text,
        user_id -> Text,
        encrypted_access_token -> Text,
        created_at -> Timestamp,
        
    }
}

table! {
    app_user_watch_repos_table (id) {
        id -> Text,
        user_id -> Text,
        repo_name -> Text,
        created_at -> Timestamp,
    }
}

// Set up the relationships between tables
joinable!(github_user_sessions_table -> github_users_table (user_id));
joinable!(github_user_tokens_table -> github_users_table (user_id));
joinable!(app_user_watch_repos_table -> github_users_table (user_id));

// Allow tables to appear in the same query
allow_tables_to_appear_in_same_query!(
    github_users_table,
    github_user_sessions_table,
    github_user_tokens_table,
    app_user_watch_repos_table,
);

// Add unique constraints and default values where necessary
sql_function!(fn gen_random_uuid() -> Text);
sql_function!(fn now() -> Timestamp);
