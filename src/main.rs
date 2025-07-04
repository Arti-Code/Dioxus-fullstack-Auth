#![allow(non_snake_case)]

use dioxus::prelude::*;
#[cfg(feature="server")]
use backend::axum_server::launch_server;

mod backend;
mod components;
use components::{
    register::Register, 
    login::Login, 
    user::User,
    profile::Profile,
    home::Home,
    robots::Robots,
    robots2::Robots2,
    add_robot::AddRobot,
};
use crate::components::navbar::Navbar;

static ROBOT128: Asset = asset!("/assets/robot128.png");
static ONLINE128: Asset = asset!("/assets/online.png");
static OFFLINE128: Asset = asset!("/assets/offline.png");

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
    use_context_provider(|| Signal::new(UserSession::new()));
    rsx!(
        document::Stylesheet { href: asset!("assets/main.css") }
        document::Stylesheet { href: asset!("assets/tailwind.css") }
        body {
            class: "bg-sky-950", 
            Router::<Route> {}
        }
    )
}


#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/register")]
    Register {},
    #[route("/login")]
    Login{},
    #[route("/user")]
    User {},
    #[route("/profile")]
    Profile {},
    #[route("/robots")]
    Robots {},
    #[route("/robots2")]
    Robots2 {},
    #[route("/robots/add")]
    AddRobot {},
}

#[derive(PartialEq, Clone, Debug/* , Serialize, Deserialize */)]
pub struct UserSession {
    is_login: bool,
}

impl Default for UserSession {
    fn default() -> Self {
        Self {
            is_login: false,
        }
    }
}

impl UserSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn login(&mut self) {
        self.is_login = true;
    }

    pub fn is_logged(&self) -> bool {
        self.is_login
    }

    pub fn logout(&mut self) {
        self.is_login = false;
    }
}