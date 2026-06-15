use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{DefaultTerminal, layout::Rect};

use crate::{
    calc::Calc,
    common::{
        AppState, Operation,
        UserInput::{self},
    },
    ui::render_ui,
};

/// Main app
#[derive(Default, Clone)]
pub struct CalcApp {
    pub state: AppState,
    pub text_input: String,
    pub input_cursor_position: i32,
    pub operation: Operation,
    pub first_part: String,
    pub second_part: String,
    pub total_text: String,
}

impl CalcApp {
    pub fn new() -> Self {
        Self {
            state: AppState::default(),
            text_input: String::from("0"),
            input_cursor_position: 1,
            operation: Operation::None,
            first_part: String::new(),
            second_part: String::new(),
            total_text: String::new(),
        }
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while self.is_running() {
            terminal.draw(|frame| render_ui(&mut self, frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    // Keybinds
    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) => match key.code {
                KeyCode::Char('Q') => self.quit(),
                KeyCode::Char('C') => self.reset(),
                KeyCode::Char('c') => self.clear_input(),
                KeyCode::Char(c) => self.typing_action(c),
                KeyCode::Backspace => self.backspace_action(),
                KeyCode::Delete => self.delete_action(),
                KeyCode::Enter => self.do_calc(),
                KeyCode::Esc => self.reset(),
                _ => (),
            },
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::Up(MouseButton::Left) => {
                    // self.text_input = "1".into();
                    // self.input_cursor_position += 1;
                },
                _ => (),
            },
            _ => (),
        };
        Ok(())
    }

    fn typing_action(&mut self, c: char) {
        if self.text_input == "Cannot divide by zero!" {
            self.reset();
        }

        let char_type = match c {
            '0'..='9' | '.' => UserInput::Symbol,
            '+' | '-' | '*' | '/' => UserInput::Operation,
            _ => UserInput::None,
        };

        if self.text_input == "0" && c != '.' && char_type == UserInput::Symbol {
            self.text_input = c.to_string();
            return;
        }

        if self.text_input.contains(".") && c == '.' {
            return;
        }

        if char_type == UserInput::Symbol {
            if self.first_part.len() != 0 && self.text_input.len() == 0 {
                self.text_input = "".into();
                self.input_cursor_position = 0;
            }

            self.text_input += &c.to_string();
            self.input_cursor_position += 1;
        }

        if char_type == UserInput::Operation {
            if self.first_part.len() > 0 && (self.text_input.len() > 0 && self.text_input != 0.to_string()) && self.second_part.len() == 0 && self.operation != Operation::None {
                self.first_part = Calc::calculate(
                    self.first_part.clone(),
                    self.text_input.clone(),
                    self.operation.clone(),
                );
                self.default_text();
            }

            self.operation = match c {
                '+' => Operation::Addition,
                '-' => Operation::Subtraction,
                '*' => Operation::Multiplication,
                '/' => Operation::Division,
                _ => Operation::None,
            };

            if self.first_part.len() == 0 && self.text_input.len() > 0 {
                self.first_part = self.text_input.clone();
                let first_part = self.first_part.clone();
                let operation = self.operation.to_string();
                self.total_text = format!("{first_part} {operation}");
                self.default_text();
            }

            if self.first_part.len() > 0 && self.text_input.len() > 0 && self.second_part.len() == 0
            {
                let first_part = self.first_part.clone();
                let operation = self.operation.to_string();
                self.total_text = format!("{first_part} {operation}");
                self.default_text();
            }

            if self.first_part.len() > 0 && self.second_part.len() > 0 && self.text_input.len() > 0
            {
                self.first_part = self.text_input.clone();
                self.second_part = "".into();
                let first_part = self.first_part.clone();
                let operation = self.operation.to_string();
                self.total_text = format!("{first_part} {operation}");
                self.default_text();
            }
        }
    }

    fn backspace_action(&mut self) {
        if self.text_input == "0" {
            return;
        }

        if self.text_input.len() > 0 && self.input_cursor_position > 0 {
            self.input_cursor_position -= 1;
            self.text_input = self
                .text_input
                .chars()
                .take(self.input_cursor_position.try_into().unwrap())
                .collect();
        }

        if self.text_input == "-" {
            self.default_text();
        }

        if self.text_input.len() == 0 && self.input_cursor_position == 0 {
            self.default_text();
        }
    }
    fn delete_action(&mut self) {
        if self.input_cursor_position >= 0
            && self.text_input.len() > 0
            && self.input_cursor_position < self.text_input.len().try_into().unwrap()
        {}
    }

    fn do_calc(&mut self) {
        if self.text_input.len() != 0
            && self.text_input != "Cannot divide by zero!"
            && self.first_part.len() != 0
        {
            if self.second_part.len() == 0 {
                self.second_part = self.text_input.clone();
            } else {
                self.first_part = self.text_input.clone();
            }

            let first_part = self.first_part.clone();
            let operation = self.operation.to_string();
            let second_part = self.second_part.clone();
            self.total_text = format!("{first_part} {operation} {second_part} =");
            self.text_input = Calc::calculate(
                self.first_part.clone(),
                self.second_part.clone(),
                self.operation.clone(),
            );
            self.input_cursor_position = self.text_input.clone().len().try_into().unwrap();
        }
    }

    fn default_text(&mut self) {
        self.text_input = "0".into();
        self.input_cursor_position = 1;
    }

    fn clear_input(&mut self) {
        self.default_text();
    }
    
    fn reset(&mut self) {
        self.default_text();
        self.first_part = "".into();
        self.operation = Operation::None;
        self.second_part = "".into();
        self.total_text = "".into();
    }

    fn quit(&mut self) {
        self.reset();
        self.state = AppState::Quit;
    }
}
