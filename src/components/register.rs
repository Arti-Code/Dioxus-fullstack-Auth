use dioxus::prelude::*;

use crate::{backend::server_functions::register, Route};

#[component]
pub fn Register() -> Element {
  let mut username = use_signal(|| String::new());
  let mut password = use_signal(|| String::new());
  let mut error_msg = use_signal(|| String::new());
  let navigator = use_navigator();

  rsx!(
    div { class: "screen flex justify-center items-center bg-slate-600",
      div { class: "border-solid border-2 border-slate-500 rounded-lg px-3 py-5 w-2/4",
        div { class: "text-center text-slate-300 text-3xl", "Register" }
        if !error_msg.to_string().is_empty() {
          div { class: "bg-rose-300 text-rose-600 py-1 px-2 rounded-lg my-3",
            " {error_msg}"
          }
        }
        div { class: "my-5",
          div { class: "text-lg", "username: " }
          input {
            class: "w-full rounded-lg px-2 py-1 text-slate-300",
            r#type: "text",
            value: username,
            oninput: move |e| username.set(e.value()),
          }
        }
        div { class: "my-5",
          div { class: "text-lg text-slate-300", "password: " }
          input {
            class: "w-full rounded-lg px-2 py-1",
            r#type: "password",
            value: password,
            oninput: move |e| password.set(e.value()),
          }
        }
        button {
          class: "bg-sky-500 text-slate-300 px-3 py-2 rounded-lg w-full my-5 hover:bg-sky-600",
          onclick: move |_| async move {
              match register(username(), password()).await {
                  Ok(_) => {
                      match navigator.push(Route::Login {}) {
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
          "Register"
        }
        div {
          "Already have an account ?"
          Link { to: Route::Login {}, class: "text-sky-500", "login now" }
        }
      }
    }
  )
}

#[component]
pub fn Register2() -> Element {
  let mut username = use_signal(|| String::new());
  let mut password = use_signal(|| String::new());
  let mut error_msg = use_signal(|| String::new());
  let navigator = use_navigator();
  rsx!(
    div { class: "w-2/3 flex justify-center items-center bg-slate-800 mx-auto my-5",
      div { class: "border-solid justify-center items-center border-2 border-slate-700 rounded-lg px-3 py-5 w-full",
          div {
            label {
              class: "text-lg text-slate-300", 
              "User"
            }
            div { 
              class: "mx-5", 
              input {
                class: "block w-full border-2 border-slate-500 border-solid rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6", 
                r#type: "text",
                value: username,
                oninput: move |e| username.set(e.value()),
              }
            }
            div {
              div { 
                class: "flex items-center justify-between", 
                label {
                  class: "text-lg text-slate-300", 
                  "Password",
                }
              }
            }
            div { 
              class: "mt-2", 
              input {
                class: "block w-full border-2 border-slate-500 border-solid rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6", 
                r#type: "password",
                  value: password,
                  oninput: move |e| password.set(e.value()),
              }
            }
            div {
              /* button {
                class: "flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600", 
                "Sign in",
              } */
              class: "mt-4 mx-auto items-center justify-center",
              button {
                class: "text-black bg-sky-500 p-1 rounded w-3/4 hover:bg-sky-600",
                onclick: move |_| async move {
                  match register(username(), password()).await {
                    Ok(_) => {
                        match navigator.push(Route::Login {}) {
                            Some(_) => {}
                            None => {}
                        }
                    }
                    Err(e) => {
                        error_msg.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string())
                    }
                  }
                },
                "REGISTER"
              }
          }
          }
        }
      }
    )
}