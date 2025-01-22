mod components;

use crate::components::input::{MessageInput, UrlInput};
use crate::components::output::{OutputDetail, OutputSummary};
use futures_util::{AsyncWriteExt, SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::SliceIndex;
use web_sys::{console, window};
use yew::platform::spawn_local;
use yew::prelude::*;

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
    let ws_writer = Rc::new(RefCell::new(None));

    let event_detail_show = match *event_to_show {
        None => "".to_string(),
        Some(index) => match events.get(index) {
            None => "".to_string(),
            Some(event) => event.message.clone(),
        },
    };

    let connect_click : Callback<String> = {
        let is_connected = is_connected.clone();
        let events = events.clone();
        let ws_writer = ws_writer.clone();
        Callback::from(move |url: String| {
            is_connected.set(true);
            let socket = match WebSocket::open(&*url) {
                Ok(ws) => ws,
                Err(_e) => {
                    is_connected.set(false);
                    if let Some(window) = window() {
                        window.alert_with_message("Failed to connect to websocket server.").unwrap();
                    }
                    return;
                }
            };

            let (writer, mut read) = socket.split();
            ws_writer.borrow_mut().replace(writer);

            spawn_local( {
                let events = events.clone();
                async move {
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            console::log_1(&text.clone().into());
                            events.set({
                                let mut new_events = (*events).clone();
                                new_events.push(Event {
                                    message: text,
                                    kind: EventKind::System,
                                });
                                new_events
                            });
                        }
                        Ok(Message::Bytes(_)) => {
                            // Handle binary messages if needed
                        }
                        Err(e) => {
                            console::error_1(&format!("WebSocket error: {:?}", e).into());
                            break;
                        }
                    };

                }
            }});
        })
    };

    let disconnect_click: Callback<()> = {
        let ws_writer = ws_writer.clone();
        let is_connected = is_connected.clone();
        Callback::from(move |_| {
            let is_connected = is_connected.clone();
            let ws_writer = ws_writer.clone();
            spawn_local(async move {
                if *is_connected {
                    if let Some(mut writer) = ws_writer.borrow_mut().take() {
                        // Close the con
                        if let Err(res) = writer.close().await {
                            console::log_1(&"unable to disconnect".into());
                        }
                    }
                    is_connected.set(false);
                }
            });
        })
    };

    let send_click : Callback<String> = {
        let ws_writer = ws_writer.clone();
        Callback::from(move |payload: String| {
            let _payload = payload.as_str();
    })};

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
                    <UrlInput is_connected={*is_connected.clone()} connect_click={connect_click} disconnect_click={disconnect_click}/>
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