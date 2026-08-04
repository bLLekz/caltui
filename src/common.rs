use strum::Display;
use clap::Parser;

/// Launch arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Run in altscreen
    #[arg(short, long)]
    pub altscreen: bool,
    /// Run with big text
    #[arg(short, long)]
    pub big: bool,
}

// App size
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppSize {
    #[default]
    Default,
    Big,
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