use dioxus::prelude::*;

use crate::{
  backend::server_functions::log_in, 
  components::style::*, 
  Route, UserSession
};

#[component]
pub fn Login() -> Element {
  let mut username = use_signal(|| String::from("arturek"));
  let mut password = use_signal(|| String::from("arrakis"));
  let mut error_msg = use_signal(|| String::new());
  let navigator = use_navigator();
  let mut user_session = use_context::<Signal<UserSession>>();
  rsx!(
    div { class: STYLE_CARD_BOX1,
      div { class: STYLE_CARD_BOX2,
        div { class: STYLE_BOX_LABEL_3XL, "LOGIN" }
        if !error_msg.to_string().is_empty() {
          div { class: STYLE_ERROR_MESSAGE,
            " {error_msg}"
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
              match log_in(username(), password()).await {
                  Ok(_) => {
                      user_session.write().login();
                      match navigator.push(Route::Profile{}) {
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
      }
    }
  )
}