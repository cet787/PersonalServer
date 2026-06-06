use csv;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FlightTimeEntry {
    date: String,
    
}
struct FlightHours {
    records: csv::StringRecord,
}

impl FlightHours {

    fn open_csv(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(filename)?;

        for result in rdr.records(){
            let record = result?;

            self.records = record;
        }

        Ok(())
    }


}