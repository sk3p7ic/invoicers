pub enum AddressKind {
    Client,
    Contractor,
}

pub struct Address {
    kind: AddressKind,
    name: String,
    street_num: String,
    city: String,
    state: String,
    zip: String,
}

impl Address {
    pub fn new(kind: AddressKind) -> Self {
        Self {
            kind,
            name: String::new(),
            street_num: String::new(),
            city: String::new(),
            state: String::new(),
            zip: String::new(),
        }
    }
}
