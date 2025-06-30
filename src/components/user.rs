use dioxus::prelude::*;

use crate::{backend::server_functions::{get_user, log_out}, Route};

#[component]
pub fn User() -> Element {
  let navigator = use_navigator();
  let mut is_log = use_signal(|| false);
  let mut active_user = use_signal(|| (0, String::new()));
  let mut message = use_signal(|| String::new());
  let _ = use_resource(move || async move {
    match get_user().await {
      Ok(data) => {
        is_log.set(true);
        active_user.set(data);
      },
      Err(e) => {
        is_log.set(false);
        message.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string());
      }
    }
  });
  rsx!(
    div { class: "flex justify-center items-center screen",
      div { class: "text-5xl", "{message}" }
      if is_log() {
        button {
          class: "px-1 py-2 rounded-lg bg-slate-100 hover:bg-slate-200",
          onclick: move |_| async move {
              if let Ok(_) = log_out().await {
                  is_log.set(false);
                  match navigator.push(Route::Login {}) {
                      Some(_) => {}
                      None => {}
                  }
              }
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


#[component]
pub fn User2() -> Element {
  //let navigator = use_navigator();
  let mut is_log = use_signal(|| false);
  let mut active_user = use_signal(|| (0, String::new()));
  let mut message = use_signal(|| String::new());
  let _ = use_resource(move || async move {
    match get_user().await {
      Ok(data) => {
        is_log.set(true);
        active_user.set(data);
      },
      Err(e) => {
        is_log.set(false);
        message.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string());
      },
    };
  });
  rsx!(
    div { 
      class: "flex justify-center items-center screen",
      div {
        class: "flex columns w-3/5",
        div { 
          class: "flex row justify-center items-center w-full",
          div { 
            class: "flex bg-sky-800 border-solid border-2 border-slate-400 text-center text-bold text-2xl text-slate-200",
            "{active_user.read().0}"                                                                                                                                                                                                                                                                                                                                                                                 
          }
          div { 
            class: "flex bg-sky-800 border-solid border-2 border-slate-400 text-center text-bold text-2xl text-slate-200",
            "{active_user.read().1}"                                                                                                                                                                                                                                                                                                                                                                                 
          }
        }
      }
    }
  )
}