mod user_input;

// standard crates
use std::io::{self, Write, stdout};

// non-standard crates
use crossterm::{cursor, execute, queue, style, terminal};

// modules
use user_input::user_input;

fn main() -> io::Result<()> {
    let mut stdout = stdout();
    let mut is_running = true;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    while is_running {
        is_running = user_input(&mut stdout)?;

        queue!(
            stdout,
            cursor::MoveTo(0, 0),
            style::SetForegroundColor(style::Color::Cyan),
            style::Print("russshhht"),
        )?;

        stdout.flush()?;
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
