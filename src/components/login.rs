use dioxus::prelude::*;

use crate::MySession;
use crate::Route; 
use crate::request::*;

#[component]
pub fn Login() -> Element {
  let mut username = use_signal(|| String::new());
  let mut password = use_signal(|| String::new());
  let mut error_msg = use_signal(|| String::new());
  //let client = use_context::<Signal<reqwest::Client>>();
  let navigator = use_navigator();
  let mut my_session = use_context::<Signal<MySession>>();
  rsx!(
    div { class: "screen flex justify-center items-center bg-slate-200",
      div { class: "border-solid border-2 border-slate-600 rounded-lg px-2 py-2 w-3/4",
        div { class: "text-center text-3xl text-slate-200", "Login" }
        if !error_msg.to_string().is_empty() {
          div { class: "bg-rose-100 text-rose-600 py-1 px-2 rounded-lg my-3",
            " {error_msg}"
          }
        }
        div { class: "my-5",
          div { class: "text-2xl text-slate-200", "username: " }
          input {
            class: "w-full rounded-lg px-2 py-1 text-slate-200",
            r#type: "text",
            value: username,
            oninput: move |e| username.set(e.value()),
          }
        }
        div { class: "my-5",
          div { class: "text-lg text-slate-200", "password: " }
          input {
            class: "w-full rounded-lg px-2 py-1 text-slate-200",
            r#type: "password",
            value: password,
            oninput: move |e| password.set(e.value()),
          }
        }
        button {
          class: "bg-sky-500 text-slate-50 px-3 py-2 rounded-lg w-full my-5 hover:bg-sky-600",
          onclick: move |_| async move {
              //let c = client.read().clone();
              match login(username(), password()).await {
                  Ok(s) => {
                      my_session.set(MySession(s));
                      match navigator.push(Route::User {}) {
                          Some(_) => {}
                          None => {}
                      }
                  }
                  Err(e) => {
                      error_msg
                          .set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string())
                  }
              }
          },
          "Login"
        }
        div {
          "Don't have an account ?"
          Link { to: Route::Register {}, class: "text-sky-400", "register now" }
        }
      }
    }
  )
}