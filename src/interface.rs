use crate::state::App;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::debug;
use rfesi::prelude::Esi;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState},
    Terminal,
};

const EVENT_POLL_RATE: u64 = 5;
const API_POLL_RATE: u64 = 15;

/// Run the TUI.
pub async fn run(_esi: Esi) -> Result<()> {
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
            let list_items = match app.current_system.as_ref() {
                Some(s) => match app.system_data.get(s) {
                    Some(d) => d.iter().map(|e| ListItem::new(format!("{e}"))).collect(),
                    None => Vec::new(),
                },
                None => Vec::new(),
            };
            let anoms = List::new(list_items)
                .block(block)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().bg(Color::White))
                .highlight_symbol(">> ");
            let mut anoms_state = ListState::default();
            if app.system_anomalies().len() > 0 {
                anoms_state.select(Some(app.data_index));
            }
            f.render_stateful_widget(anoms, top_chunks[1], &mut anoms_state);

            let block = Block::default().title("Map").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);

            if app.is_adding || app.is_editing {
                let title = if app.is_adding { "Add" } else { "Edit" };
                let block = Block::default()
                    .border_style(Style::default().fg(Color::Yellow))
                    .title(title)
                    .borders(Borders::ALL);
                let area = centered_rect(40, 40, f.size());
                f.render_widget(Clear, area);
                f.render_widget(block, area);
            }
        })?;

        // keyboard interaction
        if event::poll(Duration::from_secs(EVENT_POLL_RATE))? {
            if let Event::Key(key) = event::read()? {
                // keys that are always active
                match key.code {
                    KeyCode::Esc => {
                        app.is_adding = false;
                        app.is_editing = false;
                    }
                    _ => {}
                }

                if app.is_adding {
                    // ...
                } else if app.is_editing {
                    // ...
                } else {
                    // normal state
                    let anoms = app.system_anomalies();
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Enter => {
                            if anoms.len() > 0 {
                                app.is_editing = true;
                            }
                        }
                        KeyCode::Down => {
                            if anoms.len() > 1 && app.data_index < anoms.len() - 1 {
                                app.data_index += 1;
                            }
                        }
                        KeyCode::Up => {
                            if anoms.len() > 1 && app.data_index > 0 {
                                app.data_index -= 1;
                            }
                        }
                        KeyCode::Char('n') => {
                            app.is_adding = true;
                        }
                        _ => {}
                    }
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

/// https://github.com/fdehau/tui-rs/blob/master/examples/popup.rs#L103
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
