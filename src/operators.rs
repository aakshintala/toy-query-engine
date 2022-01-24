use std::collections::HashMap;
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
    NoSuchColumn {
        operator: String,
        column_name: String,
    },
    OrderByColumnNotNumeric {
        column_name: String,
    },
}

impl Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OperatorError::NoSuchColumn {
                operator,
                column_name,
            } => f.write_fmt(format_args!(
                "You attempted to {} the {} column, which was not found in the data.",
                operator, column_name
            )),
            OperatorError::OrderByColumnNotNumeric { column_name } => f.write_fmt(format_args!(
                "You attempted to ORDERBY the {} column whose type is not numeric.",
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
                numeric_columns: City::numeric_columns()
                    .iter()
                    .map(|s| s.to_string())
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
                numeric_columns: Country::numeric_columns()
                    .iter()
                    .map(|s| s.to_string())
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
                numeric_columns: Language::numeric_columns()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            })
        }
    }
}

fn process_select(chain: Box<Operator>, column_names: Vec<String>) -> Result<Table, OperatorError> {
    let data = process_operator(*chain)?;
    let mut col_indices = Vec::<usize>::new();
    let mut numeric_columns = Vec::<String>::new();
    for name in &column_names {
        match data.find_column_index_by_name(name) {
            Some(index) => col_indices.push(index),
            None => {
                return Err(OperatorError::NoSuchColumn {
                    operator: String::from("SELECT"),
                    column_name: name.clone(),
                })
            }
        }
        if data.numeric_columns.contains(name) {
            numeric_columns.push(name.clone())
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
        numeric_columns,
    })
}

fn process_take(chain: Box<Operator>, count: usize) -> Result<Table, OperatorError> {
    let table = process_operator(*chain)?;
    Ok(Table {
        header: table.header,
        rows: table
            .rows
            .iter()
            .take(count)
            .map(|row| row.clone())
            .collect::<Vec<Row>>(),
        numeric_columns: table.numeric_columns,
    })
}

fn process_orderby(chain: Box<Operator>, column: String) -> Result<Table, OperatorError> {
    let mut table = process_operator(*chain)?;
    if !table.numeric_columns.contains(&column) {
        return Err(OperatorError::OrderByColumnNotNumeric {
            column_name: column,
        });
    }
    let col_index = match table.find_column_index_by_name(&column) {
        Some(index) => index,
        None => {
            return Err(OperatorError::NoSuchColumn {
                operator: String::from("ORDERBY"),
                column_name: column,
            })
        }
    };
    table.rows.sort_by(|a: &Row, b: &Row| {
        let b_val = match b.cells[col_index] {
            Cell::Int64(val) => val,
            _ => unreachable!(),
        };
        let a_val = match a.cells[col_index] {
            Cell::Int64(val) => val,
            _ => unreachable!(),
        };
        b_val.cmp(&a_val)
    });
    Ok(table)
}

fn process_countby(chain: Box<Operator>, column: String) -> Result<Table, OperatorError> {
    let table = process_operator(*chain)?;
    let col_index = match table.find_column_index_by_name(&column) {
        Some(index) => index,
        None => {
            return Err(OperatorError::NoSuchColumn {
                operator: String::from("COUNTBY"),
                column_name: column,
            })
        }
    };

    let histogram: Vec<Row> = table
        .rows
        .into_iter()
        .fold(HashMap::<Cell, usize>::new(), |mut m, x| {
            *m.entry(x.cells[col_index].clone()).or_default() += 1;
            m
        })
        .into_iter()
        .map(|(cell, count)| Row {
            cells: vec![cell, Cell::Int64(count as i64)],
        })
        .collect();

    Ok(Table {
        header: vec![column.clone(), String::from("count")],
        numeric_columns: if table.numeric_columns.contains(&column) {
            vec![column.clone(), String::from("count")]
        } else {
            vec![String::from("count")]
        },
        rows: histogram,
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
        Operator::OrderBy { chain, column } => process_orderby(chain, column),
        Operator::CountBy { chain, column } => process_countby(chain, column),
        Operator::Join {
            chain: _,
            right: _,
            column: _,
        } => todo!(),
    }
}
