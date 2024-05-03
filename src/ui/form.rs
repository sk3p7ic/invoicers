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
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Fill(2),
        ])
        .margin(1)
        .split(f.size());

    render_invoice_name(app, form_layout[0], f);
    render_address(app, AddressKind::Contractor, form_layout[1], f);
    render_address(app, AddressKind::Client, form_layout[2], f);
    render_hourly_table(app, form_layout[3], f);
}

fn render_invoice_name(app: &App, parent: Rect, f: &mut Frame) {
    let name =
        Paragraph::new(app.name.as_str()).block(Block::bordered().title("Invoice Name / Number"));
    f.render_widget(name, parent);
}

fn render_address(app: &App, kind: AddressKind, parent: Rect, f: &mut Frame) {
    let address_block = Block::bordered().title(kind.as_str());
    f.render_widget(address_block, parent);

    let address_layout = Layout::new(
        Direction::Vertical,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .margin(2)
    .split(parent);

    let name = Paragraph::new(app.get_addr_from_kind(kind).name.as_str())
        .block(Block::bordered().title("Name"));

    let inner_address_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(50),
            Constraint::Fill(2),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ],
    )
    .split(address_layout[1]);

    let street_num = Paragraph::new(app.get_addr_from_kind(kind).street_num.as_str())
        .block(Block::bordered().title("Street Number"));
    let city = Paragraph::new(app.get_addr_from_kind(kind).city.as_str())
        .block(Block::bordered().title("City"));
    let state = Paragraph::new(app.get_addr_from_kind(kind).state.as_str())
        .block(Block::bordered().title("State"));
    let zip = Paragraph::new(app.get_addr_from_kind(kind).zip.as_str())
        .block(Block::bordered().title("Zip"));

    f.render_widget(street_num, inner_address_layout[0]);
    f.render_widget(city, inner_address_layout[1]);
    f.render_widget(state, inner_address_layout[2]);
    f.render_widget(zip, inner_address_layout[3]);

    f.render_widget(name, address_layout[0]);
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