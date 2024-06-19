use app::App;
use config::Config;
use runner::run;
use std::io;
use telemetry::init_logging;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

mod app;
mod config;
mod runner;
mod telemetry;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // *** intialise terminal ***
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // *** set up telemetry ***
    init_logging()?;

    // *** get configuration ***
    let config = Config::new().await?;

    // *** run application ***
    run(config, &mut terminal).await?;

    // *** leave application ***
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
