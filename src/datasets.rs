use std::error::Error;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Country {
    pub CountryCode: String,
    pub CountryName: String,
    pub Continent: String,
    pub CountryPop: i64,
    pub Capital: Option<i64>,
}

fn load_countries() -> Result<Vec<Country>, Box<dyn Error>> {
    let mut countries: Vec<Country> = Vec::new();
    let mut csv_reader = csv::Reader::from_path("data/country.csv")?;
    for record in csv_reader.deserialize() {
        let country: Country = record?;
        countries.push(country);
    }
    Ok(countries)
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct City {
    pub CityID: i32,
    pub CityName: String,
    pub CountryCode: String,
    pub CityPop: i64,
}

fn load_cities() -> Result<Vec<City>, Box<dyn Error>> {
    let mut cities: Vec<City> = Vec::new();
    let mut csv_reader = csv::Reader::from_path("data/city.csv")?;
    for record in csv_reader.deserialize() {
        let city: City = record?;
        cities.push(city);
    }
    Ok(cities)
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct Language {
    pub CountryCode: String,
    pub Language: String,
}
fn load_languages() -> Result<Vec<Language>, Box<dyn Error>> {
    let mut languages: Vec<Language> = Vec::new();
    let mut csv_reader = csv::Reader::from_path("data/language.csv")?;
    for record in csv_reader.deserialize() {
        let language: Language = record?;
        languages.push(language);
    }
    Ok(languages)
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DataSet {
    pub countries: Vec<Country>,
    pub cities: Vec<City>,
    pub languages: Vec<Language>,
}

impl DataSet {
    pub fn new() -> Self {
        Self {
            countries: load_countries().expect("Couldn't load Country dataset."),
            cities: load_cities().expect("Couldn't load City dataset."),
            languages: load_languages().expect("Couldn't load Country dataset."),
        }
    }
}
