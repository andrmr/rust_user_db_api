#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;

use api::model::{ResponseReason, Response};
use axum::extract;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Extension, Router, Json};
use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use std::str::FromStr;
use std::sync::Arc;
use std::net::SocketAddr;

use api::*;

#[tokio::main]
async fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let db = match database::db_connect().await {
        Ok(db) => db,
        Err(e) => {
            error!("DB: connection error {e}");
            return
        }
    };

    let app = Router::new()
        .route("/register", post(register))
        .route("/authenticate", post(authenticate))
        .layer(Extension(Arc::new(db)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    debug!("API: listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn register(db: Extension<Arc<DatabaseConnection>>, extract::Json(register_req): Json<model::RegisterRequest>) -> impl IntoResponse {
    if let Ok(Some(_)) = user::Entity::find_by_id(register_req.username.clone()).one(db.as_ref()).await {
        return (StatusCode::OK, Json(Response::bad(ResponseReason::UsernameAlreadyExists)));
    };
    
    let hash = bcrypt::hash(&register_req.password, bcrypt::DEFAULT_COST).unwrap();
    let salt = bcrypt::HashParts::from_str(&hash).unwrap().get_salt();

    let new_user = user::ActiveModel {
        username: Set(register_req.username),
        password: Set(hash),
        salt: Set(salt),
        ..Default::default()        
    };

    let (status_code, response) = match user::Entity::insert(new_user).exec(db.as_ref()).await {
        Err(e) => {
            error!("DB: internal error {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Response::bad(ResponseReason::InternalError))
        }
        _ => (StatusCode::CREATED, Response::good())
    };

    (status_code, Json(response))
}

async fn authenticate(db: Extension<Arc<DatabaseConnection>>, extract::Json(authenticate_req): Json<model::AuthenticateRequest>) -> impl IntoResponse {
    let (status_code, response) = match user::Entity::find_by_id(authenticate_req.username.clone()).one(db.as_ref()).await {
        Ok(Some(user)) => {
            match bcrypt::verify(&authenticate_req.password, &user.password) {
                Ok(true) => (StatusCode::OK, Response::good()),
                Ok(false) => (StatusCode::OK, Response::bad(ResponseReason::InvalidUsernameOrPassword)),
                Err(e) => {
                    error!("BCrypt: internal error {e}");
                    (StatusCode::INTERNAL_SERVER_ERROR, Response::bad(ResponseReason::InternalError))
                },
            }            
        },
        Ok(None) => (StatusCode::OK, Response::bad(ResponseReason::UsernameNotFound)),
        Err(e) => {
            error!("DB: internal error {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Response::bad(ResponseReason::InternalError))
        }
    };

    (status_code, Json(response))
}