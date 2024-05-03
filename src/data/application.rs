use std::borrow::BorrowMut;

use chrono::Datelike;
use crossterm::event::KeyCode;

use super::{
    address::{Address, AddressKind},
    table::HourlyTable,
};

const N_FIELDS: usize = 11;

pub struct App {
    pub name: String,
    pub addr_client: Address,
    pub addr_contractor: Address,
    pub hours: HourlyTable,

    pub selected_field: usize,
    pub editing: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            name: format!("INV{}-TEMP000", chrono::Utc::now().year() - 2000),
            addr_client: Address::new(AddressKind::Client),
            addr_contractor: Address::new(AddressKind::Contractor),
            hours: HourlyTable::default(),
            selected_field: 0,
            editing: false,
        }
    }

    pub fn get_addr_from_kind(&self, kind: AddressKind) -> &Address {
        match kind {
            AddressKind::Client => &self.addr_client,
            AddressKind::Contractor => &self.addr_contractor,
        }
    }

    pub fn incr_selected_field(&mut self) {
        self.selected_field = (self.selected_field + 1) % (N_FIELDS + 1);
    }

    pub fn decr_selected_field(&mut self) {
        self.selected_field = self.selected_field.checked_sub(1).unwrap_or(N_FIELDS);
    }

    pub fn edit_selected_field(&mut self, kc: KeyCode) {
        match kc {
            KeyCode::Esc => self.editing = false,
            KeyCode::Tab => self.incr_selected_field(),
            KeyCode::BackTab => self.decr_selected_field(),
            _ => {
                match self.selected_field {
                    0 => Self::edit_field(self.name.borrow_mut(), kc),
                    1 => Self::edit_field(self.addr_contractor.name.borrow_mut(), kc),
                    2 => Self::edit_field(self.addr_contractor.street_num.borrow_mut(), kc),
                    3 => Self::edit_field(self.addr_contractor.city.borrow_mut(), kc),
                    4 => Self::edit_field(self.addr_contractor.state.borrow_mut(), kc),
                    5 => Self::edit_field(self.addr_contractor.zip.borrow_mut(), kc),
                    6 => Self::edit_field(self.addr_client.name.borrow_mut(), kc),
                    7 => Self::edit_field(self.addr_client.street_num.borrow_mut(), kc),
                    8 => Self::edit_field(self.addr_client.city.borrow_mut(), kc),
                    9 => Self::edit_field(self.addr_client.state.borrow_mut(), kc),
                    10 => Self::edit_field(self.addr_client.zip.borrow_mut(), kc),
                    _ => unreachable!(""),
                };
            }
        }
    }

    fn edit_field(field: &mut String, kc: KeyCode) {
        match kc {
            KeyCode::Backspace => {
                let _ = field.pop();
            }
            KeyCode::Char(c) => field.push(c),
            _ => {}
        };
    }
}
