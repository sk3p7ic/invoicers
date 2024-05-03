use chrono::Datelike;

use super::{
    address::{Address, AddressKind},
    table::HourlyTable,
};

pub struct App {
    pub name: String,
    pub addr_client: Address,
    pub addr_contractor: Address,
    pub hours: HourlyTable,
}

impl App {
    pub fn new() -> Self {
        Self {
            name: format!("INV{}-TEMP000", chrono::Utc::now().year() - 2000),
            addr_client: Address::new(AddressKind::Client),
            addr_contractor: Address::new(AddressKind::Contractor),
            hours: HourlyTable::default(),
        }
    }

    pub fn get_addr_from_kind(&self, kind: AddressKind) -> &Address {
        match kind {
            AddressKind::Client => &self.addr_client,
            AddressKind::Contractor => &self.addr_contractor,
        }
    }
}
