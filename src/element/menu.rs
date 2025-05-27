use crate::{app::*, component::*, t};
use ::dioxus::{desktop::use_window, prelude::*};

pub fn Menu() -> Element {
    rsx! {
        div {
            class: "dropdown dropdown-end block",
            title: t!("menu-title"),
            button {
                class: "btn btn-ghost join-item",
                tabindex: 0,
                Icon { icon: Icons::Menu, class: "size-6" }
            }
            ul {
                class: "dropdown-content bg-base-200 text-base-content rounded-box",
                class: "top-px max-h-[calc(100vh-11rem)] w-52 overflow-y-hidden",
                class: "border border-white/5 shadow-2xl outline-1 outline-black/5 mt-11 z-500",
                tabindex: 0,
                li {
                    button {
                        class: "btn btn-block btn-ghost justify-start",
                        class: if use_state().queue().is_empty() { "btn-disabled" },
                        onclick: move |_| {
                            use_navigator().push(Route::Show {});
                        },
                        Icon { icon: Icons::View, class: "size-6" }
                        { t!("menu-play") }
                    }
                }
                li {
                    button {
                        class: "btn btn-block btn-ghost justify-start",
                        onclick: move |_| {
                            use_state().load_playlist();
                            use_navigator().push(Route::Manager {});
                        },
                        Icon { icon: Icons::Stack, class: "size-6" }
                        { t!("menu-playlist") }
                    }
                }
                div {
                    class: "divider my-0"
                }
                li {
                    button {
                        class: "btn btn-block btn-ghost justify-start",
                        onclick: move |_| {
                            use_window().close();
                        },
                        Icon { icon: Icons::Exit, class: "size-6" }
                        { t!("menu-exit") }
                    }
                }
            }
        }
    }
}
