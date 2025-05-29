use ::std::path::PathBuf;
use ::dioxus::prelude::*;
use crate::{app::*, component::*, t};

pub fn Home() -> Element {
    let state = use_state();
    let path = PathBuf::from(state.path());
    let queue = state.queue();
    let nav = use_navigator();
    
    if path.exists() {
        state.refresh_files();
        if !queue.is_empty() {
            nav.push(Route::Show {});
        } else {
            nav.push(Route::Manager {});
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
                onclick: {
                    to_owned![state, nav];
                    move |_| {
                        if state.pick_folder() {
                            state.refresh_files();
                            nav.push(Route::Manager {});
                        }
                    }
                },
                Icon { icon: Icons::Folder, class: "size-10" }
            }
        }
    }
}