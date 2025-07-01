use dioxus::prelude::*;

use crate::components::style::STYLE_ROBOT_LABEL;



#[component]
pub fn Home() -> Element {
    rsx!(
        div { 
            class: "border-1 h-1/2 my-4 rounded-lg mx-auto text-slate-300 w-2/3",
            div {
                class: STYLE_ROBOT_LABEL,
                "ROBOTIC CENTER"
            }
        }
    )
}