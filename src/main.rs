use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello Yew, My name is 0xLeung, and I'm a React/Go/Rust/Solidity developer" }</h1>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

