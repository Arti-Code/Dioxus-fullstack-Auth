use dioxus::prelude::*;

use crate::{Route, UserSession};

#[component]
pub fn Navbar() -> Element {
  let navigator = use_navigator();
  //let mut session = use_context::<Signal<MySession>>();
  //let client = use_context::<Signal<reqwest::Client>>();
  //let s = session.read().clone();
  let user_session = use_context::<Signal<UserSession>>();
  rsx! (
    nav { class:"bg-slate-900",
          div { class:"flex w-2/3 justify-center",
                ul {
                  class:"flex row justify-left align-center w-full",
                  if !user_session.read().is_logged() {
                    li {
                      class: "flex",
                      button { 
                        class: "bg-slate-800 mx-8 my-3 px-6 py-3 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700",
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
                        class: "bg-slate-800 mx-8 my-3 px-6 py-3 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700",
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
                        class: "bg-slate-800 mx-8 my-3 px-6 py-3 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700",
                        //class:"border-solid border-2 bg-slate-300 rounded-md px-3 py-2 text-sm font-medium text-gray-400 hover:bg-slate-700 hover:text-white",
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
                        class: "bg-slate-800 mx-8 my-3 px-6 py-3 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700",
                        "ROBOTS" 
                      }
                    }
                    li {
                      class: "flex",
                      button { 
                        class: "bg-slate-800 mx-8 my-3 px-6 py-3 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700",
                        "PROFILE"
                      } 
                    }
                    li {
                      class: "flex",
                      button { 
                        class: "bg-slate-800 mx-8 my-3 px-6 py-3 rounded-xl text-center text-bold text-2xl text-slate-200 hover:bg-slate-700", 
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