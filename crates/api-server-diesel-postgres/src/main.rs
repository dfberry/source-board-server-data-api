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
    http::StatusCode,
    body::Body,
    extract::{Query, Json, Extension},
    Router
};

mod github_user;
mod schema;

use github_user::crud::{establish_connection, create_user, read_all_users};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/user", post(user_new_handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn user_new_handler() -> impl IntoResponse {
    create_user(&mut establish_connection(), "1", "github_1", "user1"); 
}
