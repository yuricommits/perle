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
            println!("Sessions? (default: 4): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let sessions = input.trim().parse::<u32>().unwrap_or(4);

            for i in 1..=sessions {
                println!("\nSession {}/{}", i, sessions);

                // work timer
                let timer = Timer::new(5, SessionType::Work);
                timer.run().await.unwrap();

                // break timer - skip on last session
                if i < sessions {
                    let brk = Timer::new(3, SessionType::Break);
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
            None => println!("Unknown command. Try: s(tart), h(istory), q(uit)"),
        }
    }
}
