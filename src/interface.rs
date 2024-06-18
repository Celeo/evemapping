use crate::state::App;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear},
    Terminal,
};

const EVENT_POLL_RATE: u64 = 5;
const API_POLL_RATE: u64 = 15;

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
    // delay first ESI query
    let mut last_updated = Instant::now();

    // app loop
    loop {
        // update data every few seconds
        if last_updated.elapsed() >= Duration::from_secs(API_POLL_RATE) {
            debug!("Query ESI");
            last_updated = Instant::now();
        }

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

            let mut block = Block::default()
                .title("Scanning data")
                .borders(Borders::ALL);
            if !app.is_adding && !app.is_editing {
                block = block.border_style(Style::default().fg(Color::Yellow));
            }
            f.render_widget(block, top_chunks[1]);

            let block = Block::default().title("Map").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);

            if app.is_adding || app.is_editing {
                let title = if app.is_adding { "Add" } else { "Edit" };
                let block = Block::default()
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(title)
                    .borders(Borders::ALL);
                let area = crate::helpers::centered_rect(40, 40, f.size());
                f.render_widget(Clear, area);
                f.render_widget(block, area);
            }
        })?;

        // keyboard interaction
        if event::poll(Duration::from_secs(EVENT_POLL_RATE))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        app.is_adding = false;
                        app.is_editing = false;
                    }
                    KeyCode::Enter => {
                        if !app.is_adding {
                            app.is_editing = true;
                        }
                    }
                    KeyCode::Down => {}
                    KeyCode::Up => {}
                    KeyCode::Char('q') => break,
                    KeyCode::Char('n') => {
                        if !app.is_editing {
                            app.is_adding = true;
                        }
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
