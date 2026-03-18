# Perle

A beautiful, minimal Pomodoro timer for the terminal — built in Rust.

```
───────────────────── Perle ──────────────────
                                               
                  Work Session                       
                                               
         ██████  ██████ ▪ ██████  ██████       
         █    █  █    █   █    █  █    █       
         █    █  █    █   █    █  █    █       
         █    █  █    █   █    █  █    █       
         ██████  ██████   ██████  ██████       
                                               
  Progress ──────────────────────────────────  
  ████████████████████░░░░░░░░░░░░  60%        
                                               
──────────────────────────────────────────────
```

## Features

- **ASCII clock** — large, readable countdown display
- **Work & break sessions** — automatic cycling with configurable durations
- **Progress bar** — visual TUI powered by ratatui
- **Sound notification** — audio alert when each session ends
- **Session history** — persistent JSON storage with stats
- **Custom configuration** — set your own work/break durations, saved between runs
- **Graceful Ctrl+C** — clean terminal exit, no broken state
- **Proper error handling** — no panicking `unwrap()` in production code

## Installation

### Prerequisites

- Rust (stable) — install via [rustup.rs](https://rustup.rs)
- Linux: `sudo apt install pkg-config libasound2-dev`

### Build from source

```bash
git clone https://github.com/yuricommits/perle
cd perle
cargo build --release
./target/release/perle
```

## Usage

```
Welcome to Perle!
[s] Start
[h] History
[q] Quit
> 
```

### Starting a session

```
> s
Work duration: 25 mins (enter to keep): 
Break duration: 5 mins (enter to keep): 
Sessions? (default: 4): 2

Session 1/2
[ASCII clock counts down]
[Sound plays]

Session 2/2
[ASCII clock counts down]
[Sound plays]

Done!
```

### Viewing history & stats

```
> h
Total sessions: 8
Completed: 8
Total focus time: 200 mins
```

### Keyboard shortcuts

| Key | Action |
|-----|--------|
| `s` | Start a new session |
| `h` | View history & stats |
| `q` | Quit |
| `Ctrl+C` | Exit cleanly during a timer |

## Configuration

On first run, Perle creates `config.json` with defaults:

```json
{
  "work_duration_mins": 25,
  "break_duration_mins": 5
}
```

You can change these each time you start a session — changes are saved automatically.

## Project Structure

```
src/
├── main.rs       — menu loop, command parsing
├── timer.rs      — async countdown, TUI integration
├── ui.rs         — ratatui ASCII clock & progress bar
├── sound.rs      — audio notification
├── history.rs    — session persistence & stats
├── config.rs     — user configuration
└── error.rs      — custom error types
assets/
└── beep.mp3      — notification sound
```

## Tech Stack

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime |
| `ratatui` | Terminal UI |
| `crossterm` | Terminal control |
| `rodio` | Audio playback |
| `serde` + `serde_json` | JSON serialization |
| `thiserror` | Custom error types |

## License

MIT - [LICENSE](LICENSE)
