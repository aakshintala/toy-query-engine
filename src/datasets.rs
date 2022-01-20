use std::error::Error;
use std::fmt::Display;

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
impl Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let capital = if self.Capital.is_some() {
            self.Capital.unwrap().to_string()
        } else {
            String::new()
        };
        f.write_fmt(format_args!(
            "{},{},{},{},{}",
            self.CountryCode, self.CountryName, self.Continent, self.CountryPop, capital,
        ))
    }
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

impl Display for City {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{},{},{},{}",
            self.CityID, self.CityName, self.CountryCode, self.CityPop,
        ))
    }
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

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.CountryCode, self.Language,))
    }
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
