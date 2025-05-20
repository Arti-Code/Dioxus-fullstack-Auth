use dioxus::prelude::*;

use crate::{request::{get_user, logout}, MySession, Route};

#[component]
pub fn User() -> Element {
  let navigator = use_navigator();
  let mut is_log = use_signal(|| false);
  //let mut message = use_signal(|| String::new());
  let mut user_signal = use_signal(|| String::new());
  let my_session = use_context::<Signal<MySession>>();
  let _ = use_resource(move || async move {
    //let c = client.read().clone();
    match get_user(my_session.read().clone()).await {
      Ok(user) => {
        is_log.set(true);
        user_signal.set(user.username.clone());
      },
      Err(_e) => {
        is_log.set(false);
        //message.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string());
      }
    }
  });
  rsx!(
    div { class: "flex flex-col justify-center screen",
      div { class: "flex text-5xl", "{user_signal.read()}" }
      if is_log() {
        div { class: "flex",
          button {
            class: "px-1 py-2 rounded-lg bg-slate-100 hover:bg-slate-200",
            onclick: move |_| async move {
                if let Ok(_) = logout(my_session.read().clone()).await {
                    is_log.set(false);
                    match navigator.push(Route::Login {}) {
                        Some(_) => {}
                        None => {}
                    }
                }
            },
            "Logout"
          }
        }
      } else {
        button {
          class: "px-1 py-2 rounded-lg bg-slate-100 hover:bg-slate-200",
          onclick: move |_| {
              match navigator.push(Route::Login {}) {
                  Some(_) => {}
                  None => {}
              }
          },
          "LogIn Now"
        }
      }
    }
  )
}