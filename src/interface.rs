use crate::TerminalHelper;
use crate::ui;

use tui::{
    *,
    backend::TermionBackend,
};


use termion::{
    // raw::IntoRawMode,
    // clear,
    event::Key,
};

// const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Interface {
    should_quit: bool,
    cursor_position: Position,
}

impl Interface {
    pub fn run(&mut self) -> Result<(), std::io::Error>{
        let terminal = TerminalHelper::default()?;
        TerminalHelper::clear_screen();
        let backend = TermionBackend::new(terminal._stdout);
        let mut terminal = Terminal::new(backend)?;
        loop {
            TerminalHelper::cusrsor_position(&self.cursor_position);
            if self.should_quit {
                break;
            }
            
            terminal.draw(|f| ui::draw(f))?;
            
           let processed_key = TerminalHelper::read_key()?;
            match processed_key {
                Key::Ctrl('q') => self.should_quit = true,
                _ => (),
            }
            
        }
        TerminalHelper::clear_screen();
        Ok(())
    }

    pub fn default() -> Result<Self, std::io::Error> {
        Ok(Self {
            should_quit: false,
            cursor_position: Position {x: 0, y: 0},
            // events: Events::with_config(Config {
            //     tick_rate: Duration::from_millis(250),
            //     ..Config::default()
            // }),
        })
    }
}