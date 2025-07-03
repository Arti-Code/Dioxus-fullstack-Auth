use dioxus::prelude::*;
use crate::{components::style::*, Route};


#[component]
pub fn Robots() -> Element {
  let navigator = use_navigator();
  rsx!(
    div { 
      class: STYLE_CARD_BOX1,
      div {
        class: STYLE_GRID_SINGLE,
        div { 
          class: my_style(STYLE_BUTTON_NO_COLOR, "bg-green-600 hover:bg-green-500"),
          onclick: move |_| {
            navigator.push(Route::AddRobot{});          
          },
          "CREATE"
        }
        div { 
          class: STYLE_GRID_SINGLE,
          for i in 0..5 {
            div { 
              class: STYLE_LABEL_XL_BOLD_MX,
              "ROBOT {i}"
            }
          }
        }
      }
    }
  )
}