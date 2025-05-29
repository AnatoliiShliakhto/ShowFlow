use crate::{element::*, component::*, app::*};
use ::dioxus::prelude::*;

pub fn Header() -> Element {
    let state = use_state();
    let nav = use_navigator();
    let is_queue_empty = use_memo(|| use_state().queue().is_empty());
    
    rsx! {
        nav {
            class: "navbar p-0 items-center",
            div {
                class: "flex justify-start items-center pl-5",
                button {
                    class: "btn btn-ghost btn-primary justify-start",
                    class: if is_queue_empty() { "btn-disabled" },
                    onclick: move |_| {
                        nav.push(Route::Show {});
                    },
                    Icon { icon: Icons::View, class: "size-6" }
                }                
                button {
                    class: "btn btn-ghost btn-accent justify-start",
                    onclick: move |_| {
                        state.load_playlist();
                        nav.push(Route::Manager {});
                    },
                    Icon { icon: Icons::Stack, class: "size-6" }
                }                
            }
            div {
                class: "flex-1"
            }
            div {
                class: "flex flex-nowrap mr-5",
                Languages {}
                Themes {}
                Menu {}
            }
        }
    }
}
