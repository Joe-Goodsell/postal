use std::{collections::HashMap, hash::Hash};

use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::App;

use super::TUIComponents;

mod executions;
pub mod keymaps;

pub type KeyMap = HashMap<Input, Command>;

#[derive(PartialEq, Eq, Hash)]
pub struct Input {
    key: KeyCode,
    modifiers: KeyModifiers,
}

impl Input {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { key, modifiers }
    }
}

// impl Hash for Input {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         todo!()
//     }
// }

pub enum ExecutionResult {
    NotFound,
    Found,
}

#[derive(Clone)]
pub enum Command {
    Quit,
    DisplayHelp,
    EnterInsertMode,
    EnterNormalMode,
    EnterVisualMode,
    SendHttpRequest,
    NextControlFocus,
    PrevControlFocus,
    NextHistoryEntry,
    PrevHistoryEntry,
    DeleteHistoryEntry,
    LoadHistoryEntry,
    NewQuery,
    CycleVerbs,
}

impl Command {
    pub fn execute(&self, tui: &mut TUIComponents, app: &mut App) -> anyhow::Result<()> {
        match self {
            Command::Quit => executions::execute_quit(tui)?,
            Command::DisplayHelp => executions::execute_display_help(tui)?,
            Command::EnterInsertMode => todo!(),
            Command::EnterNormalMode => todo!(),
            Command::EnterVisualMode => todo!(),
            Command::SendHttpRequest => todo!(),
            Command::NextControlFocus => todo!(),
            Command::PrevControlFocus => todo!(),
            Command::NextHistoryEntry => todo!(),
            Command::PrevHistoryEntry => todo!(),
            Command::DeleteHistoryEntry => todo!(),
            Command::LoadHistoryEntry => todo!(),
            Command::NewQuery => todo!(),
            Command::CycleVerbs => todo!(),
        }
        Ok(())
    }
}
