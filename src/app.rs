use color_eyre::eyre::{Ok, Result};
use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
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
    pub button_areas: Vec<(Rect, &'static str)>,
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
            button_areas: vec![],
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
                    self.handle_mouse(mouse.column, mouse.row);
                }
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
            if self.first_part.len() > 0
                && (self.text_input.len() > 0 && self.text_input != 0.to_string())
                && self.second_part.len() == 0
                && self.operation != Operation::None
            {
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

            self.total_text = format!(
                "{} {} {} =",
                self.first_part.clone(),
                self.operation.to_string(),
                self.second_part.clone()
            );

            self.text_input = Calc::calculate(
                self.first_part.clone(),
                self.second_part.clone(),
                self.operation.clone(),
            );

            self.input_cursor_position = self.text_input.clone().len().try_into().unwrap();
        }
    }

    /// Calc 1% of x
    fn calc_procent(&mut self) {
        if self.text_input != "0" && self.text_input.len() > 0 {
            self.total_text = format!("1 % of {}", self.text_input);
            self.text_input = Calc::calc_procent(self.text_input.clone());
        }
    }

    /// Calc 1/x
    fn calc_one_divide_x(&mut self) {
        if self.text_input != "0" && self.text_input.len() > 0 {
            self.total_text = format!("1/({})", self.text_input);
            self.text_input = Calc::calc_one_divide_x(self.text_input.clone());
        }
    }

    /// Calc x²
    fn calc_x_sqr(&mut self) {
        if self.text_input != "0" && self.text_input.len() > 0 {
            self.total_text = format!("sqr({})", self.text_input);
            self.text_input = Calc::calc_sqr(self.text_input.clone());
        }
    }

    /// Calc ²√x
    fn calc_sqrt(&mut self) {
        if self.text_input != "0" && self.text_input.len() > 0 {
            self.total_text = format!("sqrt({})", self.text_input);
            self.text_input = Calc::calc_sqrt(self.text_input.clone());
        }
    }

    fn switch_plus_minus(&mut self) {
        if self.text_input.starts_with('-') {
            self.text_input = self.text_input.trim_start_matches('-').to_string();
        } else if self.text_input != "0" {
            self.text_input = format!("-{}", self.text_input);
        }
    }

    fn handle_mouse(&mut self, column: u16, row: u16) {
        for (rect, label) in &self.button_areas {
            if column >= rect.x
                && column < rect.x + rect.width
                && row >= rect.y
                && row < rect.y + rect.height
            {
                self.on_button_press(label);
                break;
            }
        }
    }

    fn on_button_press(&mut self, label: &str) {
        match label {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." => {
                for ch in label.chars() {
                    self.typing_action(ch);
                }
            }
            "+" | "-" | "*" | "/" => {
                let ch = label.chars().next().unwrap();
                self.typing_action(ch);
            }
            "=" => self.do_calc(),
            "C" => self.reset(),
            "CE" => self.clear_input(),
            "Del" => self.backspace_action(),
            "+/-" => self.switch_plus_minus(),
            "%" => self.calc_procent(),
            "1/x" => self.calc_one_divide_x(),
            "sqr" => self.calc_x_sqr(),
            "sqrt" => self.calc_sqrt(),
            _ => {}
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
