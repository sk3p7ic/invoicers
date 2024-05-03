#[derive(Clone, Copy)]
pub enum StatuslineStatus { Error, Warning }

#[derive(Default, Clone)]
pub struct StatusMessage {
    pub msg: String,
    pub status: Option<StatuslineStatus>
}
