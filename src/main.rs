mod config;
mod history;
mod sound;
mod timer;
mod ui;

use crate::timer::{SessionType, Timer};
use std::io::{self, Write};

enum Command {
    Start,
    History,
    Quit,
}

fn parse_command(input: &str) -> Option<Command> {
    let parts = input.trim().to_lowercase();
    if parts.is_empty() {
        return None;
    } // skip empty lines silently
    let mut parts = parts.splitn(2, ' ');
    let cmd = parts.next().unwrap_or("");

    match cmd {
        "s" => Some(Command::Start),
        "h" => Some(Command::History),
        "q" => Some(Command::Quit),
        _ => None,
    }
}

async fn handle_command(cmd: Command) {
    match cmd {
        Command::Start => {
            let mut config = crate::config::load_config();

            // read work duration
            print!(
                "Work duration: {} mins (enter to keep): ",
                config.work_duration_mins
            );
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                config.work_duration_mins =
                    trimmed.parse::<u64>().unwrap_or(config.work_duration_mins);
            }

            // read break duration
            print!(
                "Break duration: {} mins (enter to keep): ",
                config.break_duration_mins
            );
            io::stdout().flush().unwrap();
            let mut input = String::new(); // fresh input variable
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();
            if !trimmed.is_empty() {
                config.break_duration_mins =
                    trimmed.parse::<u64>().unwrap_or(config.break_duration_mins);
            }

            crate::config::save_config(&config);

            print!("Sessions? (default: 4): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let sessions = input.trim().parse::<u32>().unwrap_or(4);

            for i in 1..=sessions {
                println!("\nSession {}/{}", i, sessions);

                // work timer
                let timer = Timer::new(config.work_duration_mins * 60, SessionType::Work);
                timer.run().await.unwrap();

                // break timer - skip on last session
                if i < sessions {
                    let brk = Timer::new(config.break_duration_mins * 60, SessionType::Break);
                    brk.run().await.unwrap();
                }
            }
            println!("Done!");
        }

        Command::History => {
            let sessions = crate::history::load_history();
            if sessions.is_empty() {
                println!("No history yet!");
            } else {
                crate::history::print_stats(&sessions);
            }
        }

        Command::Quit => unreachable!(),
    }
}

#[tokio::main]
async fn main() {
    println!("Welcome to Perle!");
    println!("[s] Start");
    println!("[h] History");
    println!("[q] Quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match parse_command(&input) {
            Some(Command::Quit) => {
                break;
            }
            Some(cmd) => {
                handle_command(cmd).await;
            }
            None => {
                if !input.trim().is_empty() {
                    println!("Unknown command. Try: s(tart), h(istory), q(uit)");
                }
            }
        }
    }
}
