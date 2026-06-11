use color_eyre::eyre::Result;
// use ratatui::{TerminalOptions, Viewport};
use crate::app::CalcApp;
mod app;
mod common;
mod ui;
mod calc;

fn main() -> Result<()> {

    color_eyre::install()?;
    let mut terminal = ratatui::init();

    // let options = TerminalOptions {
    //     viewport: Viewport::Inline(40),
    // };
    // let mut terminal = ratatui::try_init_with_options(options)?;

    crossterm::execute!(
        terminal.backend_mut(),
        // crossterm::event::EnableMouseCapture
    )?;

    let app = CalcApp::new();

    let result = app.run(&mut terminal);

    // Cleanup
    crossterm::execute!(terminal.backend_mut(), 
    //crossterm::event::DisableMouseCapture
    )?;
    ratatui::restore();
    // print!("\x1b[2J\x1b[1;1H");

    result
}