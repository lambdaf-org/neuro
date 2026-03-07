use actix_web::HttpResponse;
use actix_web::web;
use serde_json::json;
use uuid::Uuid;

use crate::models::app_state::AppState;
use crate::models::game::FinalizeSessionReq;
use crate::models::user::MiddlewareData;
use crate::models::validate::Validate;
use crate::repositories::game_repository;

#[utoipa::path(
    post,
    path = "/api/game/{code}",
    params(
        ("code" = String, Path, description = "Game code identifier"),
    ),
    responses(
        (status = 201, description = "Session created", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn start_game(
    code: web::Path<String>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::create_session(&state.sb_client, ext_data.user_id, code.into_inner())
        .await
    {
        Ok(id) => HttpResponse::Created().json(json!({"id": id})),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    patch,
    path = "/api/game/session/{id}",
    params(
        ("id" = Uuid, Path, example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    request_body = FinalizeSessionReq,
    responses(
        (status = 200, description = "Session finalized"),
        (status = 400, description = "Validation error", body = Object),
        (status = 403, description = "Session belongs to another user"),
        (status = 404, description = "Session not found", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn finalize_session(
    id: web::Path<Uuid>,
    body: web::Json<FinalizeSessionReq>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }
    let session_id = id.into_inner();
    let session = match game_repository::get_session(&state.sb_client, session_id).await {
        Ok(v) => v,
        Err(e) => return e.to_response(),
    };
    if session.user_id != ext_data.user_id {
        return HttpResponse::Forbidden().finish();
    }
    match game_repository::finalize_session(&state.sb_client, session_id, body.score).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/game/session/{id}",
    params(
        ("id" = Uuid, Path, example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    responses(
        (status = 200, description = "Session details", body = GameSession),
        (status = 403, description = "Session belongs to another user"),
        (status = 404, description = "Session not found", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn get_game_session(
    id: web::Path<Uuid>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::get_session(&state.sb_client, id.into_inner()).await {
        Ok(v) => {
            if v.user_id == ext_data.user_id {
                HttpResponse::Ok().json(v)
            } else {
                HttpResponse::Forbidden().finish()
            }
        }
        Err(e) => e.to_response(),
    }
}
