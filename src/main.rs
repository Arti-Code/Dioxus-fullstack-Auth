#![allow(non_snake_case)]

use dioxus::prelude::*;

#[cfg(feature="server")]
use backend::axum_server::launch_server;

mod backend;
mod components;
use components::{register::Register, login::Login, user::User};

fn main() {
    #[cfg(feature = "web")]
    LaunchBuilder::web().launch(App);

    #[cfg(feature="server")]
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        launch_server(App).await;
    });
}

#[component]
fn App() -> Element {
    rsx!(
        document::Stylesheet { href: asset!("assets/main.css") }
        document::Stylesheet { href: asset!("assets/tailwind.css") }
        Router::<Route> {}
    )
}

#[component]
fn Home() -> Element {
    rsx!(
        div { class: "text-sky-500", "Home" }
    )
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/register")]
    Register {},
    #[route("/login")]
    Login{},
    #[route("/user")]
    User {}
}