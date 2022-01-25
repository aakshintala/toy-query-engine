use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::vec;

use crate::data::{load_cities, load_countries, load_languages, City, Country, Dataset, Language};
use crate::table::{Cell, Row, Table};

/// Operations supported by this tool.
/// These are constructed by parsing the user input on the toy-query-engine command line.
/// See [`crate::commands::parse_command`]
/// Each operator returns a [`Table`].
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// Loads a dataset from disk.
    /// See [`Dataset`] for available datasets.
    From(Dataset),
    /// Selects a column from the [`Table`] produced by the chained operator.
    Select {
        ///  Chain of [`Operator`]s that must be executed to produce the input [`Table`] for this
        /// operator.
        chain: Box<Operator>,
        /// The name of the column to select from the input [`Table`].
        column_names: Vec<String>,
    },
    /// Returns the first 'count' number of rows from the [`Table`] produced by the chained
    /// operator.
    Take {
        ///  Chain of [`Operator`]s that must be executed to produce the input [`Table`] for this
        /// operator.
        chain: Box<Operator>,
        /// The number of rows from the input [`Table`] to return.
        count: usize,
    },
    /// Sorts the dataset in descending order by the specified column.
    /// The column must contain numeric values
    OrderBy {
        ///  Chain of [`Operator`]s that must be executed to produce the input [`Table`] for this
        /// operator.
        chain: Box<Operator>,
        /// The name of the column to reverse sort (i.e., in descending order) the input [`Table`]
        /// by.
        column: String,
    },
    /// Returns a histogram from the dataset for the selected column.
    CountBy {
        ///  Chain of [`Operator`]s that must be executed to produce the input [`Table`] for this
        /// operator.
        chain: Box<Operator>,
        /// The name of the column to produce the histogram for.
        column: String,
    },
    /// Peforms a Merge of the chained and right data sets on the specified column.
    Join {
        /// Chain of [`Operator`]s that must be executed to produce the `left` [`Table`] for this
        /// operator.
        chain: Box<Operator>,
        /// The [`Dataset`] to load as the `right` [`Table`] for the join.
        right: Dataset,
        /// The name of the column to join the `left` and `right` tables on.
        column: String,
    },
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::From(dataset) => f.write_fmt(format_args!("FROM {}", dataset)),
            Operator::Select {
                chain,
                column_names,
            } => f.write_fmt(format_args!("{} SELECT {}", *chain, column_names.join(","))),
            Operator::Take { chain, count } => {
                f.write_fmt(format_args!("{} TAKE {}", *chain, count))
            }
            Operator::OrderBy { chain, column } => {
                f.write_fmt(format_args!("{} ORDERBY {}", *chain, column))
            }
            Operator::CountBy { chain, column } => {
                f.write_fmt(format_args!("{} COUNTBY {}", *chain, column))
            }
            Operator::Join {
                chain,
                right,
                column,
            } => f.write_fmt(format_args!("{} JOIN {} {}", *chain, right, column)),
        }
    }
}

/// The set of errors that can be returned when processing the [`Operator`]s.
/// This is primarily used to display an error message when processing fails.
#[derive(Debug)]
pub enum OperatorError {
    /// Encountered an error while trying to load the dataset from disk while processing the FROM
    /// or JOIN operators.
    CSVError {
        /// The name of the dataset that was passed to the FROM command.
        dataset: Dataset,
        /// The error returned from the [`serde`] or [`csv`] crates.
        error: Box<dyn Error>,
        /// The operator that was being processed when this error occurred.
        operator: String,
    },
    /// The `column_name` provided to the `operator` does not exist in its input [`Table`].
    NoSuchColumn {
        /// The operator that was being processed when this error was thrown
        operator: String,
        /// The operator chain where this error was thrown.
        chain: Box<Operator>,
        /// Name of the column that was specified as an argument to the operator.
        column_name: String,
    },
    /// Indicates that the `column_name` passed to the ORDERBY command is illegal as its values are
    /// non-numeric.
    OrderByColumnNotNumeric {
        /// Name of the column that was specified as an argument to the ORDERBY command.
        column_name: String,
    },
}

impl Display for OperatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OperatorError::CSVError {
                dataset,
                error,
                operator,
            } => f.write_fmt(format_args!(
                "Failed to load the {} dataset while processing the {} command. Error encountered: {}",
                dataset, operator, error
            )),
            OperatorError::NoSuchColumn {
                operator,
                chain,
                column_name,
            } => f.write_fmt(format_args!(
                "Could not find the {} column to {} on the table produced by this operator chain: {}",
                column_name, operator, chain,
            )),
            OperatorError::OrderByColumnNotNumeric { column_name } => f.write_fmt(format_args!(
                "You attempted to ORDERBY the {} column whose type is not numeric.",
                column_name
            )),
        }
    }
}

/// Common helper function to load the requested [`Dataset`] from disk.
///
/// # Arguments:
/// `dataset`: the [`Dataset`] to be laoded.
/// `operator`: the name of the operator that called this function. Used for error reporting.
///
/// # Returns:
/// On success: The loaded dataset as a [`Table`].
/// On failure: [`OperatorError::CSVError`] or other [`OperatorError`] from processing the
/// chained operators.
fn load_dataset(dataset: &Dataset, operator: &str) -> Result<Table, OperatorError> {
    match dataset {
        Dataset::City => match load_cities() {
            Ok(cities) => Ok(Table {
                header: City::column_names(),
                rows: cities
                    .into_iter()
                    .map(|city| -> Row { city.into() })
                    .collect(),
                numeric_columns: City::numeric_columns(),
            }),
            Err(e) => Err(OperatorError::CSVError {
                dataset: dataset.clone(),
                error: e,
                operator: operator.to_string(),
            }),
        },
        Dataset::Country => match load_countries() {
            Ok(countries) => Ok(Table {
                header: Country::column_names(),
                rows: countries
                    .into_iter()
                    .map(|country| -> Row { country.into() })
                    .collect(),
                numeric_columns: Country::numeric_columns(),
            }),
            Err(e) => Err(OperatorError::CSVError {
                dataset: dataset.clone(),
                error: e,
                operator: operator.to_string(),
            }),
        },
        Dataset::Language => match load_languages() {
            Ok(languages) => Ok(Table {
                header: Language::column_names(),
                rows: languages
                    .into_iter()
                    .map(|language| -> Row { language.into() })
                    .collect(),
                numeric_columns: Language::numeric_columns(),
            }),
            Err(e) => Err(OperatorError::CSVError {
                dataset: dataset.clone(),
                error: e,
                operator: operator.to_string(),
            }),
        },
    }
}

/// Handles the [`Operator::From`] operator by loading the requested [`Dataset`] from disk.
/// This is just a shim around the [`load_dataset`] function.
///
/// # Arguments:
/// `dataset`: the [`Dataset`] to be laoded.
///
/// # Returns:
/// On success: The loaded dataset as a [`Table`].
/// On failure: [`OperatorError::CSVError`] or other [`OperatorError`] from processing the
/// chained operators.
fn process_from(dataset: &Dataset) -> Result<Table, OperatorError> {
    load_dataset(dataset, "FROM")
}

#[test]
fn test_process_from_city() {
    let result = process_from(&Dataset::City);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 4079);
    assert_eq!(result.rows[0].cells.len(), 4);
}

#[test]
fn test_process_from_country() {
    let result = process_from(&Dataset::Country);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 239);
    assert_eq!(result.rows[0].cells.len(), 5);
}

#[test]
fn test_process_from_language() {
    let result = process_from(&Dataset::Language);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 984);
    assert_eq!(result.rows[0].cells.len(), 2);
}

/// Helper function to find the index that corresponds to the first occurrence of 'name' in `table`.
///
/// # Arguments:
/// 'table' : The table to find the column in.
/// 'name' : The name of the column whose index is to be returned.
/// 'chain' : The chain on operators that produced this table (used to construct the error message
/// if the column doesn't exist in the table).
/// 'current_operator': The operator calling this function.
///
/// # Returns:
/// Ok([`usize`]) for the index of the first occurrence of `name` in the `table`.
/// Err([`OperatorError::NoSuchColumn`]) if `name` is not found in the `header` field.
fn find_column_index(
    table: &Table,
    name: &str,
    chain: &Box<Operator>,
    current_operator: &str,
) -> Result<usize, OperatorError> {
    match table.find_column_index_by_name(name) {
        Some(index) => Ok(index),
        None => {
            // The requested column doesn't exist in the table.
            Err(OperatorError::NoSuchColumn {
                operator: current_operator.to_string(),
                chain: chain.clone(),
                column_name: name.to_string(),
            })
        }
    }
}

/// Test find_column_index for names that do exist in the table.
#[test]
fn test_find_column_index_exists() {
    let table = Table {
        header: vec![
            "H1".to_string(),
            "H2".to_string(),
            "H3".to_string(),
            "H4".to_string(),
        ],
        numeric_columns: vec![],
        rows: vec![],
    };

    let operator = Box::new(Operator::From(Dataset::Language));
    assert!(find_column_index(&table, "H1", &operator, "TEST").is_ok());
    assert!(find_column_index(&table, "H2", &operator, "TEST").is_ok());
    assert!(find_column_index(&table, "H3", &operator, "TEST").is_ok());
    assert!(find_column_index(&table, "H4", &operator, "TEST").is_ok());
}

/// Test find_column_index_by_name for names that do not exist in the table.
#[test]
fn test_find_column_index_does_not_exist() {
    let table = Table {
        header: vec![
            "H1".to_string(),
            "H2".to_string(),
            "H3".to_string(),
            "H4".to_string(),
        ],
        numeric_columns: vec![],
        rows: vec![],
    };
    let operator = Box::new(Operator::From(Dataset::Language));
    assert!(find_column_index(&table, "H", &operator, "TEST").is_err());
    assert!(find_column_index(&table, "H12", &operator, "TEST").is_err());
    assert!(find_column_index(&table, "H31", &operator, "TEST").is_err());
    assert!(find_column_index(&table, "H42", &operator, "TEST").is_err());
}

/// Test find_column_index_by_name for names that do not exist in the table.
#[test]
fn test_find_column_index_empty_table() {
    let table = Table {
        header: vec![],
        numeric_columns: vec![],
        rows: vec![],
    };
    let operator = Box::new(Operator::From(Dataset::Language));
    assert!(find_column_index(&table, "H", &operator, "TEST").is_err());
    assert!(find_column_index(&table, "H12", &operator, "TEST").is_err());
    assert!(find_column_index(&table, "H31", &operator, "TEST").is_err());
    assert!(find_column_index(&table, "H42", &operator, "TEST").is_err());
}

/// Handles the [`Operator::Select`] operator by processing the [`Operator`] chain and selecting the
/// requested column(s) from the resulting [`Table`].
///
/// # Arguments:
/// `chain`: A chain of one or more [`Operator`]s that produce the [`Table`] that is the input for
/// this operator.
/// `column_names`: Names of one or more columns to select from the output of the `chain`.
///
/// # Returns:
/// On success: A [`Table`] containing only the requested columns.
/// On failure: [`OperatorError::NoSuchColumn`] or other [`OperatorError`] from processing the
/// chained operators.
fn process_select(
    chain: &Box<Operator>,
    column_names: &Vec<String>,
) -> Result<Table, OperatorError> {
    // Run the chained operators to produce the input for this operator.
    // Will terminate this function and return the produced error if the processing fails.
    let table = process_operator(&**chain)?;

    // Find the indices corresponding to the input `column_names`.
    let mut col_indices = Vec::<usize>::new();
    for name in column_names {
        // This can throw the [`OperatorError::NoSuchColumn`] error.
        let index = find_column_index(&table, &name, chain, "Select")?;
        col_indices.push(index);
    }

    // Construct the output using the col_indices previously calculated.
    Ok(Table {
        header: column_names.clone(),
        rows: table
            .rows
            .iter()
            .map(|row| Row {
                // Extract the cells at the previously computed col_indices into a new Row.
                cells: col_indices
                    .iter()
                    .map(|index| row.cells[*index].clone())
                    .collect(),
            })
            .collect(),
        // Extract only those numeric_columns in the input table that are in the `column_names`.
        numeric_columns: column_names
            .iter()
            .filter(|name| table.numeric_columns.contains(name))
            .map(|name| name.clone())
            .collect(),
    })
}

#[test]
fn test_process_select_single() {
    let result = process_select(
        &Box::new(Operator::From(Dataset::Language)),
        &vec!["Language".to_string()],
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 984);
    assert_eq!(result.header.len(), 1);
    assert_eq!(result.header[0], "Language".to_string());
    assert_eq!(result.rows[0].cells.len(), 1);
}

#[test]
fn test_process_select_single_non_existant_col() {
    let result = process_select(
        &Box::new(Operator::From(Dataset::Language)),
        &vec!["Capital".to_string()],
    );
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.to_string(), "Could not find the Capital column to Select on the table produced by this operator chain: FROM language.csv".to_string())
}

#[test]
fn test_process_select_multiple() {
    let result = process_select(
        &Box::new(Operator::From(Dataset::City)),
        &vec!["CityID".to_string(), "CityName".to_string()],
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 4079);
    assert_eq!(result.header.len(), 2);
    assert_eq!(
        result.header,
        vec!["CityID".to_string(), "CityName".to_string()]
    );
    assert_eq!(result.rows[0].cells.len(), 2);
}

/// Handles the [`Operator::Take`] operator by processing the [`Operator`] chain and selecting the
/// first `count` column(s) from the resulting [`Table`].
///
/// # Arguments:
/// `chain`: A chain of one or more [`Operator`]s that produce the [`Table`] that is the input for
/// this operator.
/// `count`: Number of rows to retain in the output. If `count` is greater than the number of rows
/// in the input table, all rows in the input table will be returned.
///
/// # Returns:
/// On success: A [`Table`] containing only the requested number of rows.
/// On failure: [`OperatorError`] from processing the chained operators.
fn process_take(chain: &Box<Operator>, count: usize) -> Result<Table, OperatorError> {
    // Run the chained operators to produce the input for this operator.
    // Will terminate this function and return the produced error if the processing fails.
    let table = process_operator(&**chain)?;

    Ok(Table {
        header: table.header,
        rows: table
            .rows
            .iter()
            .take(count)
            .map(|row| row.clone())
            .collect(),
        numeric_columns: table.numeric_columns,
    })
}

#[test]
fn test_process_take() {
    let result = process_take(&Box::new(Operator::From(Dataset::Language)), 5);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 5);
    assert_eq!(result.header.len(), 2);
    assert_eq!(
        result.header,
        vec!["CountryCode".to_string(), "Language".to_string()]
    );
    assert_eq!(result.numeric_columns.len(), 0);
}

#[test]
fn test_process_take_from_empty_table() {
    let result = process_take(
        &Box::new(Operator::Take {
            chain: Box::new(Operator::From(Dataset::Language)),
            count: 0,
        }),
        5,
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 0);
    assert_eq!(result.header.len(), 2);
    assert_eq!(
        result.header,
        vec!["CountryCode".to_string(), "Language".to_string()]
    );
    assert_eq!(result.numeric_columns.len(), 0);
}

#[test]
fn test_process_take_more_than_rows_in_data() {
    let result = process_take(&Box::new(Operator::From(Dataset::Language)), 10000);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 984);
    assert_eq!(result.header.len(), 2);
    assert_eq!(
        result.header,
        vec!["CountryCode".to_string(), "Language".to_string()]
    );
    assert_eq!(result.numeric_columns.len(), 0);
}

/// Helper function to sort the input 'rows' on the `col_index` column.
/// # Usage Note: The caller must guarantee that the col_index exists in the table and is numeric.
fn sort_table(rows: &mut Vec<Row>, col_index: usize) {
    rows.sort_by(|a: &Row, b: &Row| {
        let b_val = match b.cells[col_index] {
            Cell::Int64(val) => val,
            // This is unreachable because we would have returned
            // OperatorError::OrderByColumnNotNumeric in the check above if this column was not
            // numeric.
            _ => unreachable!(),
        };
        let a_val = match a.cells[col_index] {
            Cell::Int64(val) => val,
            // This is unreachable because we would have returned
            // OperatorError::OrderByColumnNotNumeric in the check above if this column was not
            // numeric.
            _ => unreachable!(),
        };
        b_val.cmp(&a_val)
    });
}

/// Handles the [`Operator::OrderBy`] operator by processing the [`Operator`] chain and reverse
/// sorting (descending order) the rows of the resulting [`Table`] by the `column`.
///
/// # Arguments:
/// `chain`: A chain of one or more [`Operator`]s that produce the [`Table`] that is the input for
/// this operator.
/// `column`: Name of the column to reverse sort by. Must be a `numeric` column, i.e., the values in
/// the column must be numeric.
///
/// # Returns:
/// On success: A [`Table`] containing only the sorted rows.
/// On failure: [`OperatorError::OrderByColumnNotNumeric`] if the input column is not a numeric
/// column, or  [`OperatorError::NoSuchColumn`] if the input column is not found, or any
/// other [`OperatorError`] produced on processing the operator chain.
fn process_orderby(chain: &Box<Operator>, column: String) -> Result<Table, OperatorError> {
    // Run the chained operators to produce the input for this operator.
    // Will terminate this function and return the produced error if the processing fails.
    let mut table = process_operator(&**chain)?;

    // Ensure the `column` to sort by is a numeric column.
    if !table.numeric_columns.contains(&column) {
        return Err(OperatorError::OrderByColumnNotNumeric {
            column_name: column,
        });
    }

    // Find the index corresponding to the `column`.
    // This can throw the [`OperatorError::NoSuchColumn`] error.
    let col_index = find_column_index(&table, &column, chain, "ORDERBY")?;

    // Do the actual sort
    sort_table(&mut table.rows, col_index);

    Ok(table)
}

#[test]
fn test_process_orderby_numeric() {
    let result = process_orderby(
        &Box::new(Operator::Take {
            chain: Box::new(Operator::From(Dataset::City)),
            count: 10,
        }),
        "CityPop".to_string(),
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 10);
    assert_eq!(result.header.len(), 4);
    assert!(result.rows[0].cells[3] >= result.rows[1].cells[3]);
    assert!(result.rows[1].cells[3] >= result.rows[2].cells[3]);
    assert!(result.rows[2].cells[3] >= result.rows[3].cells[3]);
}

#[test]
fn test_process_orderby_non_numeric() {
    let result = process_orderby(
        &Box::new(Operator::Take {
            chain: Box::new(Operator::From(Dataset::City)),
            count: 10,
        }),
        "CityName".to_string(),
    );
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(
        err.to_string(),
        "You attempted to ORDERBY the CityName column whose type is not numeric.".to_string()
    );
}

/// Handles the [`Operator::CountBy`] operator by processing the [`Operator`] chain and produces a
/// [`Table`] containing only two columns: the first contains the values of the specified `column`,
/// and the second `count` column contains the number of times that value appears in the dataset.
///
/// # Arguments:
/// `chain`: A chain of one or more [`Operator`]s that produce the [`Table`] that is the input for
/// this operator.
/// `column`: Name of the column to create the histogram for.
///
/// # Returns:
/// On success: A [`Table`] containing the two columns described above.
/// On failure: [`OperatorError::NoSuchColumn`] if the input column is not found, or any
/// other [`OperatorError`] produced on processing the operator chain.
fn process_countby(chain: &Box<Operator>, column: String) -> Result<Table, OperatorError> {
    // Run the chained operators to produce the input for this operator.
    // Will terminate this function and return the produced error if the processing fails.
    let table = process_operator(&**chain)?;

    // Find the index corresponding to the `column`.
    // This can throw the [`OperatorError::NoSuchColumn`] error.
    let col_index = find_column_index(&table, &column, chain, "COUNTBY")?;

    let mut histogram: Vec<Row> = table
        .rows
        .into_iter()
        // Count the number of times each `value` in the selected column occurs in the input table
        // using a hashmap with Key = `value` and Value = count.
        .fold(HashMap::<Cell, usize>::new(), |mut m, x| {
            *m.entry(x.cells[col_index].clone()).or_default() += 1;
            m
        })
        .into_iter()
        // Output each (Key, Value) in the resulting hashamp as a Row.
        .map(|(cell, count)| Row {
            cells: vec![cell, Cell::Int64(count as i64)],
        })
        .collect();

    // sort the histogram on the 'count' column for stable ordering in the output.
    sort_table(&mut histogram, col_index);

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

#[test]
fn test_process_countby() {
    let result = process_countby(
        &Box::new(Operator::Take {
            chain: Box::new(Operator::From(Dataset::Language)),
            count: 100,
        }),
        "Language".to_string(),
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 70);
    assert_eq!(result.header.len(), 2);
    assert_eq!(
        result.rows[0].cells,
        vec![Cell::String("English".to_string()), Cell::Int64(7)],
    );
    assert_eq!(
        result.rows[1].cells,
        vec![Cell::String("Arabic".to_string()), Cell::Int64(4)],
    );
}

#[test]
fn test_process_countby_empty() {
    let result = process_countby(
        &Box::new(Operator::Take {
            chain: Box::new(Operator::From(Dataset::Language)),
            count: 0,
        }),
        "Language".to_string(),
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 0);
    assert_eq!(result.header.len(), 2);
}

#[test]
fn test_process_countby_no_such_column() {
    let result = process_countby(
        &Box::new(Operator::Take {
            chain: Box::new(Operator::From(Dataset::Language)),
            count: 100,
        }),
        "CityPop".to_string(),
    );
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert_eq!(result.to_string(), "Could not find the CityPop column to COUNTBY on the table produced by this operator chain: FROM language.csv TAKE 100".to_string());
}

/// Handles the [`Operator::Join`] operator by processing the [`Operator`] chain to produce the
/// 'left' table and loading the `dataset` as the 'right' table and performing a left-join on them
/// on the input `column`.
///
/// # Arguments:
/// `chain`: A chain of one or more [`Operator`]s that produce the 'left' [`Table`] to join on.
/// `dataset`: The dataset to load for the 'right' table to join on.
/// `column`: Name of the column to perform the left-join on. This column must be in both the 'left'
/// and 'right' tables.
///
/// # Returns:
/// On success: A [`Table`] containing the joined rows.
/// On failure: [`OperatorError::NoSuchColumn`] if the input column is not found, or any
/// other [`OperatorError`] produced on processing the operator chain.
fn process_join(
    chain: &Box<Operator>,
    dataset: &Dataset,
    column: String,
) -> Result<Table, OperatorError> {
    // Run the chained operators to produce the input for this operator.
    // Will terminate this function and return the produced error if the processing fails.
    let left = process_operator(&**chain)?;

    // Load the right table.
    // This can throw [`OperatorError::CSVError`].
    let right = load_dataset(dataset, "JOIN")?;

    // Make sure the column to join on is in both the 'left' and 'right' tables.
    if !(left.header.contains(&column) && right.header.contains(&column)) {
        return Err(OperatorError::NoSuchColumn {
            operator: String::from("JOIN"),
            chain: chain.clone(),
            column_name: column,
        });
    }

    // Construct the new header by concatenating the headers of the 'left' and 'right' tables,
    // taking care to remove the common column from the 'right' table.
    let header = {
        let mut header = left.header.clone();
        for name in &right.header {
            if *name != column {
                header.push(name.clone());
            }
        }
        header
    };

    // Construct the new numeric_columns by concatenating the numeric_columns of the 'left' and
    // 'right' tables, taking care to remove the common column from the 'right' table.
    let numeric_columns = {
        let mut numeric_columns = left.numeric_columns.clone();
        for name in &right.numeric_columns {
            if *name != column {
                numeric_columns.push(name.clone());
            }
        }
        numeric_columns
    };

    // Perform the actual join using the "nested-loop" algorithm.
    let rows: Vec<Row> = {
        let mut rows: Vec<Row> = Vec::new();
        let left_index = left.find_column_index_by_name(&column).unwrap();
        let right_index = right.find_column_index_by_name(&column).unwrap();
        for left_row in &left.rows {
            for right_row in &right.rows {
                if left_row.cells[left_index] == right_row.cells[right_index] {
                    let mut row = left_row.clone();
                    for (index, cell) in right_row.cells.iter().enumerate() {
                        if index != right_index {
                            row.cells.push(cell.clone());
                        }
                    }
                    rows.push(row);
                }
            }
        }
        rows
    };

    Ok(Table {
        header,
        numeric_columns,
        rows,
    })
}

#[test]
fn test_process_join_simple() {
    let result = process_join(
        &Box::new(Operator::From(Dataset::City)),
        &Dataset::Country,
        "CountryCode".to_string(),
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 4079);
    assert_eq!(
        result.rows[4066].cells,
        vec![
            Cell::Int64(4067),
            Cell::String("Charlotte_Amalie".to_string()),
            Cell::String("VIR".to_string()),
            Cell::Int64(13000),
            Cell::String("Virgin_Islands_U.S.".to_string()),
            Cell::String("North_America".to_string()),
            Cell::Int64(93000),
            Cell::OptInt64(Some(4067))
        ]
    )
}

#[test]
fn test_process_join_complex() {
    let result = process_join(
        &Box::new(Operator::Join {
            chain: Box::new(Operator::From(Dataset::City)),
            right: Dataset::Country,
            column: "CountryCode".to_string(),
        }),
        &Dataset::Language,
        "CountryCode".to_string(),
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.rows.len(), 30670);
    assert_eq!(
        result.rows[30668].cells,
        vec![
            Cell::Int64(4079),
            Cell::String("Rafah".to_string()),
            Cell::String("PSE".to_string()),
            Cell::Int64(92020),
            Cell::String("Palestine".to_string()),
            Cell::String("Asia".to_string()),
            Cell::Int64(3101000),
            Cell::OptInt64(Some(4074)),
            Cell::String("Arabic".to_string()),
        ]
    )
}

#[test]
fn test_process_join_no_such_column_left() {
    let result = process_join(
        &Box::new(Operator::Join {
            chain: Box::new(Operator::From(Dataset::City)),
            right: Dataset::Country,
            column: "Language".to_string(),
        }),
        &Dataset::Language,
        "CountryCode".to_string(),
    );
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert_eq!(result.to_string(), "Could not find the Language column to JOIN on the table produced by this operator chain: FROM city.csv".to_string());
}

#[test]
fn test_process_join_no_such_column_right() {
    let result = process_join(
        &Box::new(Operator::Join {
            chain: Box::new(Operator::From(Dataset::City)),
            right: Dataset::Country,
            column: "CountryCode".to_string(),
        }),
        &Dataset::Language,
        "Capital".to_string(),
    );
    assert!(result.is_err());
    let result = result.unwrap_err();
    assert_eq!(result.to_string(), "Could not find the Capital column to JOIN on the table produced by this operator chain: FROM city.csv JOIN country.csv CountryCode".to_string());
}

/// Handles the input [`Operator`] by delegating to the functions above.
///
/// # Arguments:
/// `operator`: The operator chain to process.
///
/// # Returns:
/// On success: A [`Table`] containing the rows obtained by processing the operator chain.
/// On failure: [`OperatorError`].
pub fn process_operator(operator: &Operator) -> Result<Table, OperatorError> {
    match operator {
        Operator::From(dataset) => process_from(dataset),
        Operator::Select {
            chain,
            column_names,
        } => process_select(chain, column_names),
        Operator::Take { chain, count } => process_take(chain, *count),
        Operator::OrderBy { chain, column } => process_orderby(chain, column.clone()),
        Operator::CountBy { chain, column } => process_countby(chain, column.clone()),
        Operator::Join {
            chain,
            right,
            column,
        } => process_join(chain, right, column.clone()),
    }
}
