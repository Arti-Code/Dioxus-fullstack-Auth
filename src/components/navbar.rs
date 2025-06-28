use dioxus::prelude::*;

use crate::{Route, UserSession};

#[component]
pub fn Navbar() -> Element {
  let navigator = use_navigator();
  let mut user_session = use_context::<Signal<UserSession>>();
  rsx! (
    nav { class:"bg-slate-900",
      div { class:"flex w-2/3 justify-center",
        ul {
          class:"flex row justify-left align-center w-full",
          if !user_session.read().is_logged() {
            li {
              class: "flex",
              button { 
                class: "bg-slate-800 mx-8 my-3 px-6 py-3 border-blue-400 border-solid border-2 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700 hover:border-blue-500",
                onclick: move |_| {
                  //session.write().logout();
                  navigator.push(Route::Register{});
                  
                }, 
                "REGISTER" 
              }
            }
            li {
              class: "flex",
              button { 
                class: "bg-slate-800 mx-8 my-3 px-6 py-3 border-green-400 border-solid border-2 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700 hover:border-green-500",
                  onclick: move |_| {
                    //session.write().logout();
                    navigator.push(Route::Login{});
                  }, 
                  "LOGIN" 
              }
            }
          } else {
            li {
              class: "flex",
              button { 
                class: "bg-slate-800 mx-8 my-3 px-6 py-3 border-slate-400 border-solid border-2 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700 hover:border-slate-300",
                onclick: move |_| {
                  //session.write().logout();
                  navigator.replace(Route::Home{});
                }, 
                "HOME"
              } 
            }
            li {
              class: "flex",
              button {
                class: "bg-slate-800 mx-8 my-3 px-6 py-3 border-cyan-400 border-solid border-2 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-cyan-700 hover:border-cyan-500",
                "ROBOTS" 
              }
            }
            li {
              class: "flex",
              button { 
                class: "bg-slate-800 mx-8 my-3 px-6 py-3 border-cyan-400 border-solid border-2 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700 hover:border-cyan-500",
                "PROFILE"
              } 
            }
            li {
              class: "flex",
              button { 
                class: "bg-slate-800 mx-8 my-3 px-6 py-3 border-orange-400 border-solid border-2 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700 hover:border-orange-500",
                onclick: move |_| {
                  let s = user_session.write().clone();
                  if s.is_logged() {
                    user_session.write().logout();
                    navigator.replace(Route::Home{});
                  }
                },
                "LOGOUT" 
              }
            }
          }
        }
      }
    }
    Outlet::<Route> {}
  )
}