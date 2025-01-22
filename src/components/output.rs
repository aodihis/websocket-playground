use crate::{Event, EventKind};
use web_sys::wasm_bindgen::JsCast;
use web_sys::{console, EventTarget, HtmlSelectElement};
use yew::{function_component, html, use_state, Callback, Event as YewEvent, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq, Clone)]
pub struct OutputSummaryProps {
    pub events: Vec<Event>,
    pub on_click: Callback<usize>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct OutputDetailProps {
    pub data: String,
}
#[function_component]
pub fn OutputSummary(props: &OutputSummaryProps) -> Html {
    let OutputSummaryProps {events, on_click} = props;
    let kind: UseStateHandle<EventKind> = use_state(|| EventKind::All);

    let onclick = |index: usize| {
        let on_click = on_click.clone();
        Callback::from(move |_| {
            on_click.emit(index);
        })};

    let onchange = {
        let kind = kind.clone();
        Callback::from( move |e: YewEvent| {
            let target: Option<EventTarget> = e.target();
            let val = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());

            if let Some(val) = val {
                match val.value().as_str() {
                    "received" => {
                        kind.set(EventKind::Receive);
                    },
                    "sent" => {
                        kind.set(EventKind::Send);
                    },
                    _ => {
                        kind.set(EventKind::All);
                    }
                };
                console::log_1(&(val.value()).into());
            }
        })
    };
    html! {
        <>
            <div class="data-sum-header">
                <h4>{"Event Logs:"}</h4>
                <select name="event_type" id="event_type" onchange={onchange}>
                  <option value="all" selected={true}>{"All"}</option>
                  <option value="sent">{"Sent"}</option>
                  <option value="received">{"Received"}</option>
                </select>
            </div>
            <div class="data-sum">
            {
                events.iter().enumerate().filter(|(_, event)| (event.kind == *kind) || (*kind == EventKind::All) )
                .map(|(index, event) : (usize, &Event)| {
                    html! {
                        <div class="data-summary-info" onclick={onclick(index)}>
                            <div>{ &event.message }</div>
                            <div>{ match &event.kind {
                                EventKind::Send => {html!("Send")},
                                EventKind::Receive => {html!("Receive")},
                                _ => {html!()}
                            }}
                            </div>
                        </div>
                    }
            }).collect::<Html>()
            }
            </div>
        </>
    }
}

#[function_component]
pub fn OutputDetail(props: &OutputDetailProps) -> Html {
    let OutputDetailProps { data } = props;

    html! {
        <div class="data-detail">
            {data}
        </div>
    }
}