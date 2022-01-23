use std::error::Error;
use std::fmt::Display;

use serde::Deserialize;

/// In-memory representation of each record in the `country.csv` dataset.
/// This is represented as a struct so we can use the [`serde`] and [`csv`] crates to generate
/// the deserialization code.
///
/// Example record:
/// CountryCode, CountryName, Continent,        CountryPop, Capital
/// ABW,         Aruba,       North_America,    103000,     129
#[derive(Clone, Debug, Deserialize)]
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
    pub fn column_names() -> Vec<&'static str> {
        vec![
            "CountryCode",
            "CountryName",
            "Continent",
            "CountryPop",
            "Capital",
        ]
    }
    #[allow(dead_code)]
    pub fn numeric_columns() -> Vec<&'static str> {
        vec!["CountryPop", "Capital"]
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

/// In-memory representation of each record in the `city.csv` dataset.
/// This is represented as a struct so we can use the [`serde`] and [`csv`] crates to generate
/// the deserialization code.
///
/// Example record:
/// CityID, CityName,   CountryCode,    CityPop
/// 1,      Kabul,      AFG,            1780000
#[derive(Clone, Debug, Deserialize)]
// This is necessary as the header row in the dataset (`city.csv`) is in CamelCase. `serde` and
// `csv` rely on these names being the same as those in the header row in the dataset.
#[allow(non_snake_case)]
pub struct City {
    /// 1 in the example above.
    pub CityID: i32,
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

impl City {
    pub fn column_names() -> Vec<&'static str> {
        vec!["CityID", "CityName", "CountryCode", "CityPop"]
    }

    #[allow(dead_code)]
    pub fn numeric_columns() -> Vec<&'static str> {
        vec!["CityID", "CityPop"]
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

/// In-memory representation of each record in the `city.csv` dataset.
/// This is represented as a struct so we can use the [`serde`] and [`csv`] crates to generate
/// the deserialization code.
///
/// Example record:
/// CountryCode,    Language
/// ABW,            Dutch
#[derive(Clone, Debug, Deserialize)]
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
    pub fn column_names() -> Vec<&'static str> {
        vec!["CountryCode", "Language"]
    }

    #[allow(dead_code)]
    pub fn numeric_columns() -> Vec<&'static str> {
        vec![]
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

#[derive(Clone, Debug)]
pub enum RowFragment {
    City(City),
    Country(Country),
    Language(Language),
}

impl RowFragment {
    pub fn headers(&self, excluded: Vec<bool>) -> Vec<&str> {
        match self {
            RowFragment::City(_) => City::column_names(),
            RowFragment::Country(_) => Country::column_names(),
            RowFragment::Language(_) => Language::column_names(),
        }
        .into_iter()
        .enumerate()
        .filter(|(index, _)| excluded[*index] == false)
        .map(|(_, name)| name)
        .collect()
    }

    fn to_string_vec(&self, excluded: Vec<bool>) -> Vec<String> {
        match self {
            RowFragment::City(data) => excluded
                .into_iter()
                .enumerate()
                .filter(|(_, val)| *val == false)
                .map(|(index, _)| match index {
                    0 => data.CityID.to_string(),
                    1 => data.CityName.clone(),
                    2 => data.CountryCode.clone(),
                    3 => data.CityPop.to_string(),
                    _ => unreachable!(),
                })
                .collect(),
            RowFragment::Country(data) => excluded
                .into_iter()
                .enumerate()
                .filter(|(_, val)| *val == false)
                .map(|(index, _)| match index {
                    0 => data.CountryCode.clone(),
                    1 => data.CountryName.clone(),
                    2 => data.Continent.clone(),
                    3 => data.CountryPop.to_string(),
                    4 => {
                        if data.Capital.is_some() {
                            data.Capital.unwrap().to_string()
                        } else {
                            String::new()
                        }
                    }
                    _ => unreachable!(),
                })
                .collect(),
            RowFragment::Language(data) => excluded
                .into_iter()
                .enumerate()
                .filter(|(_, val)| *val == false)
                .map(|(index, _)| match index {
                    0 => data.CountryCode.clone(),
                    1 => data.Language.clone(),
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Row {
    pub fragments: Vec<RowFragment>,
}

#[derive(Clone, Debug)]
pub enum ExcludedColumns {
    City([bool; 4]),
    Country([bool; 5]),
    Language([bool; 2]),
}

#[allow(dead_code)]
impl ExcludedColumns {
    pub fn get_num_included_columns(&self) -> usize {
        match self {
            ExcludedColumns::City(excluded) => excluded
                .into_iter()
                .filter(|is_excluded| **is_excluded == false)
                .count(),
            ExcludedColumns::Country(excluded) => excluded
                .into_iter()
                .filter(|is_excluded| **is_excluded == false)
                .count(),
            ExcludedColumns::Language(excluded) => excluded
                .into_iter()
                .filter(|is_excluded| **is_excluded == false)
                .count(),
        }
    }
}

impl Into<Vec<bool>> for ExcludedColumns {
    fn into(self) -> Vec<bool> {
        match self {
            ExcludedColumns::City(array) => array.into(),
            ExcludedColumns::Country(array) => array.into(),
            ExcludedColumns::Language(array) => array.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Data {
    pub rows: Vec<Row>,
    pub excluded_columns: Vec<ExcludedColumns>,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column_headers: String = self.rows[0]
            .fragments
            .iter()
            .enumerate()
            .map(|(index, fragment)| fragment.headers(self.excluded_columns[index].clone().into()))
            .flatten()
            .collect::<Vec<&str>>()
            .join(",");

        f.write_fmt(format_args!("{}\n", column_headers))?;

        for row in &self.rows {
            let row_vals: String = row
                .fragments
                .iter()
                .enumerate()
                .map(|(index, fragment)| {
                    fragment.to_string_vec(self.excluded_columns[index].clone().into())
                })
                .flatten()
                .collect::<Vec<String>>()
                .join(",");

            f.write_fmt(format_args!("{}\n", row_vals))?;
        }

        Ok(())
    }
}
