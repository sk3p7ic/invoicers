use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Row, Table, Widget},
    Frame,
};

use crate::data::{address::AddressKind, application::App};

pub fn ui(f: &mut Frame, app: &App) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(app.name.as_str())
        .title_alignment(Alignment::Center);
    f.render_widget(title_block, f.size());

    let form_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(f.size());

    let info_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ],
    )
    .split(form_layout[0]);
    let table_layout =
        Layout::new(Direction::Vertical, [Constraint::Percentage(100)]).split(form_layout[1]);

    render_invoice_name(app, info_layout[0], f);
    render_address(app, AddressKind::Contractor, info_layout[1], f);
    render_address(app, AddressKind::Client, info_layout[2], f);
    render_hourly_table(app, table_layout[0], f);
}

fn render_invoice_name(app: &App, parent: Rect, f: &mut Frame) {
    let name = Paragraph::new(app.name.as_str())
        .style(get_style(0, app))
        .block(Block::bordered().title("Invoice Name / Number"));
    f.render_widget(name, parent);
}

fn render_address(app: &App, kind: AddressKind, parent: Rect, f: &mut Frame) {
    let address_block = Block::bordered().title(kind.as_str());
    f.render_widget(address_block, parent);

    let idx_offset = match kind {
        AddressKind::Client => 5,
        AddressKind::Contractor => 0,
    };

    let address_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ],
    )
    .margin(2)
    .split(parent);

    let name = Paragraph::new(app.get_addr_from_kind(kind).name.as_str())
        .style(get_style(1 + idx_offset, app))
        .block(Block::bordered().title("Name"));
    f.render_widget(name, address_layout[0]);

    let street_num = Paragraph::new(app.get_addr_from_kind(kind).street_num.as_str())
        .style(get_style(2 + idx_offset, app))
        .block(Block::bordered().title("Street Number"));
    let city = Paragraph::new(app.get_addr_from_kind(kind).city.as_str())
        .style(get_style(3 + idx_offset, app))
        .block(Block::bordered().title("City"));
    let state = Paragraph::new(app.get_addr_from_kind(kind).state.as_str())
        .style(get_style(4 + idx_offset, app))
        .block(Block::bordered().title("State"));
    let zip = Paragraph::new(app.get_addr_from_kind(kind).zip.as_str())
        .style(get_style(5 + idx_offset, app))
        .block(Block::bordered().title("Zip"));

    f.render_widget(street_num, address_layout[1]);

    let inner_address_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Fill(2),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ],
    )
    .split(address_layout[2]);

    f.render_widget(city, inner_address_layout[0]);
    f.render_widget(state, inner_address_layout[1]);
    f.render_widget(zip, inner_address_layout[2]);
}

fn render_hourly_table(app: &App, parent: Rect, f: &mut Frame) {
    let hours_block = Block::bordered().title("Hours Worked");
    f.render_widget(hours_block, parent);

    let hours_layout = Layout::new(Direction::Vertical, [Constraint::Fill(1)])
        .margin(1)
        .split(parent);

    let hours_table = Table::new(
        app.hours.to_rows(),
        [
            Constraint::Fill(1),
            Constraint::Length(7),
            Constraint::Length(7),
            Constraint::Length(7),
        ],
    )
    .header(Row::new(vec!["Description", "Rate", "Hours", "Total"]));

    f.render_widget(hours_table, hours_layout[0])
}

fn get_style(idx: usize, app: &App) -> Style {
    Style::new().fg(if idx == app.selected_field {
        match app.editing {
            true => Color::Cyan,
            false => Color::Blue,
        }
    } else {
        Color::White
    })
}
