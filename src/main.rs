use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <div class="header">
                <h1>{"Websocket Playground"}</h1>
            </div>
            <div>
                <form>
                    <input type="text" id="text" />
                </form>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}