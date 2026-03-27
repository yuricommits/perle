use std::vec;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
};

fn digit_to_lines(d: u8) -> [&'static str; 5] {
    match d {
        0 => [" ██ ", "█  █", "█  █", "█  █", " ██ "],
        1 => ["  █ ", " ██ ", "  █ ", "  █ ", " ███"],
        2 => [" ██ ", "   █", " ██ ", "█   ", " ███"],
        3 => [" ██ ", "   █", " ██ ", "   █", " ██ "],
        4 => ["█  █", "█  █", " ███", "   █", "   █"],
        5 => [" ███", "█   ", " ██ ", "   █", " ██ "],
        6 => [" ██ ", "█   ", " ███", "█  █", " ██ "],
        7 => [" ███", "   █", "  █ ", " █  ", " █  "],
        8 => [" ██ ", "█  █", " ██ ", "█  █", " ██ "],
        9 => [" ██ ", "█  █", " ███", "   █", " ██ "],

        _ => ["    ", "  █ ", "    ", "  █ ", "    "],
    }
}

fn time_to_ascii(mins: u64, secs: u64) -> Vec<String> {
    let m1 = (mins / 10) as u8;
    let m2 = (mins % 10) as u8;
    let s1 = (secs / 10) as u8;
    let s2 = (secs % 10) as u8;

    let digits = [
        digit_to_lines(m1),
        digit_to_lines(m2),
        digit_to_lines(s1),
        digit_to_lines(s2),
    ];

    let mut rows = vec![String::new(); 5];
    for row in 0..5 {
        let separator = if row == 1 || row == 3 { " █ " } else { "   " };
        rows[row] = format!(
            "{}  {}{}{}  {}",
            digits[0][row], digits[1][row], separator, digits[2][row], digits[3][row]
        );
    }
    rows
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn draw(frame: &mut Frame, label: &str, remaining: u64, total: u64) {
    let color = if label == "Work" {
        Color::White
    } else {
        Color::Gray
    };

    let title = Line::from(vec![Span::styled(
        format!("{} Session", label),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )]);

    let paragraph = Paragraph::new(title).alignment(Alignment::Center);

    let percent = ((total - remaining) as f64 / total as f64 * 100.0) as u16;

    let centered = centered_rect(50, 70, frame.area());

    let outer = Block::default()
        .borders(Borders::ALL)
        .title(" Perle ")
        .title_alignment(Alignment::Center);

    let inner = outer.inner(centered);
    frame.render_widget(outer, centered);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // top padding — flexible
            Constraint::Length(2), // title
            Constraint::Length(8), // clock — more room
            Constraint::Length(2), // gauge — border adds height
            Constraint::Min(0),    // bottom padding — flexible
        ])
        .split(inner);

    let gauge_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(5),  // left padding
            Constraint::Percentage(90), // gauge
            Constraint::Percentage(5),  // right padding
        ])
        .split(chunks[3]);

    let mins = remaining / 60;
    let secs = remaining % 60;

    let gauge = Gauge::default()
        .block(Block::default().title(" Progress "))
        .gauge_style(Style::default().fg(Color::White))
        .percent(percent);

    let clock_lines: Vec<Line> = time_to_ascii(mins, secs)
        .into_iter()
        .map(|row| Line::from(Span::styled(row, Style::default().fg(color))))
        .collect();

    let clock = Paragraph::new(clock_lines).alignment(Alignment::Center);

    frame.render_widget(paragraph, chunks[1]); // title
    frame.render_widget(clock, chunks[2]); // clock
    frame.render_widget(gauge, gauge_chunks[1]); // gauge
}
