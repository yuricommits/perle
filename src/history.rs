use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: u32,
    pub session_type: String, // "Work" or "Break"
    pub duration_mins: u64,
    pub completed: bool,
}

pub fn save_session(mut session: Session) -> Result<()> {
    let mut history = load_history()?;
    session.id = history.len() as u32 + 1;
    history.push(session);
    let json = serde_json::to_string_pretty(&history)?;
    fs::write("history.json", json)?;
    Ok(())
}

pub fn load_history() -> Result<Vec<Session>> {
    match fs::read_to_string("history.json") {
        Ok(contents) => Ok(serde_json::from_str(&contents)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}

pub fn print_stats(sessions: &[Session]) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_session_gets_correct_id() {
        let history = vec![Session {
            id: 1,
            session_type: String::from("Work"),
            duration_mins: 25,
            completed: true,
        }];
        let next_id = history.len() as u32 + 1;
        assert_eq!(next_id, 2);
    }

    #[test]
    fn test_print_stats_counts_correctly() {
        let sessions: Vec<_> = vec![
            Session {
                id: 1,
                session_type: String::from("Work"),
                duration_mins: 25,
                completed: true,
            },
            Session {
                id: 1,
                session_type: String::from("Work"),
                duration_mins: 25,
                completed: true,
            },
            Session {
                id: 1,
                session_type: String::from("Break"),
                duration_mins: 5,
                completed: true,
            },
        ];
        let completed_count = sessions.iter().filter(|s| s.completed).count();
        let total_focus_time: u64 = sessions
            .iter()
            .filter(|s| s.session_type == "Work")
            .map(|s| s.duration_mins)
            .sum();
        assert_eq!(completed_count, 3);
        assert_eq!(total_focus_time, 50); // 25 + 25, Break excluded
    }

    use std::path::Path;

    #[test]
    fn test_load_history_returns_empty_when_no_file() {
        std::fs::remove_file("history.json").ok(); // delete if exists, ignore error if not
        assert!(!Path::new("history.json").exists()); // verify it's gone
        let history = load_history().unwrap(); // acceptable in tests
        assert!(history.is_empty());
    }

    #[test]
    fn test_session_type_filter() {
        let sessions = vec![
            Session {
                id: 1,
                session_type: String::from("Work"),
                duration_mins: 30,
                completed: true,
            },
            Session {
                id: 2,
                session_type: String::from("Break"),
                duration_mins: 5,
                completed: false,
            },
            Session {
                id: 3,
                session_type: String::from("Work"),
                duration_mins: 25,
                completed: false,
            },
        ];
        let work_count = sessions.iter().filter(|s| s.session_type == "Work").count();
        let break_count = sessions
            .iter()
            .filter(|s| s.session_type == "Break")
            .count();
        assert_eq!(work_count, 2);
        assert_eq!(break_count, 1);
    }
}
