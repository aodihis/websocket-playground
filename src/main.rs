mod components;

use std::slice::SliceIndex;
use yew::prelude::*;
use crate::components::input::{MessageInput, UrlInput};
use crate::components::output::{OutputDetail, OutputSummary};

#[derive(Clone, PartialEq)]
pub enum EventKind {
    System,
    Send,
    Receive
}

#[derive(Clone, PartialEq)]
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
    let events:UseStateHandle<Vec<Event>> = use_state(|| dummy_messages);
    let event_to_show: UseStateHandle<Option<usize>> = use_state(|| Option::<usize>::None.into());

    // events.set(dummy_messages);

    let event_detail_show = match *event_to_show {
        None => "".to_string(),
        Some(index) => match events.get(index) {
            None => "".to_string(),
            Some(event) => event.message.clone(),
        },
    };

    let connect_click : Callback<()> = {
        let is_connected = is_connected.clone();
        Callback::from(move |_| {
            is_connected.set(!*is_connected);
        })
    };

    let send_click : Callback<String> = Callback::from(move |payload: String| {
            let _payload = payload.as_str();
    });

    let sum_click: Callback<usize> = Callback::from(move |index| {
        event_to_show.set(Some(index));
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