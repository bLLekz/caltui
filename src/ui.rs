use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{
        Alignment, Constraint,
        Direction::{self},
        Layout, Rect,
    },
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};
use tui_big_text::{BigText, PixelSize};

use crate::app::CalcApp;

/// Render App UI
pub fn render_ui(app: &mut CalcApp, frame: &mut Frame) {
    let layout = Layout::horizontal([Constraint::Length(100)]);
    let [content] = frame.area().layout(&layout);

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .title("calc")
        .border_style(Style::new().cyan());

    Paragraph::new("display")
        .alignment(Alignment::Right)
        .fg(Color::Rgb(150, 150, 150))
        .block(block)
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

    let button_labels = vec![
        vec!["%", "CE", "C", "Del"],
        vec!["1/x", "sqr", "sqrt", "/"],
        vec!["7", "8", "9", "*"],
        vec!["4", "5", "6", "-"],
        vec!["1", "2", "3", "+"],
        vec!["+/-", "0", ".", "="],
    ];

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
            let label = button_labels[i][j];

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
