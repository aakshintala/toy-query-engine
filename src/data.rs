use std::error::Error;
use std::fmt::Display;

use serde::Deserialize;

use crate::table::{Cell, Row};

/// In-memory representation of each record in the `country.csv` dataset.
/// This is represented as a struct so we can use the [`serde`] and [`csv`] crates to generate
/// the deserialization code.
///
/// Example record:
/// CountryCode, CountryName, Continent,        CountryPop, Capital
/// ABW,         Aruba,       North_America,    103000,     129
#[derive(Clone, Debug, Deserialize, PartialEq)]
// This is necessary as the header in the dataset (`country.csv`) is in CamelCase. `serde` and `csv`
// rely on these names being the same as those in the header row in the dataset.
#[allow(non_snake_case)]
pub struct Country {
    /// "ABW" in the example above.
    pub CountryCode: String,
    /// "Aruba" in the example above.
    pub CountryName: String,
    /// "North_America" in the example above.
    pub Continent: String,
    /// 103000 in the example above.
    pub CountryPop: i64,
    /// 129 in the example above.
    pub Capital: Option<i64>,
}

impl Country {
    /// Returns the names of the columns in the City dataset.
    pub fn column_names() -> Vec<String> {
        vec![
            "CountryCode".to_string(),
            "CountryName".to_string(),
            "Continent".to_string(),
            "CountryPop".to_string(),
            "Capital".to_string(),
        ]
    }

    /// Returns the names of only those columns whose values are numeric.
    pub fn numeric_columns() -> Vec<String> {
        vec!["CountryPop".to_string()]
    }
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

/// Trait to make it easy to convert the Country struct in a [`Row`].
impl Into<Row> for Country {
    fn into(self) -> Row {
        Row {
            cells: vec![
                Cell::String(self.CountryCode),
                Cell::String(self.CountryName),
                Cell::String(self.Continent),
                Cell::Int64(self.CountryPop),
                Cell::OptInt64(self.Capital),
            ],
        }
    }
}

/// Helper function to deserialize the `country.csv` dataset.
///
/// Returns
/// A vector of all the rows in the dataset represented as a [`Country`], or
/// an error propagated from the csv and serde deserialization code.
pub fn load_countries() -> Result<Vec<Country>, Box<dyn Error>> {
    let mut countries: Vec<Country> = Vec::new();
    let mut csv_reader = csv::Reader::from_path("data/country.csv")?;
    for record in csv_reader.deserialize() {
        let country: Country = record?;
        countries.push(country);
    }
    Ok(countries)
}

#[test]
fn test_load_countries() {
    let countries = load_countries();
    assert!(countries.is_ok());
    let countries = countries.unwrap();
    assert!(countries.len() > 0);
    let first = countries.first().unwrap().to_owned();
    assert_eq!(
        first,
        Country {
            CountryCode: "ABW".to_string(),
            CountryName: "Aruba".to_string(),
            Continent: "North_America".to_string(),
            CountryPop: 103000,
            Capital: Some(129),
        }
    );
}

/// In-memory representation of each record in the `city.csv` dataset.
/// This is represented as a struct so we can use the [`serde`] and [`csv`] crates to generate
/// the deserialization code.
///
/// Example record:
/// CityID, CityName,   CountryCode,    CityPop
/// 1,      Kabul,      AFG,            1780000
#[derive(Clone, Debug, Deserialize, PartialEq)]
// This is necessary as the header row in the dataset (`city.csv`) is in CamelCase. `serde` and
// `csv` rely on these names being the same as those in the header row in the dataset.
#[allow(non_snake_case)]
pub struct City {
    /// 1 in the example above.
    pub CityID: i64,
    /// "Kabul" in the example above.
    pub CityName: String,
    /// "AFG" in the example above.
    pub CountryCode: String,
    /// 1780000 in the example above.
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

/// Trait to make it easy to convert the [`City`] struct in a [`Row`].
impl Into<Row> for City {
    fn into(self) -> Row {
        Row {
            cells: vec![
                Cell::Int64(self.CityID),
                Cell::String(self.CityName),
                Cell::String(self.CountryCode),
                Cell::Int64(self.CityPop),
            ],
        }
    }
}

impl City {
    /// Returns the names of the columns in the City dataset.
    pub fn column_names() -> Vec<String> {
        vec![
            "CityID".to_string(),
            "CityName".to_string(),
            "CountryCode".to_string(),
            "CityPop".to_string(),
        ]
    }

    /// Returns the names of only those columns whose values are numeric.
    pub fn numeric_columns() -> Vec<String> {
        vec!["CityID".to_string(), "CityPop".to_string()]
    }
}

/// Helper function to deserialize the `city.csv` dataset.
///
/// Returns
/// A vector of all the rows in the dataset represented as a [`City`], or
/// an error propagated from the csv and serde deserialization code.
pub fn load_cities() -> Result<Vec<City>, Box<dyn Error>> {
    let mut cities: Vec<City> = Vec::new();
    let mut csv_reader = csv::Reader::from_path("data/city.csv")?;
    for record in csv_reader.deserialize() {
        let city: City = record?;
        cities.push(city);
    }
    Ok(cities)
}

#[test]
fn test_load_cities() {
    let cities = load_cities();
    assert!(cities.is_ok());
    let cities = cities.unwrap();
    assert!(cities.len() > 0);
    let first = cities.first().unwrap().to_owned();
    assert_eq!(
        first,
        City {
            CityID: 1,
            CityName: "Kabul".to_string(),
            CountryCode: "AFG".to_string(),
            CityPop: 1780000,
        }
    );
}

/// In-memory representation of each record in the `city.csv` dataset.
/// This is represented as a struct so we can use the [`serde`] and [`csv`] crates to generate
/// the deserialization code.
///
/// Example record:
/// CountryCode,    Language
/// ABW,            Dutch
#[derive(Clone, Debug, Deserialize, PartialEq)]
// This is necessary as the header row in the dataset (`city.csv`) is in CamelCase. `serde` and
// `csv` rely on these names being the same as those in the header row in the dataset.
#[allow(non_snake_case)]
pub struct Language {
    /// "ABW" in the example above.
    pub CountryCode: String,
    /// "Dutch" in the example above.
    pub Language: String,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.CountryCode, self.Language,))
    }
}

impl Language {
    /// Returns the names of the columns in the Language dataset.
    pub fn column_names() -> Vec<String> {
        vec!["CountryCode".to_string(), "Language".to_string()]
    }

    /// Returns the names of only those columns whose values are numeric.
    pub fn numeric_columns() -> Vec<String> {
        vec![]
    }
}

/// Trait to make it easy to convert the [`Language`] struct in a [`Row`].
impl Into<Row> for Language {
    fn into(self) -> Row {
        Row {
            cells: vec![Cell::String(self.CountryCode), Cell::String(self.Language)],
        }
    }
}

/// Helper function to deserialize the `language.csv` dataset.
///
/// Returns
/// A vector of all the rows in the dataset represented as a [`Language`], or
/// an error propagated from the csv and serde deserialization code.
pub fn load_languages() -> Result<Vec<Language>, Box<dyn Error>> {
    let mut languages: Vec<Language> = Vec::new();
    let mut csv_reader = csv::Reader::from_path("data/language.csv")?;
    for record in csv_reader.deserialize() {
        let language: Language = record?;
        languages.push(language);
    }
    Ok(languages)
}

#[test]
fn test_load_languages() {
    let languages = load_languages();
    assert!(languages.is_ok());
    let languages = languages.unwrap();
    assert!(languages.len() > 0);
    let first = languages.first().unwrap().to_owned();
    assert_eq!(
        first,
        Language {
            CountryCode: "ABW".to_string(),
            Language: "Dutch".to_string(),
        }
    );
}

/// The datasets known to the toy-query-engine.
#[derive(Debug, Clone, PartialEq)]
pub enum Dataset {
    /// city.csv
    City,
    /// country.csv
    Country,
    /// language.csv
    Language,
}

impl Display for Dataset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Dataset::City => f.write_str("city.csv"),
            Dataset::Country => f.write_str("country.csv"),
            Dataset::Language => f.write_str("language.csv"),
        }
    }
}
