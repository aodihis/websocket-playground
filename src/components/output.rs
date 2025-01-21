use crate::{Event, EventKind};
use yew::{function_component, html, use_state, Callback, Html, Properties};

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
    let kind = use_state(|| EventKind::System);

    let onclick = |index: usize| {
        let on_click = on_click.clone();
        Callback::from(move |_| {
            on_click.emit(index);
        })};

    html! {
        <>
            <div class="data-sum-header">
                <h4>{"Event Logs:"}</h4>
                <select name="event_type" id="event_type">
                  <option value="system" selected={true}>{"System"}</option>
                  <option value="sent">{"Sent"}</option>
                  <option value="received">{"Received"}</option>
                </select>
            </div>
            <div class="data-sum">
            {
                events.iter().enumerate().map(|(index, event)| html! {
                    <div class="data-summary-info" onclick={onclick(index)}>
                        { &event.message }
                    </div>
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