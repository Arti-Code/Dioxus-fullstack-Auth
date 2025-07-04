use dioxus::prelude::*;
use crate::{backend::server_functions::get_robots, components::style::*, Route, OFFLINE128, ONLINE128, ROBOT128};


#[component]
pub fn Robots2() -> Element {
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
      class: "w-2/3 mx-auto my-4",
        ul { class: "divide-y divide-gray-100", role: "list",
          for (id, name, online) in robot_list.read().iter() {
            RobotElement { id: *id, name: name.clone(), online: *online }
          }
        }
    }
  ) 
} 

#[component]
fn RobotElement(id: i64, name: String, online: bool) -> Element {
  rsx!(
      li { class: "flex justify-between gap-x-6 py-5",
        div { class: "flex min-w-0 gap-x-4",
          img {
            class: "size-24 flex-none rounded-full",
            src: ROBOT128,
          }
          div { class: "min-w-0 flex-auto",
            p { class: "text-xl font-bold text-slate-200", "{name.to_uppercase()}" }
            p { class: "mt-1 text-xl text-slate-200",
                "ID: {id}"
            }
          }
          div {
            class: "",
            img {
              class: "size-12 flex-none rounded-full",
              src: if online { ONLINE128 } else { OFFLINE128 },
            }
            //class: my_style("font-bold text-3xl rounded-xl px-2 text-center align-center", if online {"text-green-500"} else {"text-red-500"}),
            //"⊚"
          }
        }
      }
  )
}