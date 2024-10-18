//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

use axum::{
    response::{
        IntoResponse, 
        Response,
        Html
    }, 
    routing::{get,post},
    http::{StatusCode, header},
    body::Body,
    extract::{Query, Json, Extension},
    Router
};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;
use serde_json::json;

use diesel::deserialize::{self, FromSqlRow};
use dotenvy::dotenv;
use std::env;

mod github_user;
mod schema;

use github_user::crud::{establish_connection, create_user, read_all_users};
use github_user::models::{New_GitHub_User};


#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new().route("/user", post(user_new_handler))
        .route("/users", get(get_all_users_handler));;

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4001")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}


pub async fn user_new_handler(
    Json(payload): Json<New_GitHub_User>,
) -> impl IntoResponse {
    let mut connection = establish_connection();
    let github_user = payload;

    if !github_user.is_valid() {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap();
    }

    match create_user(&mut connection, &github_user){
        Ok(user) => {
            let json_repo = json!(user);

            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_repo.to_string()))
                .unwrap()
        },
        Err(e) => {
            let error_message = format!("Error: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_message))
                .unwrap()
        }
    }

    // let json_user = json!(user);

    // Response::builder()
    //     .header(http::header::CONTENT_TYPE, "application/json")
    //     .status(StatusCode::OK)
    //     .body(Body::from(json_user.to_string()))
    //     .unwrap()
}

pub async fn get_all_users_handler() -> impl IntoResponse {
    let mut connection = establish_connection();
    match read_all_users(&mut connection){
        Ok(users) => {
            let json_users = json!(users);

            Response::builder()
                .header(http::header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_users.to_string()))
                .unwrap()
        },
        Err(e) => {
            let error_message = format!("Error: {:?}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_message))
                .unwrap()
        }
    }

    // let json_users = json!(users);

    // Response::builder()
    //     .header(http::header::CONTENT_TYPE, "application/json")
    //     .status(StatusCode::OK)
    //     .body(Body::from(json_users.to_string()))
    //     .unwrap()
}