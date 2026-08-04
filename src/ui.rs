use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{
        Alignment, Constraint,
        Direction::{self},
        Layout, Rect,
    },
    style::{Color, Style, Stylize},
    text::{Line},
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};
use tui_widgets::big_text::{BigText, PixelSize};

use crate::{app::CalcApp, common::AppSize};

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
    match app.size {
        AppSize::Default => {
            render_app_ui(frame, app);
        }
        AppSize::Big => {
            render_app_ui_big(frame, app);
        }
    }
}

// Render App UI
fn render_app_ui(frame: &mut Frame, app: &mut CalcApp) {
    let mut height = [Constraint::Length(27)];
    let mut width = [Constraint::Length(60)];
    if app.altscreen {
        height = [Constraint::Fill(1)];
        width = [Constraint::Fill(1)];
    }

    let layout_vertical = Layout::vertical(height);
    let [content_vertical] = frame.area().layout(&layout_vertical);
    let layout = Layout::horizontal(width);
    let [content] = content_vertical.layout(&layout);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .title("calc")
        .border_style(Style::new().cyan())
        .render(content, frame.buffer_mut());

    let layout = Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
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
    Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().white())
        .render(area, buf);

    let layout =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).margin(1);
    let [total_line, input_line] = area.layout(&layout);

    let total_text = Line::styled(app.total_text.clone(), Color::White);
    let text_input = Line::styled(app.text_input.clone(), Color::White);

    Paragraph::new(total_text)
        .style(Style::new().white())
        .alignment(Alignment::Right)
        .render(total_line, buf);

    Paragraph::new(text_input)
        .style(Style::new().white())
        .alignment(Alignment::Right)
        .render(input_line, buf);
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
            .split(*row);

        for (j, cell) in cols.iter().enumerate() {
            let label = BUTTON_LABELS[i][j];

            block.clone().render(*cell, buf);

            let inner_layout = Layout::vertical([Constraint::Fill(1)]).margin(1);
            let [inner_area] = cell.layout(&inner_layout);

            app.button_areas.push((inner_area, label));

            let inner_area = inner_area.centered(Constraint::Fill(1), Constraint::Fill(1));

            let line = Line::styled(label, Color::White);

            Paragraph::new(line)
                .style(Style::new().white())
                .centered()
                .render(inner_area, buf);
        }
    }
}

/// Render App BIG UI
fn render_app_ui_big(frame: &mut Frame, app: &mut CalcApp) {
    let mut height = [Constraint::Length(150)];
    let mut width = [Constraint::Length(100)];
    if app.altscreen {
        height = [Constraint::Fill(1)];
        width = [Constraint::Fill(1)];
    }

    let layout_vertical = Layout::vertical(height);
    let [content_vertical] = frame.area().layout(&layout_vertical);
    let layout = Layout::horizontal(width);
    let [content] = content_vertical.layout(&layout);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .title("calc")
        .border_style(Style::new().cyan())
        .render(content, frame.buffer_mut());

    let mut layout: Layout = Layout::vertical([
        Constraint::Length(7),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .margin(1);

    if app.altscreen {
        layout = Layout::vertical([
            Constraint::Percentage(17),
            Constraint::Percentage(83),
            Constraint::Length(1),
        ])
        .margin(1);
    }
    let [display, buttons, help] = content.layout(&layout);

    render_display_big(frame.buffer_mut(), display, app);
    render_buttons_big(frame.buffer_mut(), buttons, app);
    render_help(frame.buffer_mut(), help);
}

/// Render display zone
fn render_display_big(buf: &mut Buffer, area: Rect, app: &mut CalcApp) {
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
fn render_buttons_big(buf: &mut Buffer, area: Rect, app: &mut CalcApp) {
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

            let inner_area = inner_area.centered(Constraint::Length(25), Constraint::Length(5));

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
