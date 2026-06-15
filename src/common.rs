use strum::Display;
use clap::Parser;

/// Launch arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Run without altscreen
    #[arg(short, long)]
    pub inline: bool,
}

/// App state
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    #[default]
    Running,
    Quit,
}

#[derive(PartialEq)]
pub enum UserInput {
    Symbol,
    Operation,
    None,
}

#[derive(PartialEq, Default, Clone, Display)]
pub enum Operation {
    #[default]
    None,
    #[strum(to_string = "+")]
    Addition,
    #[strum(to_string = "-")]
    Subtraction,
    #[strum(to_string = "*")]
    Multiplication,
    #[strum(to_string = "/")]
    Division,
}