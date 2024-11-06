use crate::http::models;
use crate::messages::websocket::{ApproveJoin, JoinGroupRequest};
use crate::{db, messages::workers::WorkerMessageRequest};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::Pool;
use tokio::sync::mpsc;
use tokio_postgres::NoTls;
use validator::Validate;

use crate::http::error::HttpError;

async fn create_group(
    req: actix_web::HttpRequest,
    create_group_request: web::Json<models::CreateGroupRequest>,
    db_pool: web::Data<Pool<NoTls>>,
) -> Result<HttpResponse, HttpError> {
    create_group_request.validate()?;
    let create_group_request = create_group_request.into_inner();
    let claims = super::get_auth_claims(&req)?;

    if claims.sub != create_group_request.group_owner_id {
        return Err(HttpError::BadRequest("Invalid group owner id".to_string()));
    }

    let group = db::models::Group::from(create_group_request);

    let mut client = db_pool.get().await?;
    group.insert(&mut client).await?;

    let create_group_response = models::Group::from(group);

    Ok(HttpResponse::Ok().json(serde_json::to_string(&create_group_response)?))
}

async fn get_group_users(
    req: actix_web::HttpRequest,
    path: web::Path<(uuid::Uuid,)>,
    db_pool: web::Data<Pool<NoTls>>,
) -> Result<HttpResponse, HttpError> {
    let group_id = path.into_inner().0;
    let claims = super::get_auth_claims(&req)?;

    let client = db_pool.get().await?;

    let user_group_ids = db::models::User::get_group_ids_of_user(&claims.sub, &client).await?;

    if !user_group_ids.contains(&group_id) {
        return Err(HttpError::Unauthorized);
    }

    let users = db::models::Group::get_users(&group_id, &client)
        .await?
        .into_iter()
        .map(|user| {
            let user = models::User::from(user);
            serde_json::to_string(&user).expect("Failed to serialize user")
        })
        .collect::<Vec<String>>();

    Ok(HttpResponse::Ok().json(users))
}

async fn create_join_group_request(
    req: actix_web::HttpRequest,
    path: web::Path<(uuid::Uuid,)>,
    mpsc_sender: web::Data<mpsc::UnboundedSender<WorkerMessageRequest>>,
    db_pool: web::Data<Pool<NoTls>>,
) -> Result<HttpResponse, HttpError> {
    let group_id = path.into_inner().0;
    let claims = super::get_auth_claims(&req)?;

    let client = db_pool.get().await?;
    let maybe_group = db::models::Group::get_by_id(&group_id, &client).await?;

    let group = if let Some(group) = maybe_group {
        group
    } else {
        return Err(HttpError::NotFound);
    };

    if group.created_by_user == claims.sub {
        return Err(HttpError::BadRequest(
            "User is owner of the group".to_string(),
        ));
    }

    db::models::Group::create_group_request(&group_id, &claims.sub, &client).await?;

    mpsc_sender
        .send(WorkerMessageRequest::WebsocketMessage(
            crate::messages::websocket::WebsocketMessageRequest::JoinGroup(JoinGroupRequest {
                sender_id: claims.sub,
                group_owner_id: group.created_by_user,
                group_id,
            }),
        ))
        .expect("Failed to send message to websocket worker");

    Ok(HttpResponse::Ok().finish())
}

async fn handle_join_request(
    req: actix_web::HttpRequest,
    approve_join: web::Json<models::ApproveJoin>,
    db_pool: web::Data<Pool<NoTls>>,
    mpsc_sender: web::Data<mpsc::UnboundedSender<WorkerMessageRequest>>,
) -> Result<HttpResponse, HttpError> {
    let approve_join = approve_join.into_inner();
    let claims = super::get_auth_claims(&req)?;
    let client = db_pool.get().await?;

    let user_group_ids = db::models::User::get_group_ids_of_user(&claims.sub, &client).await?;

    if !user_group_ids.contains(&approve_join.group_id) {
        return Err(HttpError::BadRequest("Not owner of the group".to_string()));
    }

    let affected_rows = db::models::Group::handle_group_request(
        &approve_join.group_id,
        &approve_join.candidate_id,
        approve_join.approved.into(),
        &client,
    )
    .await?;

    if affected_rows == 0 {
        return Err(HttpError::BadRequest(
            "User has no join request to the group".to_string(),
        ));
    }

    let websocket_approve_join = ApproveJoin {
        candidate_id: approve_join.candidate_id,
        group_owner: claims.sub,
        approved: approve_join.approved,
        group_id: approve_join.group_id,
    };

    mpsc_sender.send(WorkerMessageRequest::WebsocketMessage(
        websocket_approve_join.into(),
    ));

    Ok(HttpResponse::Ok().finish())
}

pub fn group_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/group", web::post().to(create_group))
        .route("/group/user/{group_id}", web::get().to(get_group_users))
        .route(
            "/group/user-join-request/{group_id}",
            web::post().to(create_join_group_request),
        );
}
