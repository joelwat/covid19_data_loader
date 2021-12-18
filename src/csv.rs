use anyhow::Result;
use serde::Deserialize;

use std::fs::{ File, DirEntry };


#[derive(Deserialize, Debug, Clone)]
pub struct CsvUsData {
    pub province_state: String,
    pub country_region: String,
    pub last_update: Option<String>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub confirmed: u64,
    pub deaths: u64,
    pub recovered: Option<f64>,
    pub active: Option<f64>,
    pub fips: Option<f64>,
    pub incident_rate: Option<f64>,
    pub total_test_results: Option<f64>,
    pub people_hospitalized: Option<f64>,
    pub case_fatality_ratio: Option<f64>,
    pub uid: f64,
    pub iso3: String,
    pub testing_rate: Option<f64>,
    pub hospitalization_rate: Option<f64>,
}

impl CsvUsData {
    fn normalize(self) -> CovidData {
        CovidData {
            province_state: self.province_state,
            country_region: self.country_region,
            last_update: self.last_update,
            lat: self.lat,
            long: self.long,
            confirmed: self.confirmed,
            deaths: self.deaths,
            recovered: self.recovered,
            active: self.active,
            fips: self.fips,
            incident_rate: self.incident_rate,
            total_test_results: self.total_test_results,
            people_hospitalized: self.people_hospitalized,
            case_fatality_ratio: self.case_fatality_ratio,
            uid: self.uid,
            iso3: self.iso3,
            testing_rate: self.testing_rate,
            hospitalization_rate: self.hospitalization_rate,
        }
    }
}

pub struct CsvIntData {
    pub fips: Option<i32>,
    pub admin2: Option<String>,
    pub province_state: Option<String>,
    pub country_region: String,
    pub last_update: Option<String>,
    pub lat: f64,
    pub long: f64,
    pub confirmed: u64,
    pub deaths: u64,
    pub recovered: u64,
    pub active: u64,
    pub combined_key: String,
    pub incident_rate: f64,
    pub case_fatality_ratio: f64,
}

pub struct CovidData {
    pub province_state: String,
    pub country_region: String,
    pub last_update: Option<String>,
    pub lat: Option<f64>,
    pub long: Option<f64>,
    pub confirmed: u64,
    pub deaths: u64,
    pub recovered: Option<f64>,
    pub active: Option<f64>,
    pub fips: Option<f64>,
    pub incident_rate: Option<f64>,
    pub total_test_results: Option<f64>,
    pub people_hospitalized: Option<f64>,
    pub case_fatality_ratio: Option<f64>,
    pub uid: f64,
    pub iso3: String,
    pub testing_rate: Option<f64>,
    pub hospitalization_rate: Option<f64>,
}

pub mod util {
    use super::*;

    pub fn read_file(file: DirEntry) -> Result<Vec<CsvUsData>> {
        let file = File::open(file.path()).expect("Failed to open file");

        let mut data = Vec::new();
        let mut reader = csv::Reader::from_reader(file);

        for row in reader.records() {
            let row = row.unwrap();

            let deserialized = row.deserialize(None);

            if let Err(err) = deserialized {
               println!("Failed to parse row: {:#?}", row);
               println!("Error Message: {:?}", err);
               std::process::exit(1);
            }

            data.push(deserialized.unwrap());
        }

        Ok(data)
    }
}
