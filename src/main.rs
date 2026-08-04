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

    let mut viewport = Viewport::Inline(27);

    if args.big {
        viewport = Viewport::Inline(67)
    }

    let mut terminal = if args.altscreen {
        ratatui::init()
    } else {
        let options = TerminalOptions {
            viewport: viewport,
        };
        ratatui::try_init_with_options(options)?
    };

    enable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), EnableMouseCapture)?;

    let mut app = CalcApp::new();
    if args.big {
        app.size = common::AppSize::Big;
    }
    if args.altscreen {
        app.altscreen = true;
    }

    let result = app.run(&mut terminal);

    // Cleanup
    terminal.clear()?; 
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), DisableMouseCapture)?;
    ratatui::restore();

    result
}
