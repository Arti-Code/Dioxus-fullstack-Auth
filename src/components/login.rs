use dioxus::prelude::*;

use crate::{backend::server_functions::log_in, Route, UserSession};

#[component]
pub fn Login() -> Element {
  let mut username = use_signal(|| String::new());
  let mut password = use_signal(|| String::new());
  let mut error_msg = use_signal(|| String::new());
  let navigator = use_navigator();
  let mut user_session = use_context::<Signal<UserSession>>();
  rsx!(
    div { class: "w-2/3 flex justify-center items-center bg-slate-800 mx-auto my-5",
      div { class: "border-solid justify-center items-center border-2 border-slate-700 rounded-lg px-3 py-5 w-full",
        div { class: "text-center text-3xl", "Login" }
        if !error_msg.to_string().is_empty() {
          div { class: "bg-rose-100 text-rose-600 py-1 px-2 rounded-lg my-3",
            " {error_msg}"
          }
        }
        div { class: "my-5",
          div { class: "text-lg text-slate-300", "username: " }
          input {
            class: "w-full rounded-lg px-2 py-1 bg-slate-100",
            r#type: "text",
            value: username,
            oninput: move |e| username.set(e.value()),
          }
        }
        div { class: "my-5",
          div { class: "text-lg text-slate-300", "password: " }
          input {
            class: "w-full rounded-lg px-2 py-1 bg-slate-100",
            r#type: "password",
            value: password,
            oninput: move |e| password.set(e.value()),
          }
        }
        button {
          class: "flex bg-sky-500 text-slate-200 px-3 py-2 text-bold text-2xl rounded-lg w-1/2 mx-auto my-5 hover:bg-sky-600",
          onclick: move |_| async move {
              match log_in(username(), password()).await {
                  Ok(_) => {
                      user_session.write().login();
                      //s.login();
                      //user_session.set(s);
                      match navigator.push(Route::User {}) {
                          Some(_) => {},
                          None => {},
                      }
                  }
                  Err(e) => {
                      error_msg
                          .set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string())
                  }
              }
          },
          "LOGIN"
        }
        div {
          "Don't have an account ?"
          Link { to: Route::Register {}, class: "text-sky-400", "register now" }
        }
      }
    }
  )
}