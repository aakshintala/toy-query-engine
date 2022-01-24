use std::fmt::Display;

use crate::commands::Dataset;
use crate::data::{load_cities, load_countries, load_languages, City, Country, Language};
use crate::table::{Cell, Row, Table};

/// Operations supported by this tool.
/// Each operator returns a set of rows.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Operator {
    /// Selects a dataset as the data source for the operation.
    From(Dataset),
    /// Selects a column from the datset.
    Select {
        chain: Box<Operator>,
        column_names: Vec<String>,
    },
    /// Returns the first 'count' number of rows from the data.
    Take { chain: Box<Operator>, count: usize },
    /// Sorts the dataset in descending order by the specified column.
    /// The column must contain numeric values
    OrderBy {
        chain: Box<Operator>,
        column: String,
    },
    /// Returns a histogram from the dataset for the selected column.
    CountBy {
        chain: Box<Operator>,
        column: String,
    },
    /// Peforms a Merge of the chained and right data sets on the specified column.
    Join {
        chain: Box<Operator>,
        right: Dataset,
        column: String,
    },
}

#[allow(dead_code)]
pub enum OperatorError {
    SelectInvalidColumn { column_name: String },
}

impl Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OperatorError::SelectInvalidColumn { column_name } => f.write_fmt(format_args!(
                "Column of name {} not found in selected dataset.",
                column_name
            )),
        }
    }
}
fn process_from(dataset: Dataset) -> Result<Table, OperatorError> {
    match dataset {
        Dataset::City => {
            let cities = load_cities().expect("Couldn't load city.csv");
            Ok(Table {
                header: City::column_names().iter().map(|s| s.to_string()).collect(),
                rows: cities
                    .into_iter()
                    .map(|city| -> Row { city.into() })
                    .collect(),
            })
        }
        Dataset::Country => {
            let countries = load_countries().expect("Couldn't load city.csv");
            Ok(Table {
                header: Country::column_names()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                rows: countries
                    .into_iter()
                    .map(|country| -> Row { country.into() })
                    .collect(),
            })
        }
        Dataset::Language => {
            let languages = load_languages().expect("Couldn't load language.csv");
            Ok(Table {
                header: Language::column_names()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
                rows: languages
                    .into_iter()
                    .map(|language| -> Row { language.into() })
                    .collect(),
            })
        }
    }
}

fn process_select(chain: Box<Operator>, column_names: Vec<String>) -> Result<Table, OperatorError> {
    let data = process_operator(*chain)?;
    let mut col_indices = Vec::<usize>::new();
    for column in &column_names {
        match data
            .header
            .iter()
            .enumerate()
            .find(|(_, col_name)| *col_name == column)
        {
            Some((index, _)) => col_indices.push(index),
            None => {
                return Err(OperatorError::SelectInvalidColumn {
                    column_name: column.clone(),
                })
            }
        }
    }
    let mut rows = Vec::<Row>::new();
    for row in data.rows {
        let mut cells = Vec::<Cell>::new();
        for col_index in &col_indices {
            cells.push(row.cells[*col_index].clone())
        }
        rows.push(Row { cells });
    }
    Ok(Table {
        header: column_names,
        rows,
    })
}

fn process_take(chain: Box<Operator>, count: usize) -> Result<Table, OperatorError> {
    let table = process_operator(*chain)?;
    Ok(Table {
        header: table.header.clone(),
        rows: table
            .rows
            .iter()
            .take(count)
            .map(|row| row.clone())
            .collect::<Vec<Row>>(),
    })
}

pub fn process_operator(operator: Operator) -> Result<Table, OperatorError> {
    match operator {
        Operator::From(dataset) => process_from(dataset),
        Operator::Select {
            chain,
            column_names,
        } => process_select(chain, column_names),
        Operator::Take { chain, count } => process_take(chain, count),
        Operator::OrderBy {
            chain: _,
            column: _,
        } => todo!(),
        Operator::CountBy {
            chain: _,
            column: _,
        } => todo!(),
        Operator::Join {
            chain: _,
            right: _,
            column: _,
        } => todo!(),
    }
}
