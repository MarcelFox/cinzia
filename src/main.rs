use axum::{
    routing::get, Json, Router
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    message: String
}


#[tokio::main]
async fn main() {
    let app: Router = Router::new().route("/", get(|| async {
        return Json(HelloResponse {
            message: "Hello Cinzia!".to_string()
        });
     }));
    let listener = tokio::net::TcpListener::bind("localhost:3030").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}