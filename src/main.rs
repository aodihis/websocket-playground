use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="header">
                <h1>{"Websocket Playground"}</h1>
            </div>
            <div class="body">
                <div class="input">
                    <div class="url-input-container">
                        <input type="text" id="url" placeholder="ws://websocket.url"/>
                        <button class="button hidden primary" type="submit">{"Connect"}</button>
                        <button class="button danger" type="submit">{"Disconnect"}</button>
                    </div>
                    <div class="message">
                        <div class="message-header">
                            <label for="message">{"Message"}</label>
                            <button class="button danger">{"Send"}</button>
                        </div>
                        <textarea type="text" id="message" />
                    </div>
                </div>
                <div class="output">
                    <div class="data-sum"></div>
                    <div class="data-detail"></div>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}