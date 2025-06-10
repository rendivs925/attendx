use js_sys::JsString;
use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use leptos::web_sys::{MessageEvent, WebSocket, console};
use shared::types::ws_types::{AttendanceWsMessage, WsMessage};

#[component]
pub fn AttendanceWsTest() -> impl IntoView {
    let (messages, set_messages) = signal(vec![] as Vec<String>);
    let (ws_ready, set_ws_ready) = signal(false);
    let ws = std::rc::Rc::new(std::cell::RefCell::new(None::<WebSocket>));

    Effect::new({
        let set_messages = set_messages.clone();
        let set_ws_ready = set_ws_ready.clone();
        let ws = ws.clone();

        move |_| {
            console::log_1(&"Attempting to open WebSocket".into());

            let socket = match WebSocket::new("ws://127.0.0.1:8080/ws") {
                Ok(s) => s,
                Err(e) => {
                    console::log_1(&format!("WebSocket creation error: {:?}", e).into());
                    return;
                }
            };

            let onopen_cb = Closure::wrap(Box::new(move || {
                console::log_1(&"WebSocket opened".into());
                set_ws_ready.set(true);
            }) as Box<dyn FnMut()>);
            socket.set_onopen(Some(onopen_cb.as_ref().unchecked_ref()));
            onopen_cb.forget();

            let onmessage_cb = {
                let set_messages = set_messages.clone();
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    console::log_1(&"WebSocket message received".into());

                    if let Ok(txt) = e.data().dyn_into::<JsString>() {
                        let new_msg = txt.as_string().unwrap_or_default();
                        console::log_1(&format!("Parsed message: {}", new_msg).into());
                        set_messages.update(|msgs| msgs.push(new_msg.clone()));
                    } else {
                        console::log_1(&"Failed to cast message to string".into());
                    }
                }) as Box<dyn FnMut(_)>)
            };
            socket.set_onmessage(Some(onmessage_cb.as_ref().unchecked_ref()));
            onmessage_cb.forget();

            let onerror_cb = Closure::wrap(Box::new(move || {
                console::log_1(&"WebSocket error".into());
            }) as Box<dyn FnMut()>);
            socket.set_onerror(Some(onerror_cb.as_ref().unchecked_ref()));
            onerror_cb.forget();

            let onclose_cb = Closure::wrap(Box::new(move || {
                console::log_1(&"WebSocket closed".into());
                set_ws_ready.set(false);
            }) as Box<dyn FnMut()>);
            socket.set_onclose(Some(onclose_cb.as_ref().unchecked_ref()));
            onclose_cb.forget();

            *ws.borrow_mut() = Some(socket);
        }
    });

    let send_read_all = {
        let ws = ws.clone();
        let ws_ready = ws_ready.clone();

        move |_| {
            if !ws_ready.get() {
                console::log_1(&"WebSocket not ready. Cannot send.".into());
                return;
            }

            if let Some(socket) = &*ws.borrow() {
                let msg = WsMessage::Attendance(AttendanceWsMessage::ReadAll);
                match serde_json::to_string(&msg) {
                    Ok(msg_text) => {
                        console::log_1(&format!("Sending message: {}", msg_text).into());
                        if let Err(e) = socket.send_with_str(&msg_text) {
                            console::log_1(&format!("Send error: {:?}", e).into());
                        }
                    }
                    Err(e) => {
                        console::log_1(&format!("Serialization error: {:?}", e).into());
                    }
                }
            } else {
                console::log_1(&"WebSocket is None.".into());
            }
        }
    };

    view! {
        <div>
            <button on:click=send_read_all class="p-2 bg-blue-500 text-white rounded">
                "Send ReadAll"
            </button>
            <ul class="mt-4 space-y-2">
                {move || {
                    messages
                        .get()
                        .iter()
                        .map(|msg| view! { <li class="bg-gray-100 p-2 rounded">{msg.clone()}</li> })
                        .collect::<Vec<_>>()
                }}
            </ul>
        </div>
    }
}
