use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: u32,
    pub session_type: String, // "Work" or "Break"
    pub duration_mins: u64,
    pub completed: bool,
}

pub fn save_session(mut session: Session) {
    let mut history = load_history();
    session.id = history.len() as u32 + 1;
    history.push(session);
    let json = serde_json::to_string_pretty(&history).unwrap();
    fs::write("history.json", json).unwrap();
}

pub fn load_history() -> Vec<Session> {
    match fs::read_to_string("history.json") {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub fn print_stats(sessions: &Vec<Session>) {
    let total = sessions.len();
    let completed_count = sessions.iter().filter(|s| s.completed).count();
    let total_focus_time: u64 = sessions
        .iter()
        .filter(|s| s.session_type == "Work")
        .map(|s| s.duration_mins)
        .sum();
    println!("Total sessions: {}", total);
    println!("Completed: {}", completed_count);
    println!("Total focus time: {}", total_focus_time);
}
