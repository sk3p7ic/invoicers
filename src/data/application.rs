use chrono::Datelike;

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
}

impl App {
    pub fn new() -> Self {
        Self {
            name: format!("INV{}-TEMP000", chrono::Utc::now().year() - 2000),
            addr_client: Address::new(AddressKind::Client),
            addr_contractor: Address::new(AddressKind::Contractor),
            hours: HourlyTable::default(),
            selected_field: 0,
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
}
