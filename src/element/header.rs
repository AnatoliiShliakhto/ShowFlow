use crate::{element::*, component::*, app::*};
use ::dioxus::prelude::*;

pub fn Header() -> Element {
    rsx! {
        nav {
            class: "navbar p-0 items-center",
            div {
                class: "flex justify-start items-center pl-5",
                button {
                    class: "btn btn-ghost btn-primary justify-start",
                    class: if use_state().queue().is_empty() { "btn-disabled" },
                    onclick: move |_| {
                        use_navigator().push(Route::Show {});
                    },
                    Icon { icon: Icons::View, class: "size-6" }
                }                
                button {
                    class: "btn btn-ghost btn-accent justify-start",
                    onclick: move |_| {
                        use_state().load_playlist();
                        use_navigator().push(Route::Manager {});
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
