mod components;

use yew::prelude::*;
use crate::components::input::{MessageInput, UrlInput};
use crate::components::output::{OutputDetail, OutputSummary};

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="header">
                <h1>{"Websocket Playground"}</h1>
            </div>
            <div class="body">
                <div class="input">
                    <UrlInput/>
                    <MessageInput/>
                </div>
                <div class="output">
                    <OutputSummary/>
                    <OutputDetail/>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}