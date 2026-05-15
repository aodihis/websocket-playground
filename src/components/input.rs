use web_sys::{EventTarget, FocusEvent, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, KeyboardEvent};
use web_sys::wasm_bindgen::JsCast;
use yew::{function_component, html, use_state, Callback, Event as YewEvent, Html, NodeRef, Properties, UseStateHandle};

#[derive(Clone, PartialEq)]
pub enum InputMode {
    Raw,
    Json,
}

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

/// Convert a JavaScript selectionStart (UTF-16 unit count) to a Rust byte offset.
/// For ASCII-heavy JSON content this is equivalent to char count → byte index.
fn char_to_byte(s: &str, char_pos: usize) -> usize {
    s.char_indices()
        .nth(char_pos)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

#[function_component]
pub fn UrlInput(props: &UrlInputProps) -> Html {
    let input_ref = NodeRef::default();
    let is_connected = props.is_connected;
    let on_click = props.connect_click.clone();
    let disconnect_click = props.disconnect_click.clone();

    let onconnect = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            if let Some(el) = input_ref.cast::<HtmlInputElement>() {
                let url = el.value();
                if url.starts_with("ws://") || url.starts_with("wss://") {
                    on_click.emit(url);
                }
            }
        })
    };

    let disconnected = Callback::from(move |_| {
        disconnect_click.emit(());
    });

    html! {
        <>
            <div class="url-input-container">
                <input type="text" id="url" placeholder="ws://websocket.url" ref={input_ref}/>
                {
                    if !is_connected {
                        html! {<button class="button primary" type="submit" onclick={onconnect}>{"Connect"}</button>}
                    } else {
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
    let input_mode: UseStateHandle<InputMode> = use_state(|| InputMode::Raw);
    let json_error: UseStateHandle<Option<String>> = use_state(|| None);

    let on_mode_change = {
        let input_mode = input_mode.clone();
        let json_error = json_error.clone();
        Callback::from(move |e: YewEvent| {
            let target: Option<EventTarget> = e.target();
            let select = target.and_then(|t| t.dyn_into::<HtmlSelectElement>().ok());
            if let Some(select) = select {
                match select.value().as_str() {
                    "json" => input_mode.set(InputMode::Json),
                    _ => input_mode.set(InputMode::Raw),
                }
                json_error.set(None);
            }
        })
    };

    // Auto-format valid JSON silently when focus leaves the textarea
    let on_blur = {
        let msg_ref = msg_ref.clone();
        let input_mode = input_mode.clone();
        Callback::from(move |_: FocusEvent| {
            if *input_mode != InputMode::Json {
                return;
            }
            if let Some(el) = msg_ref.cast::<HtmlTextAreaElement>() {
                let val = el.value();
                if val.trim().is_empty() {
                    return;
                }
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&val) {
                    if let Ok(formatted) = serde_json::to_string_pretty(&parsed) {
                        el.set_value(&formatted);
                    }
                }
            }
        })
    };

    // Smart keyboard handling for JSON mode:
    //   Enter → preserves indentation, increases it after { or [
    //           and adds a closing line when cursor is before } or ]
    //   Tab   → inserts 2 spaces instead of moving focus
    let on_keydown = {
        let msg_ref = msg_ref.clone();
        let input_mode = input_mode.clone();
        Callback::from(move |e: KeyboardEvent| {
            if *input_mode != InputMode::Json {
                return;
            }
            let el = match msg_ref.cast::<HtmlTextAreaElement>() {
                Some(el) => el,
                None => return,
            };

            match e.key().as_str() {
                "Enter" => {
                    e.prevent_default();

                    let val = el.value();
                    let cursor = el.selection_start().ok().flatten().unwrap_or(0) as usize;
                    let byte_cursor = char_to_byte(&val, cursor);

                    let before = &val[..byte_cursor];
                    let after = &val[byte_cursor..];

                    // Indentation of the current line
                    let line_start = before.rfind('\n').map(|p| p + 1).unwrap_or(0);
                    let current_line = &before[line_start..];
                    let indent: String = current_line.chars().take_while(|c| *c == ' ').collect();

                    // Increase indent if the last non-whitespace char opens a block
                    let last_char = before.trim_end().chars().last();
                    let new_indent = if matches!(last_char, Some('{') | Some('[')) {
                        format!("{}  ", indent)
                    } else {
                        indent.clone()
                    };

                    // If the very next char closes a block, insert a blank inner line
                    // so the cursor lands between opening and closing at the right level
                    let next_char = after.chars().next();
                    let insertion = if matches!(next_char, Some('}') | Some(']')) {
                        format!("\n{}\n{}", new_indent, indent)
                    } else {
                        format!("\n{}", new_indent)
                    };

                    let new_val = format!("{}{}{}", before, insertion, after);
                    el.set_value(&new_val);

                    let new_cursor = (cursor + 1 + new_indent.chars().count()) as u32;
                    let _ = el.set_selection_start(Some(new_cursor));
                    let _ = el.set_selection_end(Some(new_cursor));
                }
                "}" | "]" => {
                    let val = el.value();
                    let cursor = el.selection_start().ok().flatten().unwrap_or(0) as usize;
                    let byte_cursor = char_to_byte(&val, cursor);

                    let before = &val[..byte_cursor];
                    let line_start = before.rfind('\n').map(|p| p + 1).unwrap_or(0);
                    let current_line = &before[line_start..];

                    // Only dedent when the current line is pure whitespace with room to remove
                    if current_line.chars().all(|c| c == ' ') && current_line.len() >= 2 {
                        e.prevent_default();

                        let after = &val[byte_cursor..];
                        let new_indent_len = current_line.len() - 2;
                        let new_indent = " ".repeat(new_indent_len);
                        let closing = e.key();

                        let new_val = format!("{}{}{}{}", &val[..line_start], new_indent, closing, after);
                        el.set_value(&new_val);

                        let new_cursor = (val[..line_start].chars().count() + new_indent_len + 1) as u32;
                        let _ = el.set_selection_start(Some(new_cursor));
                        let _ = el.set_selection_end(Some(new_cursor));
                    }
                }
                "Tab" => {
                    e.prevent_default();

                    let val = el.value();
                    let cursor = el.selection_start().ok().flatten().unwrap_or(0) as usize;
                    let byte_cursor = char_to_byte(&val, cursor);

                    let before = &val[..byte_cursor];
                    let after = &val[byte_cursor..];
                    let new_val = format!("{}  {}", before, after);
                    el.set_value(&new_val);

                    let new_cursor = (cursor + 2) as u32;
                    let _ = el.set_selection_start(Some(new_cursor));
                    let _ = el.set_selection_end(Some(new_cursor));
                }
                _ => {}
            }
        })
    };

    let onclick = {
        let send_click = send_click.clone();
        let msg_ref = msg_ref.clone();
        let input_mode = input_mode.clone();
        let json_error = json_error.clone();
        Callback::from(move |_| {
            if let Some(el) = msg_ref.cast::<HtmlTextAreaElement>() {
                let msg = el.value();
                if msg.is_empty() {
                    return;
                }
                if *input_mode == InputMode::Json {
                    match serde_json::from_str::<serde_json::Value>(&msg) {
                        Ok(parsed) => {
                            json_error.set(None);
                            let formatted = serde_json::to_string_pretty(&parsed).unwrap_or(msg);
                            send_click.emit(formatted);
                        }
                        Err(e) => {
                            json_error.set(Some(format!("Invalid JSON: {e}")));
                        }
                    }
                } else {
                    json_error.set(None);
                    send_click.emit(msg);
                }
            }
        })
    };

    html! {
        <div class="message">
            <div class="message-toolbar">
                <label for="message">{"Message"}</label>
                <div class="message-toolbar-controls">
                    <select name="input_mode" id="input_mode" onchange={on_mode_change}>
                        <option value="raw" selected={*input_mode == InputMode::Raw}>{"Raw"}</option>
                        <option value="json" selected={*input_mode == InputMode::Json}>{"JSON"}</option>
                    </select>

                    {
                        if *is_connected {
                            html! {<button class="button danger" onclick={onclick}>{"Send"}</button>}
                        } else {
                            html! {}
                        }
                    }
                </div>
            </div>
            <textarea
                id="message"
                ref={msg_ref}
                placeholder="Type your message here..."
                onkeydown={on_keydown}
                onblur={on_blur}
            />
            {
                if let Some(err) = (*json_error).clone() {
                    html! { <span class="json-error">{ err }</span> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
