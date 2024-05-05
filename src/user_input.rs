use crossterm::event;
use std::io::{self, Stdout};
use std::time::Duration;

pub fn user_input(_stdout: &mut Stdout) -> io::Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            event::Event::Key(keyevent) => {
                if keyevent
                    == event::KeyEvent::new(event::KeyCode::Char('c'), event::KeyModifiers::CONTROL)
                    || keyevent
                        == event::KeyEvent::new(event::KeyCode::Esc, event::KeyModifiers::NONE)
                    || keyevent
                        == event::KeyEvent::new(
                            event::KeyCode::Char('Q'),
                            event::KeyModifiers::NONE,
                        )
                    || keyevent
                        == event::KeyEvent::new(
                            event::KeyCode::Char('q'),
                            event::KeyModifiers::NONE,
                        )
                {
                    return Ok(false);
                }
            }
            event::Event::Resize(_w, _h) => {
                // clear(stdout)?;
            }
            _ => {}
        }
    }
    Ok(true)
}
