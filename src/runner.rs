use crate::{
    config::Config,
    tui::{command::Input, TUIComponents},
    App,
};
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

        let event: Event = event.expect("failed to parse event in eventstream");
        handle_input(&mut app, &mut tui_components, &event).await?;

        // do stuff in loop
        terminal.draw(|mut f| tui_components.render(&mut f, &mut app))?;

        if tui_components.should_quit {
            tui_components.quit().await?;
            break;
        }
    }

    Ok(())
}

pub async fn handle_input(
    app: &mut App,
    tui: &mut TUIComponents,
    event: &Event,
) -> anyhow::Result<()> {
    // pass to TUIComponents
    if let Event::Key(key_event) = event {
        match key_event.kind {
            KeyEventKind::Press => {
                let input = Input::new(key_event.code, key_event.modifiers);
                tui.handle_input(&input, app).await?;
            }
            _ => {}
        }
    }
    Ok(())
}
