mod cancel;
mod close;
mod exit;
mod folder;
mod language;
mod menu;
mod play;
mod question;
mod refresh;
mod repeat;
mod settings;
mod skip;
mod stack;
mod theme;
mod view;

use ::dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Icons {
    Menu,
    Theme,
    Play,
    Refresh,
    Folder,
    Cancel,
    Close,
    Exit,
    Settings,
    Question,
    Repeat,
    View,
    Stack,
    Language,
    Skip,
}

#[component]
pub fn Icon(#[props] icon: Icons, #[props(into)] class: &'static str) -> Element {
    match icon {
        Icons::Menu => menu::Icon(class),
        Icons::Theme => theme::Icon(class),
        Icons::Play => play::Icon(class),
        Icons::Refresh => refresh::Icon(class),
        Icons::Folder => folder::Icon(class),
        Icons::Cancel => cancel::Icon(class),
        Icons::Close => close::Icon(class),
        Icons::Exit => exit::Icon(class),
        Icons::Settings => settings::Icon(class),
        Icons::Question => question::Icon(class),
        Icons::Repeat => repeat::Icon(class),
        Icons::View => view::Icon(class),
        Icons::Stack => stack::Icon(class),
        Icons::Language => language::Icon(class),
        Icons::Skip => skip::Icon(class),
    }
}
