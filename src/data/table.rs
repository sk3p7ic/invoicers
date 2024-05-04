use std::num::ParseFloatError;

use ratatui::widgets::Row;

#[derive(Default)]
pub struct HourlyRecord {
    pub desc: String,
    pub rate: f32,
    pub hours: f32,
}

impl HourlyRecord {
    pub fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

#[derive(Default)]
pub struct HourlyTable {
    pub records: Vec<HourlyRecord>,
    pub new_desc: String,
    pub new_rate: String,
    pub new_hours: String,
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
                    format!("{:.2}", r.total()),
                ])
            })
            .collect()
    }

    pub fn add_row(&mut self) -> Result<(), ParseFloatError> {
        let desc = self.new_desc.clone();
        let rate = self.new_rate.parse()?;
        let hours = self.new_hours.parse()?;

        self.records.push(HourlyRecord { desc, rate, hours });
        self.new_desc.clear();
        self.new_rate.clear();
        self.new_hours.clear();
        Ok(())
    }
}
