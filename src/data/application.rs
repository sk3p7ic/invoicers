use std::borrow::BorrowMut;

use chrono::Datelike;
use crossterm::event::KeyCode;
use enum_iterator::{next_cycle, previous_cycle, Sequence};

use super::{
    address::{Address, AddressKind},
    table::HourlyTable,
};

const N_FIELDS: usize = 11;

#[derive(PartialEq, Sequence)]
pub enum AppField {
    Name,
    AddrContractorName,
    AddrContractorStreetNum,
    AddrContractorCity,
    AddrContractorState,
    AddrContractorZip,
    AddrClientName,
    AddrClientStreetNum,
    AddrClientCity,
    AddrClientState,
    AddrClientZip,
}

pub struct App {
    pub name: String,
    pub addr_client: Address,
    pub addr_contractor: Address,
    pub hours: HourlyTable,

    pub selected_field: AppField,
    pub editing: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            name: format!("INV{}-TEMP000", chrono::Utc::now().year() - 2000),
            addr_client: Address::new(AddressKind::Client),
            addr_contractor: Address::new(AddressKind::Contractor),
            hours: HourlyTable::default(),
            selected_field: AppField::Name,
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
        self.selected_field = next_cycle(&self.selected_field);
    }

    pub fn decr_selected_field(&mut self) {
        self.selected_field = previous_cycle(&self.selected_field);
    }

    pub fn edit_selected_field(&mut self, kc: KeyCode) {
        match kc {
            KeyCode::Esc => self.editing = false,
            KeyCode::Tab => self.incr_selected_field(),
            KeyCode::BackTab => self.decr_selected_field(),
            _ => {
                match self.selected_field {
                    AppField::Name => Self::edit_field(self.name.borrow_mut(), kc),
                    AppField::AddrContractorName => Self::edit_field(self.addr_contractor.name.borrow_mut(), kc),
                    AppField::AddrContractorStreetNum => Self::edit_field(self.addr_contractor.street_num.borrow_mut(), kc),
                    AppField::AddrContractorCity => Self::edit_field(self.addr_contractor.city.borrow_mut(), kc),
                    AppField::AddrContractorState => Self::edit_field(self.addr_contractor.state.borrow_mut(), kc),
                    AppField::AddrContractorZip => Self::edit_field(self.addr_contractor.zip.borrow_mut(), kc),
                    AppField::AddrClientName => Self::edit_field(self.addr_client.name.borrow_mut(), kc),
                    AppField::AddrClientStreetNum => Self::edit_field(self.addr_client.street_num.borrow_mut(), kc),
                    AppField::AddrClientCity => Self::edit_field(self.addr_client.city.borrow_mut(), kc),
                    AppField::AddrClientState => Self::edit_field(self.addr_client.state.borrow_mut(), kc),
                    AppField::AddrClientZip => Self::edit_field(self.addr_client.zip.borrow_mut(), kc),
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
