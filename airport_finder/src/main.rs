use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Runway {
    id: u32,
    airport_ref: u32,
    airport_ident: String,
    length_ft: Option<u32>,
    width_ft: Option<u32>,
    surface: Option<String>,
    lighted: Option<u32>,
    closed: Option<u32>,
    le_ident: Option<String>,
    le_latitude_deg: Option<f64>,
    le_longitude_deg: Option<f64>,
    le_elevation_ft: Option<u32>,
    le_heading_degT: Option<f64>,
    le_displaced_threshold_ft: Option<u32>,
    he_ident: Option<String>,
    he_latitude_deg: Option<f64>,
    he_longitude_deg: Option<f64>,
    he_elevation_ft: Option<u32>,
    he_heading_degT: Option<f64>,
    he_displaced_threshold_ft: Option<u32>,
}



fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("data/runways.csv")?;
    for result in rdr.deserialize() {
        let record: Runway = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
    read_csv().unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    });
}



// "id","airport_ref","airport_ident","length_ft","width_ft","surface","lighted","closed","le_ident","le_latitude_deg",
// "le_longitude_deg","le_elevation_ft","le_heading_degT","le_displaced_threshold_ft","he_ident","he_latitude_deg",
// "he_longitude_deg","he_elevation_ft","he_heading_degT","he_displaced_threshold_ft"