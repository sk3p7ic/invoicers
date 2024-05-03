use ratatui::widgets::Row;

#[derive(Default)]
pub struct HourlyRecord {
    pub desc: String,
    pub rate: f32,
    pub hours: f32,
}

#[derive(Default)]
pub struct HourlyTable {
    pub records: Vec<HourlyRecord>,
}

impl HourlyTable {
    pub fn to_rows(&self) -> Vec<Row> {
        self.records
            .iter()
            .map(|r| {
                Row::new(vec![
                    r.desc.clone(),
                    format!("{:.2}", r.rate),
                    format!("{:.2}", r.hours),
                    format!("{:.2}", r.rate * r.hours),
                ])
            })
            .collect()
    }
}
