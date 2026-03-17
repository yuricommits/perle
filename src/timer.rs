use crate::ui::draw;
use std::io::{self, Write, stdout};
use tokio::time::{Duration, sleep};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

pub struct Timer {
    pub duration_secs: u64,
    pub session_type: SessionType,
}

pub enum SessionType {
    Work,
    Break,
}

impl Timer {
    pub fn new(duration_secs: u64, session_type: SessionType) -> Timer {
        Timer {
            duration_secs,
            session_type,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut remaining = self.duration_secs;
        let label = match self.session_type {
            SessionType::Work => "Work",
            SessionType::Break => "Break",
        };
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        loop {
            terminal.draw(|frame| draw(frame, label, remaining, self.duration_secs))?;
            io::stdout().flush().unwrap();
            sleep(Duration::from_secs(1)).await;
            if remaining == 0 {
                break;
            }
            remaining -= 1;
        }
        crate::history::save_session(crate::history::Session {
            id: 0, // will be set by save_session based on history length
            session_type: String::from(label),
            duration_mins: self.duration_secs / 60,
            completed: true,
        });
        crate::sound::play_notification();
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
}
