#[derive(Clone, Copy)]
pub enum AddressKind {
    Client,
    Contractor,
}

impl AddressKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Client => "Client",
            Self::Contractor => "Contractor",
        }
    }
}

pub struct Address {
    pub kind: AddressKind,
    pub name: String,
    pub street_num: String,
    pub city: String,
    pub state: String,
    pub zip: String,
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
