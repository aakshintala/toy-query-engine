use std::fmt::Display;

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    String(String),
    Int64(i64),
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

#[derive(Clone, Debug)]
pub struct Row {
    pub cells: Vec<Cell>,
}

impl Row {
    pub fn join(&self) -> String {
        self.cells
            .iter()
            .map(|cell| format!("{}", cell))
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.join()))
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    pub header: Vec<String>,
    pub numeric_columns: Vec<String>,
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
