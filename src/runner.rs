use crate::config::Config;
use crate::{tui::TUIComponents, App};
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use futures_util::stream::StreamExt;
use ratatui::{prelude::Backend, Terminal};

pub async fn run<B: Backend>(config: Config, terminal: &mut Terminal<B>) -> anyhow::Result<()> {
    log::trace!("running app");
    let mut app = App::new(config).await?;
    log::trace!("  ... app created");
    let mut tui_components = TUIComponents::new();
    let mut event_stream = EventStream::new();

    terminal.draw(|mut f| tui_components.render(&mut f, &mut app))?;

    // *** main loop ***
    while let Some(event) = event_stream.next().await {
        log::trace!("registered event");

        let event = event?;

        if let Event::Key(key_event) = event {
            match key_event.kind {
                KeyEventKind::Press => {
                    if key_event.code == KeyCode::Char('q') {
                        break;
                    }
                }
                KeyEventKind::Repeat => {}
                KeyEventKind::Release => {}
            }
        }

        // do stuff in loop
        terminal.draw(|mut f| tui_components.render(&mut f, &mut app))?;
    }

    Ok(())
}

pub async fn handle_input(
    app: &mut App,
    tui: &mut TUIComponents,
    event: &Event,
) -> anyhow::Result<()> {
    // pass to TUIComponents
    // tui.handle_input(event, app).await?;
    Ok(())
}
