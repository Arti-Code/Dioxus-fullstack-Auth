use dioxus::prelude::*;
use crate::{
  backend::server_functions::get_user,
  components::style::*, 
  Route
};
use crate::backend::model::UserSession;

#[component]
pub fn Profile() -> Element {
    let navigator = use_navigator();
    let mut active_user = use_signal(|| (0, String::new()));
    let mut error_msg = use_signal(|| String::new());
    //let user_session = use_context::<Signal<UserSession>>();
    let _ = use_resource(move || async move {
        match get_user().await {
            Ok(data) => {
                active_user.set(data);
            },
            Err(e) => {
              error_msg.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string())
            },
        };
    });
    rsx!(
      div { 
        class: STYLE_CARD_BOX1,
        //class: "flex",  
        if !error_msg.to_string().is_empty() {
          div { class: STYLE_ERROR_MESSAGE,
            " {error_msg}"
          }
        }
        div {
          class: STYLE_GRID_SINGLE,
          div { 
            class: my_style(STYLE_FIELD_NO_BORDER, "w-2/3 text-slate-200"),
            "{active_user.read().1.to_uppercase()}"
          }
          button {
            class: my_style(STYLE_FIELD_NO_COLORS, "bg-gray-400 hover:bg-gray-300 w-2/3"),
            onclick: move |_| {
              navigator.push(Route::Home{});
            },
            "BACK"
          }
          button {
            class: my_style(STYLE_FIELD_NO_COLORS, "bg-red-400 hover:bg-red-300 w-2/3"),
            onclick: move |_| async move {
            },
            "DELETE ACCOUNT"
          }
        }
      }
      )
  }