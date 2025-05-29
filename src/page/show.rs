use crate::{app::*, page::*, component::*, t};
use ::dioxus::prelude::*;

pub fn Show() -> Element {
    let state = use_state();
    let queue = state.queue();
    let busy = use_signal(|| false);

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

    let current = {
        to_owned![state, queue];
        use_memo(move || {
            if queue().is_empty() {
                None
            } else {
                Some(state.current())
            }
        })
    };

    let open = {
        to_owned![state];
        move |idx: usize| {
            to_owned![busy];
            spawn(async move {
                busy.set(true);
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                busy.set(false)
            });
            state.open(idx);
        }
    };

    rsx! {
        div {
            class: "bg-base-300 rounded-box flex flex-1 flex-col py-1",
            if let Some(current) = &*current.read() {
                div {
                    class: "flex flex-1 flex-col justify-center items-center gap-10",
                    div {
                        class: "flex text-6xl uppercase font-semibold text-center p-2",
                        { current.name.as_str() }
                    }
                    if let Some(image) = &current.thumbnail {
                        div {
                            class: "border rounded-lg p-2 border-base-content/30 shadow-xl bg-base-200",
                            class: "hover:scale-110 hover:cursor-pointer hover:bg-accent/40",
                            class: "transition-all duration-300 ease-in-out",
                            onclick: {
                                to_owned![open];
                                move |_| { open(0) }
                            },
                            img {
                                class: "aspect-auto w-100",
                                src: image.as_str(),
                            }
                        }
                    } else {
                        div {
                            class: "inline-flex border rounded-lg p-4 gap-4 text-success group items-center",
                            class: "hover:bg-success hover:cursor-pointer hover:shadow-xl",
                            class: "transition-all duration-300 ease-in-out",
                            onclick: {
                                to_owned![open];
                                move |_| { open(0) }
                            },
                            Icon { icon: Icons::Play, class: "size-10 text-success group-hover:text-base-100" },
                            span {
                                class: "text-2xl text-success group-hover:text-base-100",
                                { t!("action-play") }
                            }
                        }
                    }
                    div {
                        class: "flex gap-10",
                        div {
                            class: "inline-flex border rounded-lg p-4 gap-4 text-error group items-center",
                            class: "hover:bg-error hover:cursor-pointer hover:shadow-xl",
                            class: "transition-all duration-300 ease-in-out",
                            onclick: {
                                to_owned![state];
                                move |_| {
                                    state.remove_from_queue(0)
                                }
                            },
                            Icon { icon: Icons::Skip, class: "size-10 text-error group-hover:text-base-100" },
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
            class: "w-2"
        }
        div {
            class:"flex flex-1 flex-col",
            ul {
                class: "list flex-1 overflow-y-auto px-3 py-1",
                for (idx, name) in queue().into_iter().enumerate() {
                    li {
                        class: "list-row hover:bg-base-200 hover:cursor-pointer hover:shadow-md group",
                        class: "transition-all duration-300 ease-in-out hover:scale-103 hover:z-1",
                        class: if idx == 0 { "outline outline-current/20" },
                        onclick: {
                            to_owned![open];
                            move |_| { open(idx); }
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