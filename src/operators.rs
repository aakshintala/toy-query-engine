use std::fmt::Display;

use crate::commands::Dataset;
use crate::data::{
    load_cities, load_countries, load_languages, Data, ExcludedColumns, Row, RowFragment,
};

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
        column: String,
    },
    /// Returns the first 'count' number of rows from the data.
    Take { chain: Box<Operator>, count: i32 },
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

pub fn process_operator(operator: Operator) -> Result<Data, OperatorError> {
    match operator {
        Operator::From(dataset) => match dataset {
            Dataset::City => {
                let cities = load_cities().expect("Couldn't load city.csv");
                Ok(Data {
                    rows: cities
                        .into_iter()
                        .map(|city| Row {
                            fragments: vec![RowFragment::City(city)],
                        })
                        .collect(),
                    excluded_columns: vec![ExcludedColumns::City([false; 4])],
                })
            }
            Dataset::Country => {
                let countries = load_countries().expect("Couldn't load city.csv");
                Ok(Data {
                    rows: countries
                        .into_iter()
                        .map(|country| Row {
                            fragments: vec![RowFragment::Country(country)],
                        })
                        .collect(),
                    excluded_columns: vec![ExcludedColumns::Country([false; 5])],
                })
            }
            Dataset::Language => {
                let languages = load_languages().expect("Couldn't load language.csv");
                Ok(Data {
                    rows: languages
                        .into_iter()
                        .map(|language| Row {
                            fragments: vec![RowFragment::Language(language)],
                        })
                        .collect(),
                    excluded_columns: vec![ExcludedColumns::Language([false; 2])],
                })
            }
        },
        Operator::Select { chain, column: _ } => {
            let data = process_operator(*chain)?;
            Ok(data)
            // for (fragment_index, fragment) in data.rows[0].fragments.iter().enumerate() {
            //     let header =
            // fragment.headers(data.excluded_columns[fragment_index].clone().into());
            //     match header.iter().enumerate().find(|(index, s)| **s == &column) {
            //         Some((col_index, column_name)) => {
            //             let mut excluded: Vec<bool> =
            // data.excluded_columns[fragment_index].into();             let excluded:
            // Vec<bool> = excluded                 .iter()
            //                 .enumerate()
            //                 .map(|(index, val)| if index == col_index { false } else { true })
            //                 .collect();
            //             Ok(data)
            //         }
            //         None => Err(OperatorError::SelectInvalidColumn {
            //             column_name: column.clone(),
            //         }),
            //     }
            // }
        }
        Operator::Take { chain: _, count: _ } => todo!(),
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
