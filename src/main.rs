use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
// use ratatui::{TerminalOptions, Viewport};
use crate::app::CalcApp;
mod app;
mod calc;
mod common;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    // let options = TerminalOptions {
    //     viewport: Viewport::Inline(40),
    // };
    // let mut terminal = ratatui::try_init_with_options(options)?;
    enable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let app = CalcApp::new();

    let result = app.run(&mut terminal);

    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    ratatui::restore();
    // print!("\x1b[2J\x1b[1;1H");

    result
}
