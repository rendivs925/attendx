use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use serde_json::{Value, json};
use shared::prelude::*;
use shared::{
    types::{
        responses::api_response::{ApiResponse, ErrorDetails},
        ws_types::{AttendanceWsMessage, AttendanceWsResponse},
    },
    utils::locale_utils::{Lang, Namespace},
};
use std::sync::Arc;

use crate::{
    services::attendance_service::{AttendanceService, AttendanceServiceError},
    utils::{http_utils::handle_internal_error, locale_utils::get_lang},
};

pub struct AttendanceWsSession {
    service: Arc<AttendanceService>,
    lang: Lang,
}

impl Actor for AttendanceWsSession {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct SendText(pub String);

impl Handler<SendText> for AttendanceWsSession {
    type Result = ();

    fn handle(&mut self, msg: SendText, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AttendanceWsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let messages = Arc::new(shared::utils::locale_utils::Messages::new(
            self.lang.clone(),
        ));

        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<AttendanceWsMessage>(&text) {
                    Ok(attendance_msg) => {
                        let service = self.service.clone();
                        let addr = ctx.address();

                        actix::spawn(handle_attendance_msg(
                            service,
                            messages,
                            attendance_msg,
                            addr,
                        ));
                    }
                    Err(e) => {
                        let err_response = ApiResponse::error(
                            messages.get_message(Namespace::Common, "json.invalid"),
                            Some(ErrorDetails {
                                details: Some(json!(e.to_string())),
                            }),
                        );
                        ctx.text(
                            serde_json::to_string(&AttendanceWsResponse {
                                response: err_response,
                            })
                            .unwrap(),
                        );
                    }
                }
            }
            Ok(ws::Message::Close(_)) => ctx.stop(),
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Err(_) => ctx.stop(),
            _ => {}
        }
    }
}

async fn handle_attendance_msg(
    service: Arc<AttendanceService>,
    messages: Arc<dyn MessageLookup + Send + Sync>,
    msg: AttendanceWsMessage,
    addr: actix::Addr<AttendanceWsSession>,
) {
    let response: ApiResponse<Value> = match msg {
        AttendanceWsMessage::Create(payload) => match service.create_attendance(payload).await {
            Ok(data) => ApiResponse::success(
                messages.get_message(Namespace::Attendance, "create.success"),
                Some(json!(data)),
            ),
            Err(e) => error_response(messages.as_ref(), e),
        },

        AttendanceWsMessage::Read(id) => match service.get_attendance_by_id(&id).await {
            Ok(Some(data)) => ApiResponse::success(
                messages.get_message(Namespace::Attendance, "fetch.success"),
                Some(json!(data)),
            ),
            Ok(None) => ApiResponse::error(
                AttendanceServiceError::NotFound.to_message(messages.as_ref()),
                None,
            ),
            Err(e) => error_response(messages.as_ref(), e),
        },

        AttendanceWsMessage::ReadAll => match service.get_all_attendances().await {
            Ok(list) => ApiResponse::success(
                messages.get_message(Namespace::Attendance, "fetch.all_success"),
                Some(json!(list)),
            ),
            Err(e) => error_response(messages.as_ref(), e),
        },

        AttendanceWsMessage::Update { id, payload } => {
            match service.update_attendance(&id, payload).await {
                Ok(data) => ApiResponse::success(
                    messages.get_message(Namespace::Attendance, "update.success"),
                    Some(json!(data)),
                ),
                Err(e) => error_response(messages.as_ref(), e),
            }
        }

        AttendanceWsMessage::Delete(id) => match service.delete_attendance(&id).await {
            Ok(_) => ApiResponse::success(
                messages.get_message(Namespace::Attendance, "delete.success"),
                Some(json!("deleted")),
            ),
            Err(e) => error_response(messages.as_ref(), e),
        },
    };

    let text = serde_json::to_string(&AttendanceWsResponse { response }).unwrap_or_else(|e| {
        json!(AttendanceWsResponse {
            response: ApiResponse::error(
                "Serialization error",
                Some(ErrorDetails {
                    details: Some(json!(e.to_string())),
                }),
            )
        })
        .to_string()
    });

    if addr.connected() {
        addr.do_send(SendText(text));
    }
}

fn error_response(
    messages: &dyn MessageLookup,
    error: AttendanceServiceError,
) -> ApiResponse<Value> {
    ApiResponse::error(
        error.to_message(messages),
        Some(ErrorDetails {
            details: Some(json!(error.to_string())),
        }),
    )
}

pub async fn attendance_ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    service: web::Data<Arc<AttendanceService>>,
) -> HttpResponse {
    let lang = get_lang(&req);
    let session = AttendanceWsSession {
        service: service.get_ref().clone(),
        lang,
    };

    ws::start(session, &req, stream)
        .unwrap_or_else(|e| handle_internal_error(format!("WebSocket start failed: {}", e)))
}
