use dioxus::prelude::*;
use crate::{
  components::style::*, 
  Route,
};
use crate::backend::server_functions::{create_robot, get_user};

#[component]
pub fn AddRobot() -> Element {
    let mut error_msg = use_signal(|| String::new());
    let mut robot_name = use_signal(|| {
        String::from(format!("Robot"))
    });
    let mut active_user = use_signal(|| (0, String::new()));
    let _ = use_resource(move || async move {
        match get_user().await {
            Ok(data) => {
                active_user.set(data);
            },
            Err(e) => {
                error_msg.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string());
            },
        };
    });
    let navigator = use_navigator();
    rsx!(
        div { class: STYLE_CARD_BOX1,
            div { class: my_style(STYLE_CARD_BOX2, ""),
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
                div {
                    class: "flex justify-around w-full", 
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
                            match create_robot(robot_name.read().clone(), active_user.read().0).await {
                                Ok(_) => {
                                    //robot_name.set(String::from("Robot"));
                                    navigator.push(Route::Robots{});
                                }
                                Err(e) => {
                                    error_msg.set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string());
                                }
                            }
                        },
                        "CREATE"
                    }
                }
            }
        }
    )
}