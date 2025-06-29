use std::io::Write;

use rtm_core::processor::{ClientAccount, ClientAccountState};

pub struct CsvWriter<T: Write> {
    writer: csv::Writer<T>,
}

impl<T: Write> CsvWriter<T> {
    pub fn new(stream: T) -> Self {
        let mut writer = csv::Writer::from_writer(stream);
        writer
            .write_record(["client", "available", "held", "total", "locked"])
            .unwrap();
        Self { writer }
    }

    pub fn write_client_account(&mut self, record: &ClientAccount) {
        let client_id = record.client_id.as_u16().to_string();
        let available = record.available_balance.to_string();
        let held = record.held_balance.to_string();
        let total = (record.available_balance.clone() + record.held_balance.clone()).to_string();
        let locked = match record.state {
            ClientAccountState::Normal => "false",
            ClientAccountState::Locked => "true",
        };
        self.writer
            .write_record([&client_id, &available, &held, &total, locked])
            .unwrap();
    }
}
