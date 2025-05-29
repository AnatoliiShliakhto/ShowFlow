use crate::{app::*, component::*, t};
use ::dioxus::prelude::*;

pub fn Finish() -> Element {
    let state = use_state();
    
    rsx! {
        div {
            class: "flex flex-1 flex-col gap-10 items-center justify-center",
            span {
                class: "text-8xl font-semibold text-accent",
                { t!("message-finish") }
            }
            button {
                class: "btn btn-lg btn-dash flex flex-inline gap-3 fixed bottom-30",
                onclick: move |_| {
                    state.load_playlist()    
                },
                Icon { icon: Icons::Repeat, class: "size-10" },
                { t!("action-restart") }
            }
        }
    }
}
