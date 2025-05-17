#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod request;
mod user;

use components::{login::Login, navbar::Navbar, register::Register, user::User};
//use reqwest::{Client, ClientBuilder};
use reqwest::*;

//static CLIENT: Client = reqwest::ClientBuilder::new().cookie_store(true).build().unwrap();

fn main() {
    #[cfg(feature = "web")]
    LaunchBuilder::web().launch(App);
}

#[component]
fn App() -> Element {
    /* let client = use_context_provider(|| Signal::new(
        Client::builder().cookie_store(true).build().unwrap()
    )); */
    rsx!(
        document::Stylesheet { href: asset!("assets/main.css") }
        document::Stylesheet { href: asset!("assets/tailwind.css") }
        Navbar {}
        Router::<Route> {}
        //document::Script { src: asset!("node_modules/flyonui/flyonui.js") }
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
    //Home {},
    Register {},
    //#[route("/register")]
    #[route("/login")]
    Login{},
    #[route("/user")]
    User {}
}