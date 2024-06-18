use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::time::Duration;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

use crate::state::App;

/// Run the TUI.
pub async fn run() -> Result<()> {
    // configure terminal
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    enable_raw_mode()?;
    terminal.hide_cursor()?;

    let mut app = App::new();

    // app loop
    loop {
        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Min(0)].as_ref())
                .split(f.size());

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Min(0)].as_ref())
                .split(chunks[0]);

            let block = Block::default().title("System data").borders(Borders::ALL);
            f.render_widget(block, top_chunks[0]);

            let mut block = Block::default().title("Data").borders(Borders::ALL);
            if !app.is_adding && !app.is_editing {
                block = block.border_style(Style::default().fg(Color::Yellow));
            }
            f.render_widget(block, top_chunks[1]);

            let block = Block::default().title("Map").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        // keyboard interaction
        if event::poll(Duration::from_secs(5))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        app.is_adding = false;
                        app.is_editing = false;
                    }
                    KeyCode::Char('q') => break,
                    KeyCode::Char('n') => {
                        app.is_adding = true;
                    }
                    _ => {}
                }
            }
        }
    }

    // exit, restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
