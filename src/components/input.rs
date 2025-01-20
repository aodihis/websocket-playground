use yew::{function_component, html, Html};

#[function_component]
pub fn UrlInput() -> Html {
    html! {
        <>
            <div class="url-input-container">
                <input type="text" id="url" placeholder="ws://websocket.url"/>
                <button class="button hidden primary" type="submit">{"Connect"}</button>
                <button class="button danger" type="submit">{"Disconnect"}</button>
            </div>
        </>
    }
}

#[function_component]
pub fn MessageInput() -> Html {
    html! {
        <div class="message">
            <div class="message-header">
                <label for="message">{"Message"}</label>
                <button class="button danger">{"Send"}</button>
            </div>
            <textarea type="text" id="message" />
        </div>
    }
}