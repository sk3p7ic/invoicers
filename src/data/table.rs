#[derive(Default)]
pub struct HourlyRecord {
    desc: String,
    rate: f32,
    hours: f32,
}

#[derive(Default)]
pub struct HourlyTable {
    records: Vec<HourlyRecord>,
}
