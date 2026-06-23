use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{
        Alignment, Constraint,
        Direction::{self},
        Layout, Rect,
    },
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};
use tui_widgets::big_text::{BigText, PixelSize};

use crate::app::CalcApp;

pub const MIN_SCREEN_WIDTH: u16 = 80;
pub const MIN_SCREEN_HEIGHT: u16 = 56;

pub const BUTTON_LABELS: &[&[&str]] = &[
    &["%", "CE", "C", "Del"],
    &["1/x", "sqr", "sqrt", "/"],
    &["7", "8", "9", "*"],
    &["4", "5", "6", "-"],
    &["1", "2", "3", "+"],
    &["+/-", "0", ".", "="],
];

/// Render UI
pub fn render_ui(app: &mut CalcApp, frame: &mut Frame) {
    if frame.area().width < MIN_SCREEN_WIDTH || frame.area().height < MIN_SCREEN_HEIGHT {
        render_wrong_size_message(frame);
    } else {
        render_app_ui(frame, app);
    }
}

/// Render message with message about small app size
fn render_wrong_size_message(frame: &mut Frame) {
    Paragraph::new(Text::from(vec![
        "".into(),
        "Terminal size too small!".into(),
        format!(
            "Current size: {}x{}",
            frame.area().width,
            frame.area().height
        )
        .into(),
        format!(
            "Minimum required: {}x{}",
            MIN_SCREEN_WIDTH, MIN_SCREEN_HEIGHT
        )
        .into(),
    ]))
    .alignment(Alignment::Center)
    .block(Block::bordered().title("Small size"))
    .render(frame.area(), frame.buffer_mut());
}

/// Render App UI
fn render_app_ui(frame: &mut Frame, app: &mut CalcApp) {
    let layout = Layout::horizontal([Constraint::Length(100)]);
    let [content] = frame.area().layout(&layout);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .title("calc")
        .border_style(Style::new().cyan())
        .render(content, frame.buffer_mut());

    let layout = Layout::vertical([
        Constraint::Percentage(17),
        Constraint::Percentage(83),
        Constraint::Length(1),
    ])
    .margin(1);
    let [display, buttons, help] = content.layout(&layout);

    render_display(frame.buffer_mut(), display, app);
    render_buttons(frame.buffer_mut(), buttons, app);
    render_help(frame.buffer_mut(), help);
}

/// Render display zone
fn render_display(buf: &mut Buffer, area: Rect, app: &mut CalcApp) {
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().white());

    let total_text = Line::styled(app.total_text.clone(), Color::White);
    let text_input = Line::styled(app.text_input.clone(), Color::White);

    BigText::builder()
        .pixel_size(PixelSize::Octant)
        .style(Style::new().white())
        .alignment(Alignment::Right)
        .block(block)
        .lines(vec![total_text, text_input])
        .build()
        .render(area, buf);
}

/// Render buttons zone
fn render_buttons(buf: &mut Buffer, area: Rect, app: &mut CalcApp) {
    app.button_areas.clear();

    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().white())
        .render(area, buf);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1); 6])
        .margin(1)
        .split(area);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().white());

    for (i, row) in rows.iter().enumerate() {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1); 4])
            .spacing(1)
            .split(*row);

        for (j, cell) in cols.iter().enumerate() {
            let label = BUTTON_LABELS[i][j];

            block.clone().render(*cell, buf);

            let inner_layout = Layout::vertical([Constraint::Fill(1)]).margin(1);
            let [inner_area] = cell.layout(&inner_layout);

            app.button_areas.push((inner_area, label));

            let inner_area = inner_area.centered(Constraint::Length(25), Constraint::Length(4));

            let line = Line::styled(label, Color::White);

            BigText::builder()
                .pixel_size(PixelSize::Sextant)
                .style(Style::new().white())
                .centered()
                .lines([line])
                .build()
                .render(inner_area, buf);
        }
    }
}

/// Render help zone
fn render_help(buf: &mut Buffer, area: Rect) {
    Paragraph::new("c - clear input, esc/C - clear all, Q - quit")
        .alignment(Alignment::Right)
        .fg(Color::Rgb(150, 150, 150))
        .wrap(Wrap { trim: true })
        .render(area, buf);
}
