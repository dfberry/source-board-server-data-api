// @generated automatically by Diesel CLI.

diesel::table! {
    app_user_watch_repos (id) {
        id -> Uuid,
        github_user_id -> Uuid,
        org_repo_name -> Text,
        #[sql_name = "type"]
        #[max_length = 30]
        type_ -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    github_user_sessions (id) {
        id -> Uuid,
        github_user_id -> Uuid,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    github_user_tokens (id) {
        id -> Uuid,
        github_user_id -> Uuid,
        encrypted_access_token -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    github_users (id) {
        id -> Uuid,
        github_id -> Text,
        username -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(app_user_watch_repos -> github_users (github_user_id));
diesel::joinable!(github_user_sessions -> github_users (github_user_id));
diesel::joinable!(github_user_tokens -> github_users (github_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    app_user_watch_repos,
    github_user_sessions,
    github_user_tokens,
    github_users,
);
