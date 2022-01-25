use std::fmt::Display;

/// Type used to hold data in the Table. All data must be wrapped in one of these variants.
/// Cells correspond to the columns of a row.
#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd)]
pub enum Cell {
    /// The value in the cell is a String.
    String(String),
    /// The value in the Cell is an integer.
    Int64(i64),
    /// The value in the Cell is an integer, if it exists.
    /// Primarily used for the `Capital` column in the `country.csv`.
    /// Example:
    /// CountryCode,CountryName,Continent,CountryPop,Capital
    /// ASM,American_Samoa,Oceania,68000,54
    /// ATA,Antarctica,Antarctica,0,
    ///                             ^--- No capital.
    /// ATF,French_Southern_territories,Antarctica,0,
    ///                                              ^--- No capital.
    /// ATG,Antigua_and_Barbuda,North_America,68000,63
    OptInt64(Option<i64>),
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::String(val) => f.write_fmt(format_args!("{}", val)),
            Cell::Int64(val) => f.write_fmt(format_args!("{}", val)),
            Cell::OptInt64(val) => {
                if val.is_some() {
                    f.write_fmt(format_args!("{}", val.unwrap()))
                } else {
                    f.write_fmt(format_args!("{}", String::new()))
                }
            }
        }
    }
}

/// Type used to represent a row of data in the data being processed.
#[derive(Clone, Debug)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    /// Constructs as a comma-seperated String from the Row's cells.
    pub fn join(&self) -> String {
        self.cells
            .iter()
            .map(|cell| format!("{}", cell))
            .collect::<Vec<String>>()
            .join(",")
    }
}

/// Test Row::join for a row with an OptInt64 Col.
#[test]
fn test_row_join_without_opt() {
    let row = Row {
        cells: vec![
            Cell::String("Hello".to_string()),
            Cell::String("World".to_string()),
            Cell::Int64(15),
            Cell::Int64(-15),
            Cell::OptInt64(Some(15)),
            Cell::OptInt64(Some(-15)),
        ],
    };
    assert_eq!(row.join(), String::from("Hello,World,15,-15,15,-15"))
}

/// Test Row::join for a row with an OptInt64 Col.
#[test]
fn test_row_join_with_opt() {
    let row = Row {
        cells: vec![
            Cell::String("Hello".to_string()),
            Cell::String("World".to_string()),
            Cell::Int64(15),
            Cell::Int64(-15),
            Cell::OptInt64(Some(15)),
            Cell::OptInt64(Some(-15)),
            Cell::OptInt64(None),
        ],
    };
    assert_eq!(row.join(), String::from("Hello,World,15,-15,15,-15,"))
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.join()))
    }
}

/// Type used to represent the data being queried.
#[derive(Clone, Debug)]
pub struct Table {
    /// The names of the columns in each row.
    pub header: Vec<String>,
    /// Extra book keeping to remember which rows in the table contain numeric values.
    /// Primarily used to quickly figure out which rows in a table the ORDERBY operation can be
    /// performed on.
    pub numeric_columns: Vec<String>,
    /// The actual data in the column. Each [`Row`] has 1 [`Cell`] per entry in the `header`.
    pub rows: Vec<Row>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n", self.header.join(",")))?;

        for row in &self.rows {
            f.write_fmt(format_args!("{}\n", row))?;
        }

        Ok(())
    }
}

impl Table {
    /// Returns the index into the `header` field that corresponds to the first occurrence of
    /// 'name'.
    ///
    /// # Arguments:
    /// 'name' : The name of the column whose index is to be returned.
    ///
    /// # Returns:
    /// [`Some(usize)`] for the index of the first occurrence of `name` in the `header` field.
    /// [`None`] if `name` is not found in the `header` field.
    pub fn find_column_index_by_name(&self, name: &str) -> Option<usize> {
        match self
            .header
            .iter()
            .enumerate()
            .find(|(_, col_name)| *col_name == name)
        {
            Some((index, _)) => Some(index),
            None => None,
        }
    }
}

/// Test find_column_index_by_name for names that do exist in the table.
#[test]
fn test_find_column_index_by_name_exists() {
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
    assert_eq!(table.find_column_index_by_name("H1"), Some(0));
    assert_eq!(table.find_column_index_by_name("H2"), Some(1));
    assert_eq!(table.find_column_index_by_name("H3"), Some(2));
    assert_eq!(table.find_column_index_by_name("H4"), Some(3));
}

/// Test find_column_index_by_name for names that do not exist in the table.
#[test]
fn test_find_column_index_by_name_does_not_exist() {
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
    assert_eq!(table.find_column_index_by_name("H"), None);
    assert_eq!(table.find_column_index_by_name("H12"), None);
    assert_eq!(table.find_column_index_by_name("H31"), None);
    assert_eq!(table.find_column_index_by_name("H44"), None);
}

/// Test find_column_index_by_name for names that exist in a table with duplicate header entries.
#[test]
fn test_find_column_index_by_name_duplicates() {
    let table = Table {
        header: vec![
            "H1".to_string(),
            "H2".to_string(),
            "H1".to_string(),
            "H2".to_string(),
        ],
        numeric_columns: vec![],
        rows: vec![],
    };
    assert_eq!(table.find_column_index_by_name("H1"), Some(0));
    assert_eq!(table.find_column_index_by_name("H2"), Some(1));
    assert_eq!(table.find_column_index_by_name("H1"), Some(0));
    assert_eq!(table.find_column_index_by_name("H2"), Some(1));
}
