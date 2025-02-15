mod components;

use std::rc::Rc;
use crate::components::input::{MessageInput, UrlInput};
use crate::components::output::{OutputDetail, OutputSummary};
use futures_util::{SinkExt, StreamExt};
use futures_util::future::{AbortHandle, Abortable};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use web_sys::{console, window};
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum EventKind {
    All,
    Send,
    Receive,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Event {
    pub message: String,
    pub kind: EventKind,
}

#[derive(Clone, Debug)]
struct EventsState {
    pub events: Vec<Event>,
}

impl Reducible for EventsState {
    type Action = Event;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut new_events = self.events.clone();
        new_events.push(action);
        Rc::new(Self { events: new_events })
    }
}

#[function_component]
fn App() -> Html {

    let is_connected: UseStateHandle<bool> = use_state(|| false);
    let events_state: UseReducerHandle<EventsState> = use_reducer(|| EventsState { events: vec![] });
    let event_to_show: UseStateHandle<Option<usize>> = use_state(|| Option::<usize>::None.into());
    // let ws_writer:Rc<RefCell<Option<SplitSink<WebSocket,Message>>>> = Rc::new(RefCell::new(None));
    let ws_writer = use_mut_ref(|| None);
    let abort_handle = use_mut_ref(|| None);


    let events_update = {
        let events = events_state.clone();
        Callback::from(move |item: Event|{
            events.dispatch(item);
        })
    };
    let event_detail_show = match *event_to_show {
        None => "".to_string(),
        Some(index) => match events_state.events.get(index) {
            None => "".to_string(),
            Some(event) => event.message.clone(),
        },
    };

    let connect_click: Callback<String> = {
        let is_connected = is_connected.clone();
        let events_update = events_update.clone();
        let ws_writer = ws_writer.clone();
        let abort_handle = abort_handle.clone();

        Callback::from(move |url: String| {
            is_connected.set(true);
            let socket = match WebSocket::open(&*url) {
                Ok(ws) => ws,
                Err(_e) => {
                    is_connected.set(false);
                    if let Some(window) = window() {
                        window
                            .alert_with_message("Failed to connect to websocket server.")
                            .unwrap();
                    }
                    return;
                }
            };

            let (writer, mut read) = socket.split();
            *ws_writer.borrow_mut() = Some(writer);

            let events_update = events_update.clone();
            let is_connected = is_connected.clone();
            let (abort_handle_inner, abort_registration) = AbortHandle::new_pair();
            *abort_handle.borrow_mut() = Some(abort_handle_inner.clone());


            let task = {
                let ws_writer = ws_writer.clone();
                async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                console::log_1(&text.clone().into());

                                events_update.emit(Event {
                                    message: text,
                                    kind: EventKind::Receive,
                                });
                                // events.set({
                                //     let mut new_events = (*events).clone();
                                //
                                //     new_events
                                // });
                            }
                            Ok(Message::Bytes(_)) => {
                                console::log_1(&"Hand".into());
                            }
                            Err(e) => {
                                console::error_1(&format!("WebSocket error: {:?}", e).into());
                                is_connected.set(false);
                                *ws_writer.borrow_mut() = None;
                                if let Some(window) = window() {
                                    window
                                        .alert_with_message("Connection error!")
                                        .unwrap();
                                }
                                break;
                            }
                        };
                    }
                }
            };

            let abort_task = Abortable::new(task, abort_registration);
            spawn_local(async move {
                let _ = abort_task.await;
            });
        })
    };

    let disconnect_click: Callback<()> = {
        let ws_writer = ws_writer.clone();
        let is_connected = is_connected.clone();
        let abort_handle = abort_handle.clone();
        Callback::from(move |_| {
            let is_connected = is_connected.clone();
            let ws_writer = ws_writer.clone();
            let abort_handle = abort_handle.clone();
            spawn_local(async move {
                let mut binding = ws_writer.borrow_mut();
                {
                    let writer = binding.as_mut().unwrap();
                    writer.close().await.unwrap();
                }
                *binding = None;
                if let Some(handle) = abort_handle.borrow_mut().take() {
                    handle.abort(); // Abort the WebSocket listener
                }

                // *ws_writer.borrow_mut() = None;
                is_connected.set(false);
            });
        })
    };

    let send_click: Callback<String> = {
        let ws_writer = ws_writer.clone();
        let events_update = events_update.clone();
        Callback::from(move |payload: String| {
            let ws_writer = ws_writer.clone();
            let events_update = events_update.clone();
            console::log_1(&payload.clone().into());
            spawn_local({
                async move {
                    if let Some(writer) = ws_writer.borrow_mut().as_mut() {
                        if writer.send(Message::Text(payload.clone())).await.is_ok() {
                            events_update.emit(Event {
                                message: payload,
                                kind: EventKind::Send,
                            });
                            // events.set({
                            //     let mut new_events = (*events).clone();
                            //     new_events.push(Event {
                            //         message: payload,
                            //         kind: EventKind::Send,
                            //     });
                            //     new_events
                            // });
                        }
                    }
                }
            })
        })
    };

    let sum_click: Callback<usize> = Callback::from(move |index| {
        event_to_show.set(Some(index));
    });

    let events = events_state.events.clone();
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
                    <OutputSummary events={events} on_click={sum_click}/>
                    <OutputDetail data={event_detail_show} />
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
