use ::dioxus::prelude::*;
use crate::{app::*, component::*, t};

pub fn Home() -> Element {
    let state = use_state();
    
    if state.path()().exists() {
        state.refresh_files();
        if !state.queue().is_empty() {
            use_navigator().push(Route::Show {});
        } else {
            use_navigator().push(Route::Manager {});
        }
        return rsx!()
    }

    rsx! {
        div {
            class: "flex flex-1 flex-col gap-10 items-center justify-center",
            h1 {
                class: "text-2xl",
                { t!("message-choose-folder") }
            }
            button {
                class: "btn btn-xl btn-outline",
                onclick: move |_| {
                    let state = use_state();
                    if state.pick_folder() {
                        state.refresh_files();
                        use_navigator().push(Route::Manager {});
                    }                        
                },
                Icon { icon: Icons::Folder, class: "size-10" }
            }
        }
    }
}