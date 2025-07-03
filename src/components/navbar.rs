use dioxus::prelude::*;

use crate::{components::style::*, Route, UserSession};

#[component]
pub fn Navbar() -> Element {
  let navigator = use_navigator();
  let mut user_session = use_context::<Signal<UserSession>>();
  rsx! (
    nav { 
        //class:"bg-slate-900",
        ul {
          class: STYLE_NAVBAR,
          if !user_session.read().is_logged() {
            li {
              class: "flex",
              button { 
                class: my_style(STYLE_NAV_BTN, "border-blue-400 hover:border-blue-500"),
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
                class: my_style(STYLE_NAV_BTN, "border-green-400 hover:border-green-500"), 
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
                class: my_style(STYLE_NAV_BTN, "border-slate-400 hover:border-slate-500"), 
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
                class: my_style(STYLE_NAV_BTN, "border-lime-400 hover:border-lime-500"), 
                onclick: move |_| {
                  //session.write().logout();
                  navigator.replace(Route::Robots{});
                }, 
                "ROBOTS"
              } 
            }
            li {
              class: "flex",
              button {
                class: my_style(STYLE_NAV_BTN, "border-cyan-400 hover:border-cyan-500"),  
                onclick: move |_| {
                  //session.write().logout();
                  navigator.replace(Route::Profile {});
                }, 
                "PROFILE"
              } 
            }
            li {
              class: "flex",
              button {
                class: my_style(STYLE_NAV_BTN, "border-orange-400 hover:border-orange-500"), 
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
    Outlet::<Route> {}
  )
}