use dioxus::prelude::*;

use crate::{backend::server_functions::register, Route};
use crate::components::style::*;

#[component]
pub fn Register() -> Element {
  let mut username = use_signal(|| String::new());
  let mut password = use_signal(|| String::new());
  let error_msg = use_signal(|| String::new());
  let navigator = use_navigator();
  rsx!(
    div { 
      class: STYLE_CARD_BOX1,
      div { 
        class: STYLE_CARD_BOX2,
        div { 
          class: STYLE_BOX_LABEL_3XL, "REGISTER" }
          if !error_msg.to_string().is_empty() {
            div { 
              class: STYLE_ERROR_MESSAGE,
              "{error_msg}"
            }
          }
          div { class: STYLE_INPUT_DIV,
            div { class: STYLE_INPUT_LABEL, "username: " }
            input {
              class: STYLE_INPUT,
              r#type: "text",
              value: username,
              oninput: move |e| username.set(e.value()),
            }
          }
          div { class: STYLE_INPUT_DIV,
            div { class: STYLE_INPUT_LABEL, "password: " }
            input {
              class: STYLE_INPUT,
              r#type: "password",
              value: password,
              oninput: move |e| password.set(e.value()),
            }
          }
          button {
            class: STYLE_BUTTON_SKY,
            onclick: move |_| async move {
              match register(username(), password()).await {
                Ok(_) => {
                  match navigator.push(Route::Login {}) {
                    Some(_) => {}
                    None => {}
                  }
                }
                Err(_) => {}
              }
            },
            "REGISTER"
          }
      }
    }
  )
}