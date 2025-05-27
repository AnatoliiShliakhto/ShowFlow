use crate::{app::*, component::*, t};
use ::dioxus::prelude::*;

pub fn Manager() -> Element {
    rsx! {
        div {
            class:"bg-base-300 rounded-box flex flex-1 flex-col mt-15 mb-5 ml-5",
            div {
                class: "flex justify-between p-5",
                h2 {
                    class: "text-3xl font-bold text-primary",
                    { t!("title-playlist") }
                }
                div {
                    class: "flex",
                    button {
                        class: "btn btn-ghost btn-accent",
                        class: if use_state().queue().is_empty() { "btn-disabled" },
                        onclick: move |_| {
                            use_navigator().push(Route::Show {});
                        },
                        Icon { icon: Icons::Play, class: "size-6" }
                    }
                    button {
                        class: "btn btn-ghost btn-error",
                        class: if use_state().queue().is_empty() { "btn-disabled" },
                        onclick: move |_| {
                            let state = use_state();
                            state.clear_queue();
                            state.save_playlist()
                        },
                        Icon { icon: Icons::Cancel, class: "size-6" }
                    }
                }
            }
            ul {
                class: "list flex-1 overflow-y-auto px-2",
                Queue {}
            }
        }
        div {
            class: "divider divider-horizontal text-2xl text-accent mt-30 mb-20",
            "⇆"
        }
        div {
            class: "bg-base-300 rounded-box flex flex-1 flex-col mt-15 mb-5 mr-5",
            div {
                class: "flex justify-between p-5",
                h2 {
                    class: "text-3xl font-bold text-primary",
                    { t!("title-files") }
                }
                div {
                    class: "flex",
                    button {
                        class: "btn btn-ghost btn-primary",
                        onclick: move |_| {
                            use_state().refresh_files()
                        },
                        Icon { icon: Icons::Refresh, class: "size-6" }
                    }
                    button {
                        class: "btn btn-ghost btn-accent",
                        onclick: move |_| {
                            let state = use_state();
                            if state.pick_folder() {
                                state.refresh_files()
                            }
                        },
                        Icon { icon: Icons::Folder, class: "size-6" }
                    }
                }
            }
            ul {
                class: "list flex-1 overflow-y-auto px-2",
                Files {}
            }
        }
    }
}

pub fn Queue() -> Element {
    let queue = use_state().queue();

    rsx! {
        for (idx, name) in queue().into_iter().enumerate() {
            li {
                class: "list-row hover:bg-base-200 hover:cursor-pointer hover:shadow-md",
                onclick: move |_| {
                    let state = use_state();
                    state.remove_from_queue(idx);
                    state.save_playlist()
                },
                div {
                    class: "text-2xl font-thin tabular-nums opacity-80",
                    { format!("{:02}", idx + 1) }
                }
                div {
                    class: "list-col-grow content-center",
                    div {
                        class: "text-lg uppercase font-semibold",
                        { name }
                    }
                }
            }
        }
    }
}

pub fn Files() -> Element {
    let state = use_state();
    let files = state.files();

    rsx! {
        for (name, name_cloned) in files().iter().map(|item| (item.name.clone(), item.name.clone())) {
            if !state.queue_contains(&name) {
                li {
                    class: "list-row hover:bg-base-200 hover:cursor-pointer hover:shadow-md",
                    onclick: move |_| {
                        let state = use_state();
                        state.add_to_queue(&name_cloned);
                        state.save_playlist()
                    },
                    div {
                        class: "list-col-grow content-center",
                        div {
                            class: "text-lg uppercase font-semibold",
                            { name }
                        }
                    }
                }
            }
        }
    }
}
