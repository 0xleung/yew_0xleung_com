use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello Yew, My name is 0xLeung" }</h1>
            <p>, and I'm a React/Go/Rust/Solidity developer </p>
        </p>
    }
}

fn main() {
    yew::start_app::<App>();
}

