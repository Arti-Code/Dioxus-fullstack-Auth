use dioxus::prelude::*;

use crate::{request::get_user, Route};

#[component]
pub fn User() -> Element {
  let navigator = use_navigator();
  let mut is_log = use_signal(|| false);
  //let mut message = use_signal(|| String::new());
  let mut user_signal = use_signal(|| crate::user::User::default());
  let _ = use_resource(move || async move {
    match get_user().await {
      Ok(user) => {
        is_log.set(true);
        user_signal.set(user);
      },
      Err(_e) => {
        is_log.set(false);
        //message.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string());
      }
    }
  });
  rsx!(
    div { class: "flex justify-center items-center screen",
      div { class: "text-5xl", "{user_signal.read().name}" }
      if is_log() {
        button {
          class: "px-1 py-2 rounded-lg bg-slate-100 hover:bg-slate-200",
          onclick: move |_| async move {
              /* if let Ok(_) = log_out().await {
                  is_log.set(false);
                  match navigator.push(Route::Login {}) {
                      Some(_) => {}
                      None => {}
                  }
              } */
          },
          "log out"
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
          "login now"
        }
      }
    }
  )
}