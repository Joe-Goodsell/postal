use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{Constraint, Direction, Layout},
    Frame,
};

use crate::tui::{
    footer::Footer,
    history_pane::HistoryPane,
    popup::*,
    query_pane::{QueryPane, QueryPaneControlMode},
    response_pane::ResponsePane,
};
use crate::App;

pub mod command;
mod footer;
mod history_pane;
mod popup;
mod query_pane;
mod response_pane;
mod utils;

pub enum ControlMode {
    Query,
    Response,
    History,
}

pub struct TUIComponents {
    query_pane: QueryPane,
    response_pane: ResponsePane,
    history_pane: HistoryPane,
    footer: Footer,
    pub control: ControlMode,
    popup_stack: Vec<Popup>,
    pub should_quit: bool,
    //*** keymappings ***
    response_keymap: command::KeyMap,
    global_keymap: command::KeyMap,
}

impl TUIComponents {
    pub fn new() -> TUIComponents {
        Self {
            query_pane: QueryPane::new(),
            response_pane: ResponsePane::new(),
            history_pane: HistoryPane::new(),
            footer: Footer::new(),
            control: ControlMode::Query,
            popup_stack: vec![],
            should_quit: false,
            response_keymap: command::keymaps::get_response_keymap(),
            global_keymap: command::keymaps::get_global_keymap(),
        }
    }

    pub fn push_popup(&mut self, popup: Popup) {
        self.popup_stack.push(popup);
    }

    pub fn render(&mut self, f: &mut Frame<'_>, app: &mut App) {
        log::trace!("rendering TUIComponents");
        let area = f.size();

        // render components

        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(2)])
            .split(area);

        self.footer.render(f, app, split[1]);

        let split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(split[0]);
        let history_pane_area = split[0];

        self.history_pane.render(f, app, history_pane_area);

        let split = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(split[1]);

        let query_pane_area = split[0];
        let response_pane_area = split[1];

        self.response_pane.render(f, app, response_pane_area);
        self.query_pane.render(f, app, query_pane_area);
    }

    pub async fn handle_input(
        &mut self,
        input: &command::Input,
        app: &mut App,
    ) -> anyhow::Result<()> {
        // input handling depends on app mode + currently selected pane + popups
        if !self.popup_stack.is_empty() {
            // handle popup input
        } else {
            match self.control {
                ControlMode::Query => self.handle_input_query(input, app).await?,
                ControlMode::Response => self.handle_input_response(input, app).await?,
                ControlMode::History => self.handle_input_history(input, app).await?,
                _ => {}
            }
        }

        Ok(())
    }

    pub async fn quit(&mut self) -> anyhow::Result<()> {
        // TODO: save and cleanup here... ?
        Ok(())
    }

    async fn handle_input_global(
        &mut self,
        input: &command::Input,
        app: &mut App,
    ) -> anyhow::Result<()> {
        if let Some(cmd) = self.global_keymap.get(input).cloned() {
            cmd.execute(self, app)?;
        }
        Ok(())
    }

    // *** private input handlers ***
    async fn handle_input_query(
        &mut self,
        input: &command::Input,
        app: &mut App,
    ) -> anyhow::Result<()> {
        // if query pane's focus is `None` then use global bindings
        if self.query_pane.control == QueryPaneControlMode::None {
            // WARN: no way to enter query mode but don't want to be mutating TUIComponents within
            // query pane
            self.handle_input_global(input, app).await?;
        } else {
            // Else, delegate to query pane
            self.query_pane.handle_input(input, app).await?;
        }
        Ok(())
    }
    async fn handle_input_response(
        &mut self,
        input: &command::Input,
        app: &mut App,
    ) -> anyhow::Result<()> {
        todo!()
    }
    async fn handle_input_history(
        &mut self,
        input: &command::Input,
        app: &mut App,
    ) -> anyhow::Result<()> {
        todo!()
    }
    async fn handle_input_popup(
        &mut self,
        input: &command::Input,
        app: &mut App,
    ) -> anyhow::Result<()> {
        todo!()
    }
    // *** ***

    fn next_control_mode(&mut self) {
        match self.control {
            ControlMode::Query => self.control = ControlMode::Response,
            ControlMode::Response => self.control = ControlMode::History,
            ControlMode::History => self.control = ControlMode::Query,
            _ => {}
        }
    }
    fn prev_control_mode(&mut self) {
        match self.control {
            ControlMode::Query => self.control = ControlMode::History,
            ControlMode::Response => self.control = ControlMode::Query,
            ControlMode::History => self.control = ControlMode::Response,
            _ => {}
        }
    }
}
