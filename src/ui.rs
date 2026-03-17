use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Paragraph},
};

pub fn draw(frame: &mut Frame, label: &str, remaining: u64, total: u64) {
    let percent = ((total - remaining) as f64 / total as f64 * 100.0) as u16;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let mins = remaining / 60;
    let secs = remaining % 60;
    let time_str = format!("{} {:02}:{:02}", label, mins, secs);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Perle"))
        .gauge_style(Style::default().fg(Color::Blue))
        .percent(percent)
        .label(time_str.clone());

    let status = Paragraph::new(time_str).block(Block::default().borders(Borders::ALL));

    frame.render_widget(gauge, chunks[0]);
    frame.render_widget(status, chunks[1]);
}
