use chrono::Datelike;

use super::{
    address::{Address, AddressKind},
    table::HourlyTable,
};

pub struct App {
    pub name: String,
    addr_client: Address,
    addr_contractor: Address,
    hours: HourlyTable,
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
}
