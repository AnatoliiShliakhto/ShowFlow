use crate::{app::Route, page::*, element::*};
use ::dioxus::prelude::*;

pub fn Layout() -> Element {
    rsx! {
        div {
            class: "h-dvh w-full flex",
            "oncontextmenu": "return false;",
            div {
                class: "text-base-content fixed top-0 left-0 right-0 z-100",
                class: "flex h-14 w-full justify-center",
                class: "print:hidden",
                Header {}
            }
            SuspenseBoundary {
                fallback: |_context: SuspenseContext| rsx! {
                    Loading {}
                },
                Outlet::<Route> {}
            }
        }
    }
}
