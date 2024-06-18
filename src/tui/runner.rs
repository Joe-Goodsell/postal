use crate::config::Config;
use ratatui::{prelude::Backend, Terminal};

pub async fn run<B: Backend>(config: Config, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
    // handle_event(event: Event, &mut app)

    Ok(())
}
