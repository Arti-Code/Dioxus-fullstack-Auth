
use dioxus::prelude::*;

use crate::components::style::*;

#[component]
pub fn ErrorMsg(e: String) -> Element {
  let mut error_msg = use_signal(|| String::new());
  let _ = use_resource(move || {
    let e = e.clone();
    async move {
        error_msg.set(e.split(":").collect::<Vec<&str>>()[1].to_string());
    }
});
  
  rsx!(
    div { class: STYLE_ERROR_MESSAGE,
        " {error_msg}"
    }
  )
}