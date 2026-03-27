use crate::ui::draw;
use std::io::stdout;
use tokio::time::{Duration, sleep};

use tokio::signal;

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

    async fn countdown(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        label: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut remaining = self.duration_secs;
        loop {
            terminal.draw(|frame| draw(frame, label, remaining, self.duration_secs))?;
            sleep(Duration::from_secs(1)).await;
            if remaining == 0 {
                break;
            }
            remaining -= 1;
        }
        Ok(())
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let label = match self.session_type {
            SessionType::Work => "Work",
            SessionType::Break => "Break",
        };
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        tokio::select! {
            r = self.countdown(&mut terminal, label) => { r?; }
            _ = signal::ctrl_c() => {
                disable_raw_mode()?;
                execute!(stdout(), LeaveAlternateScreen)?;
                println!("\nInterrupted!");
                std::process::exit(0);
            }
        }
        if let Err(e) = crate::history::save_session(crate::history::Session {
            id: 0,
            session_type: String::from(label),
            duration_mins: self.duration_secs.div_ceil(60),
            completed: true,
        }) {
            eprintln!("Failed to save session: {e}");
        }
        if let Err(e) = crate::sound::play_notification() {
            eprintln!("Sound error: {e}");
        }
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;
        Ok(())
    }
}
