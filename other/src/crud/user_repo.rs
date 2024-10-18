/*

create
read by id
update by user id and repo name
delete by id
list
list by user id
delete all by user id
*/

use axum::{
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;


// Handler to create a user repository
async fn create_user_repo(Json(payload): Json<CreateUserRepo>) -> impl IntoResponse {
    let new_repo = UserRepo {
        id: uuid::Uuid::new_v4().to_string(),
        name: payload.name,
        user_id: payload.user_id,
    };

    unsafe {
        USER_REPOS.push(new_repo);
    }

    (StatusCode::CREATED, Json(new_repo))
}

// Handler to get a user repository by ID
async fn get_user_repo(Path(id): Path<String>) -> impl IntoResponse {
    unsafe {
        if let Some(repo) = USER_REPOS.iter().find(|repo| repo.id == id) {
            return (StatusCode::OK, Json(repo.clone()));
        }
    }

    StatusCode::NOT_FOUND
}

// Handler to delete a user repository by ID
async fn delete_user_repo(Path(id): Path<String>) -> impl IntoResponse {
    unsafe {
        if let Some(pos) = USER_REPOS.iter().position(|repo| repo.id == id) {
            USER_REPOS.remove(pos);
            return StatusCode::NO_CONTENT;
        }
    }

    StatusCode::NOT_FOUND
}

// Handler to get all user repositories
async fn get_user_repos() -> impl IntoResponse {
    unsafe {
        (StatusCode::OK, Json(USER_REPOS.clone()))
    }
}

// Handler to delete all user repositories
async fn delete_user_repos() -> impl IntoResponse {
    unsafe {
        USER_REPOS.clear();
    }

    StatusCode::NO_CONTENT
}
