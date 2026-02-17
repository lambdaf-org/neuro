use actix_web::HttpResponse;
use actix_web::web;
use serde_json::json;
use uuid::Uuid;

use crate::models::app_state::AppState;
use crate::models::game::FinalizeSessionReq;
use crate::models::user::MiddlewareData;
use crate::models::validate::Validate;
use crate::repositories::game_repository;

pub async fn start_game(
    path: web::Path<String>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::create_session(&state.sb_client, ext_data.user_id, path.into_inner())
        .await
    {
        Ok(id) => HttpResponse::Created().json(json!({"id": id})),
        Err(e) => e.to_response(),
    }
}

pub async fn finalize_session(
    path: web::Path<Uuid>,
    body: web::Json<FinalizeSessionReq>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(json!({"errors": errors}));
    }

    let session_id = path.into_inner();

    // Ensure the session belongs to the authenticated user before finalizing
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

pub async fn get_game_session(
    path: web::Path<Uuid>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::get_session(&state.sb_client, path.into_inner()).await {
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
