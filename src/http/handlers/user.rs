use crate::db;
use crate::db::models::Group;
use crate::http::error::HttpError;
use crate::http::models::User;
use crate::http::{
    jwt::{create_jwt, decode_jwt, Claims},
    models,
};
use crate::{
    constants,
    messages::{websocket::WebsocketMessage, workers::WorkerMessageRequest},
};
use actix_web::{web, HttpResponse, Responder, Result};
use actix_ws::Message;
use deadpool_postgres::{Client, Pool};
use futures_util::StreamExt as _;
use tokio::sync::mpsc;
use tokio_postgres::NoTls;
use validator::Validate;

async fn create_user(
    create_user_request: web::Json<models::UserCreateRequest>,
    db_pool: web::Data<Pool<NoTls>>,
) -> Result<impl Responder, HttpError> {
    let client: Client<NoTls> = db_pool.get().await?;

    create_user_request.validate()?;

    let create_user_request = create_user_request.into_inner();

    let db_user = db::models::User::try_from(create_user_request)?;
    db_user.insert(&client).await?;
    let token = create_jwt(&db_user.id, &db_user.email)?;

    Ok(
        HttpResponse::Ok().json(serde_json::to_string(&models::LoginResponse {
            auth: token,
        })?),
    )
}

async fn get_user(
    path: web::Path<(uuid::Uuid,)>,
    db_pool: web::Data<Pool<NoTls>>,
) -> Result<impl Responder, HttpError> {
    let user_id = path.into_inner().0;

    let client = db_pool.get().await?;

    Ok(HttpResponse::Ok().json(
        serde_json::to_string(&User::from(
            db::models::User::get_by_id(&user_id, &client)
                .await?
                .ok_or(HttpError::NotFound)?,
        ))
        .expect("Failed to serialize user"),
    ))
}

async fn login(
    login_request: web::Json<models::LoginRequest>,
    db_pool: web::Data<Pool<NoTls>>,
) -> Result<HttpResponse, HttpError> {
    let client: Client<NoTls> = db_pool.get().await?;
    let login_request = login_request.into_inner();
    match db::models::User::get_by_email(&login_request.email, &client).await? {
        Some(user) => {
            if !bcrypt::verify(login_request.password, &user.password)? {
                return Err(HttpError::BadRequest("Unauthorized".to_string()));
            };

            let token = create_jwt(&user.id, &user.email)?;
            Ok(
                HttpResponse::Ok().json(serde_json::to_string(&models::LoginResponse {
                    auth: token,
                })?),
            )
        }
        None => Err(HttpError::BadRequest("User not found".to_string())),
    }
}

async fn ws(
    req: actix_web::HttpRequest,
    stream: web::Payload,
    state_sender: web::Data<mpsc::UnboundedSender<WorkerMessageRequest>>,
) -> Result<HttpResponse, actix_web::Error> {
    let claims = super::get_auth_claims(&req)?;
    let (res, session, mut stream) = actix_ws::handle(&req, stream)?;
    println!("WebSocket handshake successful!"); // Log when handshake is successful
    state_sender
        .send(WorkerMessageRequest::ClientLogin(claims.sub, session))
        .expect(constants::FAILED_TO_SEND_MESSAGE_TO_STATE_WORKER);

    actix_web::rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let received_message: WebsocketMessage = match serde_json::from_str(&text) {
                        Ok(msg) => msg,
                        Err(e) => {
                            eprintln!("Failed to deserialize message: {:?}", e);
                            continue;
                        }
                    };

                    let websocket_request_message =
                        if let WebsocketMessage::Request(msg) = received_message {
                            msg
                        } else {
                            println!("Received invalid message in handler");
                            continue;
                        };

                    if websocket_request_message.sender_id() != claims.sub {
                        println!("Unauthorized websocket message");
                        state_sender
                            .send(WorkerMessageRequest::ClientShutdown(claims.sub))
                            .expect(constants::FAILED_TO_SEND_MESSAGE_TO_STATE_WORKER);
                    }

                    state_sender
                        .send(WorkerMessageRequest::WebsocketMessage(
                            websocket_request_message,
                        ))
                        .expect("State mpsc sender crashed");
                }
                Ok(Message::Close(_)) => {
                    state_sender
                        .send(WorkerMessageRequest::ClientShutdown(claims.sub))
                        .expect(constants::FAILED_TO_SEND_MESSAGE_TO_STATE_WORKER);
                }
                _ => {
                    println!("Received other message type");
                }
            }
        }
    });

    Ok(res)
}

async fn get_unhandled_join_group_requests(
    req: actix_web::HttpRequest,
    db_pool: web::Data<Pool<NoTls>>,
) -> Result<HttpResponse, HttpError> {
    let client = db_pool.get().await?;
    let claims = super::get_auth_claims(&req)?;

    let group_requests =
        db::models::User::get_unhandled_groups_requests(&claims.sub, &client).await?;

    Ok(HttpResponse::Ok().json(
        group_requests
            .into_iter()
            .map(|(user, group)| {
                (
                    serde_json::to_string(&models::User::from(user))
                        .expect("Failed to serialize user"),
                    serde_json::to_string(&models::Group::from(group))
                        .expect("Failed to serialize group"),
                )
            })
            .collect::<Vec<(String, String)>>(),
    ))
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/user", web::post().to(create_user))
        .route("/user/login", web::post().to(login))
        .route("/user/ws", web::get().to(ws))
        .route(
            "/user/unhandled-group-requests",
            web::get().to(get_unhandled_join_group_requests),
        )
        .route("/user/{user_id}", web::get().to(get_user));
}
