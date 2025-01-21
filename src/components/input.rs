use web_sys::HtmlTextAreaElement;
use yew::{function_component, html, Callback, Html, NodeRef, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct UrlInputProps {
     pub(crate) is_connected: bool,
     pub(crate) connect_click: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MessageInputProps {
     pub is_connected: bool,
     pub send_click: Callback<String>,
}


#[function_component]
pub fn UrlInput(props: &UrlInputProps) -> Html {
    // let UrlInputProps {is_connected, on_click} = props;
    let is_connected = props.is_connected;
    let on_click = props.connect_click.clone();
    let onclick = Callback::from(move |_| {
        on_click.emit(());
    });
    html! {
        <>
            <div class="url-input-container">
                <input type="text" id="url" placeholder="ws://websocket.url"/>
                {
                    if is_connected != true {
                        html! {<button class="button primary" type="submit" onclick={onclick}>{"Connect"}</button>}
                    }
                    else {
                        html! {<button class="button danger" type="submit" onclick={onclick}>{"Disconnect"}</button>}
                    }
                }
            </div>
        </>
    }
}

#[function_component]
pub fn MessageInput(props: &MessageInputProps) -> Html {
    let MessageInputProps { is_connected, send_click } = props;

    let msg_ref = NodeRef::default();
    let onclick = {
        let send_click = send_click.clone();
        let msg_ref = msg_ref.clone();
        Callback::from(move |_| {
            let msg = msg_ref.cast::<HtmlTextAreaElement>().unwrap().value();
            send_click.emit(msg.clone());
        })
    };
    html! {
        <div class="message">
            <div class="message-header">
                <label for="message">{"Message"}</label>
                {
                    if *is_connected {
                        html! {<button class="button danger" onclick={onclick}>{"Send"}</button>}
                    } else {
                        html! {}
                     }
                }
            </div>
            <textarea type="text" id="message" ref={msg_ref}/>
        </div>
    }
}