use dioxus::prelude::*; 

#[component]
pub fn Navbar() -> Element {

    rsx!(
        nav {
            class: "bg-slate-400",
            div {
                class: "mx-auto max-w-7xl px-2 sm:px-6 lg:px-8",
                div {
                    class: "relative flex h-16 items-center justify-between",
                    div {
                        class: "absolute inset-y-0 left-0 flex items-center sm:hidden",
                        button {
                            type: "button", 
                            class: "relative inline-flex items-center justify-center rounded-md p-2 text-gray-300 bg-slate-600 hover:bg-slate-500 hover:text-white focus:ring-2 focus:ring-white focus:outline-hidden focus:ring-inset",
                            "USER"
                        }
                    }
                    div { 
                        class: "flex flex-1 items-center justify-around sm:items-stretch sm:justify-start",
                        div { 
                            class: "flex shrink-0 items-center",
                            /* img {
                                class: "h-8 w-auto", 
                                src: "https://tailwindcss.com/plus-assets/img/logos/mark.svg?color=indigo&shade=500",
                            } */
                            div { 
                                class: "hidden sm:ml-6 sm:block",
                                div { 
                                    class: "flex space-x-4",
                                    a { 
                                        href: "#", 
                                        class: "rounded-md bg-gray-900 px-3 py-2 text-sm font-medium text-white",
                                        "Dashboard"
                                    }
                                    a { 
                                        href: "#",
                                        class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white",
                                        "Team"
                                    }
                                    a { 
                                        href: "#", 
                                        class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white", 
                                        "Projects"
                                    }
                                    a { 
                                        href: "#", 
                                        class: "rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-gray-700 hover:text-white", 
                                        "Calendar"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}
