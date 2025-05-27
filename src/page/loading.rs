use crate::t;
use ::dioxus::prelude::*;

pub fn Loading() -> Element {
    rsx! {
        div {
            class: "flex flex-1 items-center justify-center",
            div {
                class: "inline-flex items-center gap-6",
                span {
                    class: "loading loading-xl"
                }
                span {
                    class: "text-4xl font-semibold",
                    { t!("message-loading") }
                }
            }
        }
    }
}