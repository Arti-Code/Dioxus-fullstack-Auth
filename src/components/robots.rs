use dioxus::prelude::*;
use crate::{backend::server_functions::get_robots, components::style::*, Route};


#[component]
pub fn Robots() -> Element {
  let mut error_msg = use_signal(|| String::new());
  let navigator = use_navigator();
  let mut robot_list: Signal<Vec<(i64, String, bool)>> = use_signal(|| vec![]);
   let _ = use_resource(move || async move {
        match get_robots().await {
            Ok(data) => {
                robot_list.set(data);
            },
            Err(e) => {
              error_msg.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string());
            },
        };
    });
  rsx!(
    div { 
      class: STYLE_CARD_BOX1,
      div {
        class: STYLE_GRID_SINGLE,
        if !error_msg.to_string().is_empty() {
          div { class: STYLE_ERROR_MESSAGE,
            " {error_msg}"
          }
        }
        div { 
          class: my_style(STYLE_BUTTON_NO_COLOR_1_2, "bg-green-600 hover:bg-green-500"),
          onclick: move |_| {
            navigator.push(Route::AddRobot{});
          },
          "CREATE"
        }
        div { 
          class: STYLE_GRID_SINGLE,
          for (id, name, online) in robot_list.read().iter() {
            div { 
              class: my_style(STYLE_LABEL_XL_BOLD_MX, STYLE_TEXT_XL),
              "[{id}]: {name}"
            }
          }
        }
      }
    }
  )
}