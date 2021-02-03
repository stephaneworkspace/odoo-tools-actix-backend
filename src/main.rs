use actix_web::{App, HttpServer, web};
/*use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt, Apiv2Schema, api_v2_operation,
    // use this instead of actix_web::web
    web::{self, Json},
};*/
use serde::{Serialize, Deserialize};
use actix_web::web::Json;

#[derive(Serialize, Deserialize)] /*Apiv2Schema*/
struct Todo {
    name: String,
    id: Option<i64>,
}

//#[api_v2_operation]
async fn post_account_write(body: Json<Todo>) -> Result<Json<Todo>, ()> {
    Ok(body)
}

async fn get_account_move() -> String {
    "Ok".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        //.wrap_api()
        .service(
            web::resource("/account-write")
                .route(web::post().to(post_account_write))
        )
        .service(
            web::resource("/account-move")
                .route(web::get().to(get_account_move))
        )
        //.with_json_spec_at("/api/spec")
        //.build()
    ).bind("127.0.0.1:8080")?
        .run().await
}