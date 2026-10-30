use std::io::Write;
use std::fs::OpenOptions;

use crate::laundry::LaundryStore;

pub struct JsonStorage {}

impl LaundryStore for JsonStorage {
    fn save_poll_event(
        &self,
        poll_event: crate::laundry::LaundryPollEvent,
    ) -> Result<(), crate::laundry::StorageError> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("polls.jsonl")?;
        
        serde_json::to_writer(&mut file, &poll_event)?;
        writeln!(file)?;
        Ok(())
    }

    fn load_poll_events(
        &self,
    ) -> Result<Vec<crate::laundry::LaundryPollEvent>, crate::laundry::StorageError> {
        Ok(Vec::new())
    }
}
