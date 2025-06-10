use crate::{
    services::attendance_service::{AttendanceService, AttendanceServiceError},
    utils::{http_utils::handle_internal_error, locale_utils::get_lang},
};
use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use futures_util::future::LocalBoxFuture;
use log::{info, warn};
use serde_json::{Value, json};
use shared::{
    types::{
        responses::api_response::{ApiResponse, ErrorDetails},
        ws_types::{AttendanceWsMessage, AttendanceWsResponse},
    },
    utils::locale_utils::{Lang, Messages, Namespace},
};
use std::sync::Arc;

pub struct AttendanceWsSession {
    service: Arc<AttendanceService>,
    lang: Lang,
}

impl Actor for AttendanceWsSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for AttendanceWsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match &msg {
            Ok(ws::Message::Text(text)) => info!("Received WS text: {}", text),
            Ok(other) => info!("Received WS message: {:?}", other),
            Err(e) => warn!("WS protocol error: {:?}", e),
        }

        if let Ok(ws::Message::Text(text)) = msg {
            let service = self.service.clone();
            let addr = ctx.address();
            let messages = Messages::new(self.lang.clone());

            match serde_json::from_str::<AttendanceWsMessage>(&text) {
                Ok(attendance_msg) => {
                    let fut: LocalBoxFuture<'static, String> = Box::pin(async move {
                        let response: ApiResponse<Value> = match attendance_msg {
                            AttendanceWsMessage::Create(payload) => {
                                match service.create_attendance(payload).await {
                                    Ok(data) => ApiResponse::success(
                                        messages
                                            .get_message(Namespace::Attendance, "create.success"),
                                        Some(json!(data)),
                                    ),
                                    Err(e) => ApiResponse::error(
                                        e.to_message(&messages),
                                        Some(ErrorDetails {
                                            details: Some(json!(e.to_string())),
                                        }),
                                    ),
                                }
                            }

                            AttendanceWsMessage::Read(id) => {
                                match service.get_attendance_by_id(&id).await {
                                    Ok(Some(data)) => ApiResponse::success(
                                        messages
                                            .get_message(Namespace::Attendance, "fetch.success"),
                                        Some(json!(data)),
                                    ),
                                    Ok(None) => ApiResponse::error(
                                        AttendanceServiceError::NotFound.to_message(&messages),
                                        None,
                                    ),
                                    Err(e) => ApiResponse::error(
                                        e.to_message(&messages),
                                        Some(ErrorDetails {
                                            details: Some(json!(e.to_string())),
                                        }),
                                    ),
                                }
                            }

                            AttendanceWsMessage::ReadAll => {
                                match service.get_all_attendances().await {
                                    Ok(list) => ApiResponse::success(
                                        messages.get_message(
                                            Namespace::Attendance,
                                            "fetch.all_success",
                                        ),
                                        Some(json!(list)),
                                    ),
                                    Err(e) => ApiResponse::error(
                                        e.to_message(&messages),
                                        Some(ErrorDetails {
                                            details: Some(json!(e.to_string())),
                                        }),
                                    ),
                                }
                            }

                            AttendanceWsMessage::Update { id, payload } => {
                                match service.update_attendance(&id, payload).await {
                                    Ok(data) => ApiResponse::success(
                                        messages
                                            .get_message(Namespace::Attendance, "update.success"),
                                        Some(json!(data)),
                                    ),
                                    Err(e) => ApiResponse::error(
                                        e.to_message(&messages),
                                        Some(ErrorDetails {
                                            details: Some(json!(e.to_string())),
                                        }),
                                    ),
                                }
                            }

                            AttendanceWsMessage::Delete(id) => {
                                match service.delete_attendance(&id).await {
                                    Ok(_) => ApiResponse::success(
                                        messages
                                            .get_message(Namespace::Attendance, "delete.success"),
                                        Some(json!("deleted")),
                                    ),
                                    Err(e) => ApiResponse::error(
                                        e.to_message(&messages),
                                        Some(ErrorDetails {
                                            details: Some(json!(e.to_string())),
                                        }),
                                    ),
                                }
                            }
                        };

                        serde_json::to_string(&AttendanceWsResponse { response }).unwrap_or_else(
                            |e| {
                                json!(AttendanceWsResponse {
                                    response: ApiResponse::error(
                                        "Serialization error",
                                        Some(ErrorDetails {
                                            details: Some(json!(e.to_string())),
                                        }),
                                    )
                                })
                                .to_string()
                            },
                        )
                    });

                    actix::spawn(async move {
                        let response = fut.await;
                        if addr.connected() {
                            addr.do_send(SendText(response));
                        }
                    });
                }

                Err(e) => {
                    let err = ApiResponse::error(
                        messages.get_message(Namespace::Common, "json.invalid"),
                        Some(ErrorDetails {
                            details: Some(json!(e.to_string())),
                        }),
                    );
                    ctx.text(
                        serde_json::to_string(&AttendanceWsResponse { response: err }).unwrap(),
                    );
                }
            }
        } else if let Ok(ws::Message::Close(_)) = msg {
            ctx.stop();
        } else if let Ok(ws::Message::Ping(msg)) = msg {
            ctx.pong(&msg);
        } else if let Err(e) = msg {
            warn!("WebSocket error: {:?}", e);
            ctx.stop();
        }
    }
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

    match ws::start(session, &req, stream) {
        Ok(resp) => resp,
        Err(e) => handle_internal_error(format!("WebSocket start failed: {}", e)),
    }
}
