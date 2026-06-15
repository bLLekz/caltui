use crate::{app::CalcApp, common::Args};
use clap::Parser;
use color_eyre::eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{TerminalOptions, Viewport};
mod app;
mod calc;
mod common;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let mut terminal = if args.inline {
        let options = TerminalOptions {
            viewport: Viewport::Inline(60),
        };
        ratatui::try_init_with_options(options)?
    } else {
        ratatui::init()
    };

    enable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), EnableMouseCapture)?;

    let app = CalcApp::new();

    let result = app.run(&mut terminal);

    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), DisableMouseCapture)?;
    ratatui::restore();

    result
}
