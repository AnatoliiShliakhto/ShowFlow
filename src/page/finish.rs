use crate::{component::*, t};
use ::dioxus::prelude::*;
use crate::app::use_state;

pub fn Finish() -> Element {
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
                    use_state().load_playlist()    
                },
                Icon { icon: Icons::Repeat, class: "size-10" },
                { t!("action-restart") }
            }
        }
    }
}
