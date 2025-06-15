use codee::string::FromToStringCodec;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_use::core::ConnectionReadyState as State;
use leptos_use::{UseWebSocketReturn, use_websocket};
use serde_json::json;

#[component]
pub fn WsButton(
    label: &'static str,
    class: &'static str,
    on_click: Callback<MouseEvent>,
    disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            class=format!("btn {} flex-1", class)
            on:click=move |ev| on_click.run(ev)
            disabled=disabled
        >
            {label}
        </button>
    }
}

#[component]
pub fn AttendanceWsTest() -> impl IntoView {
    type Msg = String;

    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        open,
        close,
        ..
    } = use_websocket::<Msg, Msg, FromToStringCodec>("ws://localhost:8000/ws/attendance");

    let is_open = Signal::derive(move || ready_state.get() == State::Open);

    let status_text = move || ready_state.get().to_string();

    let status_class = move || match ready_state.get() {
        State::Open => "text-success",
        State::Connecting => "text-warning",
        State::Closing => "text-error",
        State::Closed => "text-info",
    };

    let send_read_all = Callback::new(move |_ev: MouseEvent| {
        let payload = json!({ "type": "ReadAll" }).to_string();
        send(&payload);
    });

    let open_cb = Callback::new(move |_ev: MouseEvent| open());
    let close_cb = Callback::new(move |_ev: MouseEvent| close());

    view! {
        <div class="p-8 flex flex-col items-center justify-center min-h-screen bg-base-200 text-base-content">
            <h2 class="text-3xl font-bold mb-6">"WebSocket Test"</h2>

            <div class="card w-96 bg-base-100 shadow-xl p-6 space-y-4">
                <p class="text-lg">
                    <span class="font-semibold">"Status: "</span>
                    <span class=status_class>{status_text}</span>
                </p>

                <div class="flex space-x-4">
                    <WsButton
                        label="Send Message"
                        class="btn-primary"
                        on_click=send_read_all
                        disabled=Signal::derive(move || !is_open.get())
                    />
                    <WsButton
                        label="Open Connection"
                        class="btn-success"
                        on_click=open_cb
                        disabled=Signal::derive(move || is_open.get())
                    />
                    <WsButton
                        label="Close Connection"
                        class="btn-error"
                        on_click=close_cb
                        disabled=Signal::derive(move || !is_open.get())
                    />
                </div>

                <p class="text-lg">
                    <span class="font-semibold">"Received Message: "</span>
                    <span class="text-accent">{move || format!("{:?}", message.get())}</span>
                </p>
            </div>
        </div>
    }
}
