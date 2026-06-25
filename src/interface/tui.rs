use ratatui::{prelude::*, widgets::*};
use crossterm::event::{self, Event, KeyCode};
use crate::util::error::YetiResult;
use std::{time::Duration, path::PathBuf};

pub fn launch_ui(file: PathBuf) -> YetiResult<()> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(format!(" YETI Forensic DNA - Analysing: {:?} ", file))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            
            let text = vec![
                Line::from(vec![Span::raw("Press "), Span::styled("Q", Style::default().fg(Color::Red)), Span::raw(" to exit.")]),
                Line::from("System Status: Forensic Engine Active"),
                Line::from("Worker Threads: 8 (Rayon Parallel)"),
            ];
            
            let p = Paragraph::new(text).block(block).alignment(Alignment::Center);
            f.render_widget(p, size);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') { break; }
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}