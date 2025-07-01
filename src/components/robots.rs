use dioxus::prelude::*;
use crate::{backend::server_functions::{get_user, register}, Route};


#[component]
pub fn Robots() -> Element {
    let navigator = use_navigator();
    let mut active_user = use_signal(|| (0, String::new()));
    let _ = use_resource(move || async move {
        match get_user().await {
            Ok(data) => {
                active_user.set(data);
            },
            Err(e) => {},
        };
    });
    rsx!(
      div { class: "flex",  
        div {
          class: "grid grid-cols-1 gap-4 w-3/4 bg-slate-100 p-5 mx-auto",
          div { 
            class: "text-center bg-slate-100 border-solid border-1 border-blue-200  text-xl font-bold",
            "ID {active_user.read().0}   {active_user.read().1.to_uppercase()}"
          }
          button {
            class: "bg-slate-300 p-1 font-bold rounded w-1/2 mx-auto hover:bg-slate-100",
            onclick: move |_| {
              navigator.push(Route::Home{});
            },
            "BACK"
          }
          button {
            class: "bg-yellow-300 p-1 font-bold rounded w-1/2 mx-auto hover:bg-yellow-100",
            onclick: move |_| async move{
            },
            "LOGOUT"
          }
          button {
            class: "bg-rose-300 p-1 font-bold rounded w-1/2 mx-auto hover:bg-rose-100",
            onclick: move |_| async move {
            },
            "DELETE ACCOUNT"
          }
        }
      }
      )
  }