use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{function_component, html, Callback, Html, NodeRef, Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct UrlInputProps {
     pub is_connected: bool,
     pub connect_click: Callback<String>,
     pub disconnect_click: Callback<()>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MessageInputProps {
     pub is_connected: bool,
     pub send_click: Callback<String>,
}


#[function_component]
pub fn UrlInput(props: &UrlInputProps) -> Html {
    let input_ref = NodeRef::default();
    // let UrlInputProps {is_connected, on_click} = props;
    let is_connected = props.is_connected;
    let on_click = props.connect_click.clone();
    let disconnect_click = props.disconnect_click.clone();
    let onconnect = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            let url = input_ref.cast::<HtmlInputElement>().unwrap().value();
            on_click.emit(url);
        })
    };

    let disconnected = {
        Callback::from(move |_| {
            disconnect_click.emit(());
        })
    };
    html! {
        <>
            <div class="url-input-container">
                <input type="text" id="url" placeholder="ws://websocket.url" ref={input_ref}/>
                {
                    if is_connected != true {
                        html! {<button class="button primary" type="submit" onclick={onconnect}>{"Connect"}</button>}
                    }
                    else {
                        html! {<button class="button danger" type="submit" onclick={disconnected}>{"Disconnect"}</button>}
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
            send_click.emit(msg);
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