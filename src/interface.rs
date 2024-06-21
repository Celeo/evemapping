use crate::{
    eve_data::{parse_paste, Signature, ALL_SYSTEMS, WORMHOLE_TYPES},
    state::{App, ViewMode},
};
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
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
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
        let system_sig_count = app.system_signatures().len();

        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Min(0)].as_ref())
                .split(f.size());

            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Min(0)].as_ref())
                .split(chunks[0]);

            match app.current_system.as_ref() {
                Some(current_system) => {
                    if let Some(data) = ALL_SYSTEMS.get(current_system) {
                        let block = Block::default()
                            .title(current_system.to_string())
                            .borders(Borders::ALL);
                        let mut spans = vec![
                            Spans::from(vec![
                                Span::styled(
                                    "Type: ",
                                    Style::default().add_modifier(Modifier::BOLD),
                                ),
                                Span::styled(
                                    data.classification().as_str(),
                                    style_for_system(&data.classification().as_str()),
                                ),
                            ]),
                            Spans::from(Vec::new()),
                            Spans::from(vec![Span::raw("Static connections:")]),
                        ];
                        if data.class.is_some() {
                            let statics = format_system_statics(&data.statics);
                            spans.extend(statics);
                        }
                        let static_info_p = Paragraph::new(spans).block(block);
                        f.render_widget(static_info_p, top_chunks[0]);
                    }
                }
                None => {
                    f.render_widget(
                        Block::default()
                            .title("No system selected")
                            .borders(Borders::ALL),
                        top_chunks[0],
                    );
                }
            }

            let mut block = Block::default()
                .title("Scanning data")
                .borders(Borders::ALL);
            if app.view == ViewMode::Normal {
                block = block.border_style(Style::default().fg(Color::Yellow));
            }
            let list_items = match app.current_system.as_ref() {
                Some(s) => match app.system_data.get(s) {
                    Some(d) => d.iter().map(|e| ListItem::new(format!("{e}"))).collect(),
                    None => Vec::new(),
                },
                None => Vec::new(),
            };
            let sigs = List::new(list_items)
                .block(block)
                .style(Style::default().fg(Color::White))
                .highlight_symbol(">>  ");
            let mut sigs_state = ListState::default();
            if system_sig_count > 0 {
                sigs_state.select(Some(app.data_index));
            }
            f.render_stateful_widget(sigs, top_chunks[1], &mut sigs_state);

            let block = Block::default().title("Map").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);

            if app.view != ViewMode::Normal {
                let title = match &app.view {
                    ViewMode::Normal => "",
                    ViewMode::Adding(_) => "Add",
                    ViewMode::Editing(sig) => &format!("Edit {}", sig.identifier),
                };
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
                // can always close modals to get back to normal view
                if key.code == KeyCode::Esc {
                    app.view = ViewMode::Normal;
                }

                match &app.view {
                    ViewMode::Normal => {
                        // normal state
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Enter => {
                                if system_sig_count > 0 {
                                    if let Some(current_system) = app.current_system.as_ref() {
                                        if let Some(data) = app.system_data.get(current_system) {
                                            let sigs_to_edit = data.get(app.data_index).unwrap();
                                            app.view = ViewMode::Editing(sigs_to_edit.clone());
                                        }
                                    }
                                }
                            }
                            KeyCode::Down => {
                                if system_sig_count > 1 && app.data_index < system_sig_count - 1 {
                                    app.data_index += 1;
                                }
                            }
                            KeyCode::Up => {
                                if system_sig_count > 1 && app.data_index > 0 {
                                    app.data_index -= 1;
                                }
                            }
                            KeyCode::Char('n') => {
                                app.view = ViewMode::Adding(Signature::default());
                            }
                            KeyCode::Char('v') => {
                                if let Ok(clipboard) = cli_clipboard::get_contents() {
                                    let results = parse_paste(&clipboard);
                                    // TODO
                                }
                            }
                            _ => {}
                        }
                    }
                    ViewMode::Adding(_new_sig) => {}
                    ViewMode::Editing(_edit_sig) => {}
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

/// Styling for the system.
fn style_for_system(leads_to: &str) -> Style {
    if leads_to == "High-Sec" {
        Style::default().fg(Color::Green)
    } else if leads_to == "Low-Sec" {
        Style::default().fg(Color::Yellow)
    } else if leads_to == "Null-Sec" {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Magenta)
    }
}

/// Format the static connections for display.
pub fn format_system_statics(statics: &[String]) -> Vec<Spans> {
    statics
        .iter()
        .map(|s| {
            let data = WORMHOLE_TYPES.get(s).expect("Invalid WH type");
            Spans::from(vec![
                Span::raw("- "),
                Span::raw(s),
                Span::raw(" -> "),
                Span::styled(&data.leads_to, style_for_system(&data.leads_to)),
            ])
        })
        .collect()
}
