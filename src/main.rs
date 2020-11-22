mod terminal_helper;
mod interface;
mod ui;
mod utils;

pub use terminal_helper::TerminalHelper;
use interface::Interface;
pub use interface::Position;
pub use utils::StatefulList;


fn main() -> Result<(), std::io::Error>{
    let mut interface = Interface::default()?;
    interface.run()?;
    TerminalHelper::clear_screen();
    Ok(())
}