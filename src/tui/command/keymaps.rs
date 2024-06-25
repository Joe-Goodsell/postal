use super::{Command, Input, KeyMap};
use crossterm::event::{KeyCode, KeyModifiers};

pub fn get_global_keymap() -> KeyMap {
    let mut key_map = KeyMap::new();

    log::trace!("Building global keymap");

    key_map.insert(
        Input::new(KeyCode::Tab, KeyModifiers::NONE),
        Command::NextControlFocus,
    );

    key_map.insert(
        Input::new(KeyCode::Tab, KeyModifiers::SHIFT),
        Command::PrevControlFocus,
    );

    key_map.insert(
        Input::new(KeyCode::Char('q'), KeyModifiers::NONE),
        Command::Quit,
    );
    key_map.insert(
        Input::new(KeyCode::Char('h'), KeyModifiers::NONE),
        Command::DisplayHelp,
    );
    key_map.insert(
        Input::new(KeyCode::Char('?'), KeyModifiers::NONE),
        Command::DisplayHelp,
    );
    key_map.insert(
        Input::new(KeyCode::Char('q'), KeyModifiers::NONE),
        Command::Quit,
    );

    key_map
}

pub fn get_query_keymap() -> KeyMap {
    let mut key_map = KeyMap::new();
    key_map.insert(
        Input::new(KeyCode::Char('q'), KeyModifiers::NONE),
        Command::Quit,
    );

    key_map
}

pub fn get_response_keymap() -> KeyMap {
    let mut key_map = KeyMap::new();
    key_map.insert(
        Input::new(KeyCode::Char('q'), KeyModifiers::NONE),
        Command::Quit,
    );

    key_map
}

pub fn get_history_keymap() -> KeyMap {
    todo!()
}

pub fn get_help_popup_keymap() -> KeyMap {
    let mut key_map = KeyMap::new();
    key_map.insert(
        Input::new(KeyCode::Char('q'), KeyModifiers::NONE),
        Command::Quit,
    );
    key_map.insert(
        Input::new(KeyCode::Char('c'), KeyModifiers::NONE),
        Command::DismissPopup,
    );
    key_map.insert(
        Input::new(KeyCode::Esc, KeyModifiers::NONE),
        Command::DismissPopup,
    );

    key_map
}
