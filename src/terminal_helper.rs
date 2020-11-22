use crate::Position;
use std::io::{self, stdout, Write};
// use termion::event::Key;
// use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{
    event::Key,
    input::TermRead,
};

// pub struct Size {
//     pub width: u16,
//     pub height: u16,
// }

pub struct TerminalHelper {
    // pub size: Size,
    pub _stdout: RawTerminal<std::io::Stdout>,
}

impl TerminalHelper {
    pub fn default() -> Result<Self, std::io::Error> {
        // let size = termion::terminal_size()?;
        Ok(Self{
            // size: Size {
            //     width: size.0,
            //     height: size.1,
            // },
            _stdout: stdout().into_raw_mode()?,
        })
    }
    // pub fn size(&self) -> &Size {
    //     &self.size
    // }
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }
    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next(){
                return key;
            }
        }
    }
    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
    pub fn cusrsor_position(position: &Position){
        let Position{mut x, mut y} = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        let x = x as u16;
        let y = y as u16;
        print!("{}", termion::cursor::Goto(x, y));
    }
}