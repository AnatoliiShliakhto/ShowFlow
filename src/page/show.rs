use crate::{app::*, page::*, component::*, t};
use ::dioxus::prelude::*;

pub fn Show() -> Element {
    let mut busy = use_signal(|| false);
    let queue = use_state().queue();

    if busy() {
        return rsx! {
            Loading {}
        };
    }

    if queue().is_empty() {
        return rsx! {
            Finish {}
        };
    }

    let current = use_memo(|| {
        if use_state().queue().is_empty() {
            None
        } else {
            Some(use_state().current())
        }
    });

    rsx! {
        div {
            class:"bg-base-300 rounded-box flex flex-1 flex-col mt-15 mb-5 mx-5",
            if let Some(current) = &*current.read() {
                div {
                    class: "flex flex-1 flex-col justify-center items-center gap-10",
                    div {
                        class: "text-6xl uppercase font-semibold",
                        { current.name.clone() }
                    }
                    if let Some(image) = &current.thumbnail {
                        div {
                            class: "border rounded-lg p-2 text-base-content/30 shadow-xl bg-base-200",
                            img {
                                class: "aspect-auto w-100",
                                src: image.clone(),
                            }
                        }
                    }
                    div {
                        class: "flex gap-10",
                        div {
                            class: "inline-flex border rounded-lg p-4 p-4 gap-2 text-success hover:bg-success hover:cursor-pointer group items-center",
                            onclick: move |_| {
                                spawn(async move {
                                    busy.set(true);
                                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                    busy.set(false)
                                });
                                use_state().open(0)
                            },
                            Icon { icon: Icons::Play, class: "size-10 text-success group-hover:text-base-100" },
                            span {
                                class: "text-2xl text-success group-hover:text-base-100",
                                { t!("action-play") }
                            }
                        }
                        div {
                            class: "inline-flex border rounded-lg p-4 gap-2 text-error hover:bg-error hover:cursor-pointer group items-center",
                            onclick: move |_| {
                                use_state().remove_from_queue(0)
                            },
                            Icon { icon: Icons::Close, class: "size-10 text-error group-hover:text-base-100" },
                            span {
                                class: "text-2xl text-error group-hover:text-base-100",
                                { t!("action-skip") }
                            }
                        }
                    }
                }
            }
        }
        div {
            class:"flex flex-1 flex-col mt-15 mb-5 mr-5",
            ul {
                class: "list flex-1 overflow-y-auto",
                for (idx, name) in queue().into_iter().enumerate() {
                    li {
                        class: if idx == 0 {
                            "list-row border shadow-xl hover:bg-base-200 hover:cursor-pointer hover:shadow-md"
                        } else {
                            "list-row hover:bg-base-200 hover:cursor-pointer hover:shadow-md"
                        },
                        onclick: move |_| {
                            spawn(async move {
                                busy.set(true);
                                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                busy.set(false)
                            });
                            use_state().open(idx);
                        },
                        div {
                            class: "text-6xl font-thin tabular-nums opacity-80 text-warning",
                            { format!("{:02}", idx + 1) }
                        }
                        div {
                            class: "list-col-grow content-end pl-2",
                            div {
                                class: "text-4xl uppercase font-semibold",
                                { name }
                            }
                        }
                    }
                }
            }
            div {
                class: "bg-base-100 pointer-events-none absolute bottom-0 right-0 flex h-80",
                class: "z-1 w-1/2",
                class: "[mask-image:linear-gradient(transparent,#000000)]",
            }
        }
    }
}