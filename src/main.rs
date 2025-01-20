mod components;

use yew::prelude::*;
use crate::components::input::{MessageInput, UrlInput};
use crate::components::output::{OutputDetail, OutputSummary};

#[derive(Clone, PartialEq)]
pub enum EventKind {
    System,
    Send,
    Receive
}

#[derive(Clone)]
pub struct Event {
    pub message: String,
    pub kind: EventKind,
}
#[function_component]
fn App() -> Html {
    let simple_data = vec![
        ("name", "John Doe"),
        ("age", "30"),
        ("is_active", "true"),
    ];

    let dummy_messages = vec![Event{
        message: "Hello".to_string(),
        kind: EventKind::System,
    },  Event{
        message: "fdasfda".to_string(),
        kind: EventKind::System,
        }, Event{
        message: (&*serde_json::to_string_pretty(&simple_data).unwrap()).parse().unwrap(),
        kind: EventKind::System,
    }
    ];

    let is_connected: UseStateHandle<bool> = use_state(|| false);
    let events:UseStateHandle<Vec<Event>> = use_state(Vec::<Event>::new);
    let event_to_show: UseStateHandle<usize> = use_state(|| 0);

    events.set(dummy_messages);

    let event_detail_show = (*events)[*event_to_show].message.clone();

    let connect_click : Callback<()> = {
        let is_connected = is_connected.clone();
        Callback::from(move |_| {
            is_connected.set(!*is_connected);
        })
    };

    let send_click : Callback<(String)> = Callback::from(move |payload: String| {
            let payload = payload.as_str();
    });

    let sum_click: Callback<usize> = Callback::from(move |index| {
        event_to_show.set(index);
    });

    html! {
        <>
            <div class="header">
                <h1>{"Websocket Playground"}</h1>
            </div>
            <div class="body">
                <div class="input">
                    <UrlInput is_connected={*is_connected.clone()} connect_click={connect_click}/>
                    <MessageInput is_connected={*is_connected.clone()} send_click={send_click}/>
                </div>
                <div class="output">
                    <OutputSummary events={(*events).clone()} on_click={sum_click}/>
                    <OutputDetail data={event_detail_show} />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}