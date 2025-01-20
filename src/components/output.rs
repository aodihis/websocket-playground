use yew::{function_component, html, Html};

#[function_component]
pub fn OutputSummary() -> Html {
    let data = vec!["Hello", "world", "{josn sexamdpaddh fohsdfhsdlfhds;kS"];
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
                data.iter().map(|d| html! {
                <div class="data-summary-info">
                    {*d}
                </div>
                }).collect::<Html>()
            }
            </div>
        </>
    }
}

#[function_component]
pub fn OutputDetail() -> Html {
    html! {
        <div class="data-detail"></div>
    }
}