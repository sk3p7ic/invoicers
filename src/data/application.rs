use std::borrow::BorrowMut;

use chrono::Datelike;
use crossterm::event::KeyCode;
use enum_iterator::{next_cycle, previous_cycle, Sequence};

use super::{
    address::{Address, AddressKind}, status::{StatusMessage, StatuslineStatus}, table::HourlyTable
};

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
    HoursDesc,
    HoursRate,
    HoursHours,
    HoursBtn
}

impl AppField {
    pub fn get_len(&self) -> u8 {
        match self {
            Self::Name => 16,
            Self::AddrContractorName => 32,
            Self::AddrContractorStreetNum => 32,
            Self::AddrContractorCity => 32,
            Self::AddrContractorState => 2,
            Self::AddrContractorZip => 5,
            Self::AddrClientName => 32,
            Self::AddrClientStreetNum => 32,
            Self::AddrClientCity => 32,
            Self::AddrClientState => 2,
            Self::AddrClientZip => 5,
            Self::HoursDesc => 32,
            Self::HoursRate => 6,
            Self::HoursHours => 6,
            Self::HoursBtn => 0
        }
    }
}

pub struct App {
    pub name: String,
    pub addr_client: Address,
    pub addr_contractor: Address,
    pub hours: HourlyTable,

    pub selected_field: AppField,
    pub editing: bool,
    pub status: StatusMessage
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
            status: StatusMessage::default()
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
        let status = match kc {
            KeyCode::Esc => {self.editing = false; None},
            KeyCode::Tab => {self.incr_selected_field(); None},
            KeyCode::BackTab => {self.decr_selected_field(); None},
            _ => {
                match self.selected_field {
                    AppField::Name => Self::edit_field(&self.selected_field, self.name.borrow_mut(), kc),
                    AppField::AddrContractorName => Self::edit_field(&self.selected_field, self.addr_contractor.name.borrow_mut(), kc),
                    AppField::AddrContractorStreetNum => Self::edit_field(&self.selected_field, self.addr_contractor.street_num.borrow_mut(), kc),
                    AppField::AddrContractorCity => Self::edit_field(&self.selected_field, self.addr_contractor.city.borrow_mut(), kc),
                    AppField::AddrContractorState => Self::edit_field(&self.selected_field, self.addr_contractor.state.borrow_mut(), kc),
                    AppField::AddrContractorZip => Self::edit_field(&self.selected_field, self.addr_contractor.zip.borrow_mut(), kc),
                    AppField::AddrClientName => Self::edit_field(&self.selected_field, self.addr_client.name.borrow_mut(), kc),
                    AppField::AddrClientStreetNum => Self::edit_field(&self.selected_field, self.addr_client.street_num.borrow_mut(), kc),
                    AppField::AddrClientCity => Self::edit_field(&self.selected_field, self.addr_client.city.borrow_mut(), kc),
                    AppField::AddrClientState => Self::edit_field(&self.selected_field, self.addr_client.state.borrow_mut(), kc),
                    AppField::AddrClientZip => Self::edit_field(&self.selected_field, self.addr_client.zip.borrow_mut(), kc),
                    AppField::HoursDesc => Self::edit_field(&self.selected_field, self.hours.new_desc.borrow_mut(), kc),
                    AppField::HoursRate => Self::edit_field(&self.selected_field, self.hours.new_rate.borrow_mut(), kc),
                    AppField::HoursBtn | AppField::HoursHours if kc == KeyCode::Enter => {
                        let status = match self.hours.add_row() {
                            Ok(_) => None,
                            Err(_) => Some(StatusMessage {
                                msg: String::from("Could not add record."),
                                status: Some(StatuslineStatus::Error)
                            })
                        };
                        self.selected_field = AppField::HoursDesc;
                        status
                    },
                    AppField::HoursHours => Self::edit_field(&self.selected_field, self.hours.new_hours.borrow_mut(), kc),
                    AppField::HoursBtn => None
                }
            }
        };
        if let Some(s) = status {
            self.status = s;
        } else {
            self.status = StatusMessage { msg: String::new(), status: None };
        }
    }

    fn edit_field(field_type: &AppField, field_string: &mut String, kc: KeyCode) -> Option<StatusMessage> {
        match kc {
            KeyCode::Backspace => {
                let _ = field_string.pop();
                None
            }
            KeyCode::Char(c) => {
                if field_string.as_str().len() as u8 >= field_type.get_len() {
                    return Some(StatusMessage {
                        msg: format!("[WARN]: Max length for this field is {}.", field_type.get_len()),
                        status: Some(StatuslineStatus::Warning)
                    });
                }
                field_string.push(c);
                None
            },
            _ => None
        }
    }
}
