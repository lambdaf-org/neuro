use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Result;
use actix_web::web;
use futures_util::StreamExt;
use serde_json::json;
use uuid::Uuid;

use crate::models::anticheat::AnticheatVerdict;
use crate::models::app_state::AppState;
use crate::models::game::CreateGameEventReq;
use crate::models::game::FinalizeSessionReq;
use crate::models::game::FinalizeSessionResultRes;
use crate::models::game::GameEventAck;
use crate::models::game::GameEventWsAck;
use crate::models::game::GameEventWsAnticheat;
use crate::models::game::GameEventWsError;
use crate::models::user::MiddlewareData;
use crate::models::validate::Validate;
use crate::repositories::anticheat_repository;
use crate::repositories::game_repository;
use crate::services::anticheat;
use crate::services::anticheat::EventTuple;
use crate::services::anticheat_runtime;
use crate::services::scoring::{SCORING_VERSION, ScoreOutcome, score_game};
use crate::services::scoring_input::{self, TrialResolutionError};

#[utoipa::path(
    post,
    path = "/api/game/{code}",
    params(
        ("code" = String, Path, description = "Game code identifier"),
    ),
    responses(
        (status = 201, description = "Session created", body = Object),
        (status = 409, description = "Constraint violation", body = Object),
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
    match anticheat_repository::is_user_banned(&state.sb_client, ext_data.user_id).await {
        Ok(true) => return banned_response(),
        Ok(false) => {}
        Err(e) => return e.to_response(),
    }

    match game_repository::create_session(&state.sb_client, ext_data.user_id, code.into_inner())
        .await
    {
        Ok(id) => HttpResponse::Created().json(json!({"id": id})),
        Err(e) => e.to_response(),
    }
}

fn banned_response() -> HttpResponse {
    HttpResponse::Forbidden().json(json!({
        "error": "banned",
        "message": "Account suspended due to anti-cheat violation.",
    }))
}

#[utoipa::path(
    patch,
    path = "/api/game/session/{id}",
    params(
        ("id" = Uuid, Path, example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    request_body = FinalizeSessionReq,
    responses(
        (status = 200, description = "Session finalized", body = FinalizeSessionResultRes),
        (status = 400, description = "Validation error", body = Object),
        (status = 403, description = "Session belongs to another user"),
        (status = 404, description = "Session not found", body = Object),
        (status = 409, description = "Constraint violation", body = Object),
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
    if session.status != "in_progress" {
        return HttpResponse::Conflict().json(json!({"error": "session already finalized"}));
    }

    let scoring_trials = match session.game_code.as_str() {
        "gf" | "gv" if body.trials.is_empty() => {
            return HttpResponse::BadRequest()
                .json(json!({"error": "selected option trials are required"}));
        }
        "gf" => {
            match scoring_input::build_pattern_logic_trials(&state.sb_client, &body.trials).await {
                Ok(trials) => trials,
                Err(error) => return trial_resolution_error_to_response(error),
            }
        }
        "gv" => {
            match scoring_input::build_mental_rotation_trials(&state.sb_client, &body.trials).await
            {
                Ok(trials) => trials,
                Err(error) => return trial_resolution_error_to_response(error),
            }
        }
        _ => body.trials.clone(),
    };

    let (status, metric_value, metrics, scoring_version) = if !scoring_trials.is_empty() {
        match score_game(&session.game_code, &scoring_trials) {
            ScoreOutcome::Valid { metric, metrics } => {
                ("completed", Some(metric), metrics, SCORING_VERSION)
            }
            ScoreOutcome::Invalid { reason } => {
                ("invalid", None, json!({"reason": reason}), SCORING_VERSION)
            }
        }
    } else {
        ("completed", body.score, json!({}), 0)
    };

    let result = FinalizeSessionResultRes {
        status: status.to_string(),
        metric_value,
        metrics: metrics.clone(),
        scoring_version,
    };

    match game_repository::finalize_session(
        &state.sb_client,
        session_id,
        status,
        metric_value,
        metrics,
        scoring_version,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().json(result),
        Err(e) => e.to_response(),
    }
}

fn trial_resolution_error_to_response(error: TrialResolutionError) -> HttpResponse {
    match error {
        TrialResolutionError::BadRequest(message) => {
            HttpResponse::BadRequest().json(json!({"error": message}))
        }
        TrialResolutionError::Repository(error) => error.to_response(),
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
        (status = 409, description = "Constraint violation", body = Object),
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

#[utoipa::path(
    get,
    path = "/api/game/stats",
    responses(
        (status = 200, description = "Player stats", body = PlayerStats),
        (status = 409, description = "Constraint violation", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn get_player_stats(
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::get_player_stats(&state.sb_client, ext_data.user_id).await {
        Ok(stats) => HttpResponse::Ok().json(stats),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/game/{code}/recent",
    params(
        ("code" = String, Path, description = "Game code"),
    ),
    responses(
        (status = 200, description = "Recent sessions", body = Vec<LeaderboardEntry>),
        (status = 409, description = "Constraint violation", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn get_recent_sessions(
    path: web::Path<String>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let game_code = path.into_inner();
    match game_repository::get_recent_sessions(&state.sb_client, ext_data.user_id, &game_code, 10)
        .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/game/leaderboard/{code}",
    params(
        ("code" = String, Path, description = "Game code"),
    ),
    responses(
        (status = 200, description = "Leaderboard entries", body = Vec<LeaderboardEntry>),
        (status = 409, description = "Constraint violation", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
)]
pub async fn get_game_leaderboard(
    path: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let game_code = path.into_inner();

    match game_repository::get_leaderboard(&state.sb_client, &game_code).await {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/game/metadata/{code}",
    params(
        ("code" = String, Path, description = "Game code"),
    ),
    responses(
        (status = 200, description = "Game metadata", body = GameMetadata),
        (status = 404, description = "Game not found", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn get_game_metadata(
    code: web::Path<String>,
    state: web::Data<AppState>,
) -> HttpResponse {
    match game_repository::get_metadata_by_code(&state.sb_client, &code.into_inner()).await {
        Ok(v) => HttpResponse::Ok().json(v),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/game/session/{id}/events",
    params(
        ("id" = Uuid, Path, example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    request_body = CreateGameEventReq,
    responses(
        (status = 201, description = "Event created", body = GameEventAck),
        (status = 400, description = "Validation error", body = Object),
        (status = 403, description = "Session belongs to another user"),
        (status = 404, description = "Session not found", body = Object),
        (status = 422, description = "Operation failed", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn create_game_event(
    id: web::Path<Uuid>,
    body: web::Json<CreateGameEventReq>,
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
    match persist_game_event(&state, session_id, ext_data.user_id, &body).await {
        Ok(event) => HttpResponse::Created().json(event),
        Err(e) => e.to_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/game/session/{id}/events/ws",
    params(
        ("id" = Uuid, Path, example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    responses(
        (
            status = 101,
            description = "WebSocket upgrade accepted. After connecting, send text frames containing JSON shaped like CreateGameEventReq. Successful writes yield GameEventWsAck frames and invalid payloads yield GameEventWsError frames. Browser clients may authenticate with the token query parameter because the middleware already supports token=... for WebSocket handshakes."
        ),
        (status = 403, description = "Session belongs to another user"),
        (status = 404, description = "Session not found", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn stream_game_events(
    id: web::Path<Uuid>,
    req: HttpRequest,
    body: web::Payload,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> Result<HttpResponse> {
    let session_id = id.into_inner();
    let session = match game_repository::get_session(&state.sb_client, session_id).await {
        Ok(v) => v,
        Err(e) => return Ok(e.to_response()),
    };

    if session.user_id != ext_data.user_id {
        return Ok(HttpResponse::Forbidden().finish());
    }

    let (response, mut ws_session, mut msg_stream) = actix_ws::handle(&req, body)?;
    let user_id = ext_data.user_id;
    let app_state = state.clone();
    let game_code = session.game_code.clone();

    actix_web::rt::spawn(async move {
        let mut history: Vec<EventTuple> = Vec::new();
        while let Some(item) = msg_stream.next().await {
            let message = match item {
                Ok(message) => message,
                Err(error) => {
                    log::error!("WebSocket stream error for game session {session_id}: {error}");
                    let _ = ws_session.close(None).await;
                    break;
                }
            };

            let should_continue = handle_game_event_ws_message(
                &mut ws_session,
                message,
                &app_state,
                session_id,
                user_id,
                &game_code,
                &mut history,
            )
            .await;

            if !should_continue {
                break;
            }
        }
    });

    Ok(response)
}

async fn handle_game_event_ws_message(
    ws_session: &mut actix_ws::Session,
    message: actix_ws::Message,
    state: &web::Data<AppState>,
    session_id: Uuid,
    user_id: Uuid,
    game_code: &str,
    history: &mut Vec<EventTuple>,
) -> bool {
    match message {
        actix_ws::Message::Text(text) => {
            handle_game_event_ws_text(
                ws_session, state, session_id, user_id, game_code, history, &text,
            )
            .await
        }
        actix_ws::Message::Ping(bytes) => {
            let _ = ws_session.pong(&bytes).await;
            true
        }
        actix_ws::Message::Close(reason) => {
            let _ = ws_session.clone().close(reason).await;
            false
        }
        actix_ws::Message::Binary(_) | actix_ws::Message::Continuation(_) => {
            send_game_event_ws_error(ws_session, "only text frames are supported").await;
            true
        }
        actix_ws::Message::Pong(_) | actix_ws::Message::Nop => true,
    }
}

async fn handle_game_event_ws_text(
    ws_session: &mut actix_ws::Session,
    state: &web::Data<AppState>,
    session_id: Uuid,
    user_id: Uuid,
    game_code: &str,
    history: &mut Vec<EventTuple>,
    text: &str,
) -> bool {
    let event = match parse_game_event_ws_payload(text) {
        Ok(event) => event,
        Err(error) => {
            send_game_event_ws_error(ws_session, &error).await;
            return true;
        }
    };

    match persist_game_event(state, session_id, user_id, &event).await {
        Ok(stored_event) => send_game_event_ws_ack(ws_session, stored_event).await,
        Err(error) => {
            send_game_event_ws_error(ws_session, &error.to_string()).await;
            return true;
        }
    }

    let new_event: EventTuple = (event.event_value, event.correct);
    let verdict = anticheat::evaluate_round(game_code, history, new_event);
    history.push(new_event);

    if verdict.flags.is_empty() {
        return true;
    }

    if let Err(e) =
        anticheat_runtime::apply_verdict(state, user_id, Some(session_id), &verdict).await
    {
        log::error!("apply_verdict failed: {e}");
    }

    send_anticheat_frame(ws_session, &verdict).await;

    if verdict.is_ban() {
        anticheat_runtime::invalidate_session_for_ban(state, session_id, &verdict).await;
        let _ = ws_session.clone().close(None).await;
        return false;
    }

    true
}

async fn send_anticheat_frame(ws_session: &mut actix_ws::Session, verdict: &AnticheatVerdict) {
    let payload = json!(GameEventWsAnticheat {
        message_type: String::from("anticheat"),
        action: verdict.action.as_str().to_string(),
        flags: verdict.flags_summary(),
    })
    .to_string();
    let _ = ws_session.text(payload).await;
}

fn parse_game_event_ws_payload(text: &str) -> std::result::Result<CreateGameEventReq, String> {
    let event = serde_json::from_str::<CreateGameEventReq>(text)
        .map_err(|_| String::from("invalid event payload"))?;

    event.validate().map_err(|errors| errors.join(", "))?;

    Ok(event)
}

async fn send_game_event_ws_ack(ws_session: &mut actix_ws::Session, event: GameEventAck) {
    let payload = json!(GameEventWsAck {
        message_type: String::from("event_stored"),
        event,
    })
    .to_string();
    let _ = ws_session.text(payload).await;
}

async fn send_game_event_ws_error(ws_session: &mut actix_ws::Session, error: &str) {
    let payload = json!(GameEventWsError {
        message_type: String::from("error"),
        error: String::from(error),
    })
    .to_string();
    let _ = ws_session.text(payload).await;
}

#[utoipa::path(
    get,
    path = "/api/game/session/{id}/events",
    params(
        ("id" = Uuid, Path, example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    responses(
        (status = 200, description = "Events for session", body = Vec<GameEvent>),
        (status = 403, description = "Session belongs to another user"),
        (status = 404, description = "Session not found", body = Object),
        (status = 500, description = "Internal error", body = Object),
    ),
    tag = "games",
    security(("Authorization" = []))
)]
pub async fn get_game_events(
    id: web::Path<Uuid>,
    ext_data: web::ReqData<MiddlewareData>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let session_id = id.into_inner();
    let session = match game_repository::get_session(&state.sb_client, session_id).await {
        Ok(v) => v,
        Err(e) => return e.to_response(),
    };
    if session.user_id != ext_data.user_id {
        return HttpResponse::Forbidden().finish();
    }
    match game_repository::get_events_by_session(&state.sb_client, session_id).await {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => e.to_response(),
    }
}

async fn persist_game_event(
    state: &web::Data<AppState>,
    session_id: Uuid,
    user_id: Uuid,
    event: &CreateGameEventReq,
) -> std::result::Result<GameEventAck, crate::errors::custom_errors::RepoError> {
    let event_id = game_repository::insert_event(
        &state.sb_client,
        session_id,
        user_id,
        event.round,
        event.event_value,
        &event.client_ts,
    )
    .await?;

    Ok(GameEventAck {
        id: event_id,
        round: event.round,
    })
}
