use dioxus::prelude::*; 
//use ::rand::prelude::*;
//use rand::rng;
use crate::{
  components::style::*, 
  Route,
};

#[component]
pub fn AddRobot() -> Element {
    let mut robot_name = use_signal(|| {
        //let mut rng = rng();
        String::from(format!("Robot"))
    });
    let mut error_msg = use_signal(|| String::new());
    let navigator = use_navigator();
    rsx!(
        div { class: STYLE_CARD_BOX1,
            div { class: STYLE_CARD_BOX2,
            div { class: STYLE_BOX_LABEL_3XL, "CREATE ROBOT" }
            if !error_msg.to_string().is_empty() {
                div { class: STYLE_ERROR_MESSAGE,
                " {error_msg}"
                }
            }
            div { class: STYLE_INPUT_DIV,
                div { class: STYLE_INPUT_LABEL, "robot name: " }
                input {
                class: STYLE_INPUT,
                r#type: "text",
                value: robot_name,
                oninput: move |e| robot_name.set(e.value()),
                }
            }
             button {
                class: my_style(STYLE_BUTTON_NO_COLOR, "bg-yellow-600 hover:bg-yellow-500"),
                onclick: move |_| async move {
                    navigator.push(Route::Robots{});
                },
                "CANCEL"
            }
            button {
                class: my_style(STYLE_BUTTON_NO_COLOR, "bg-green-600 hover:bg-green-500"),
                onclick: move |_| async move {
                    navigator.push(Route::Robots{});
                },
                "CREAtE"
            }
        }
    }
  )
}