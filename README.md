# toy-query-engine
A small command-line program for querying CSV files.

## Organization
```
.
├── Cargo.toml          - Cargo build config.
├── LICENSE             - MIT license file
├── README.md           - This file
├── data                - All the CSV files are stored in this directory
│   ├── city.csv
│   ├── country.csv
│   └── language.csv
├── src                 - All source and unit tests.
│   ├── commands.rs     - Parses the CLI input into `command`s to execute.
│   ├── data.rs         - Types and functions for dealing with loading the CSV data.
│   ├── main.rs         - The main driver logic.
│   ├── operators.rs    - Types and functions for computing the requested query.
│   └── table.rs        - Types for in-memory representation of the data during processing.
├── tests
│   └── cli.rs          - tests the CLI
```

## Quickstart Guide
1. Install rust by following instructions at https://rustup.rs/. This codebase was developed on `rustc 1.60.0-nightly`.
1. `cargo build` to download all the dependencies and build the tool.
1. `cargo test` to build and run the unit and CLI tests.
1. `cargo run --release` and take the tool for a spin!
    1. `help` for the list of supported commands. The help message is reproduced below for convenience.
        ```
        Available Commands:
        FROM <dataset> - Loads the `dataset`.
            Maybe chained with other commands. Must always be the first command in a chain. If no other command is specified, will print the `dataset`.
        SELECT <column-name> - used to select particular columns from the specified dataset.
            See the Datasets section below for a list of column-names for each dataset.
        TAKE <number> - Specifies the number of rows to print from the dataset.
            <number> must be greater than or equal to 0.
        ORDERBY <numeric-column-name> - Sorts the loaded dataset by the column-name in descending order, if the column contains numeric values.
            See the Datasets section below for a list of acceptable values for <numeric-column-name> for each dataset.
        COUNTBY <column-name> - Returns the .
            <number> must be greater than or equal to 0.
        JOIN <dataset> <column-name> - performs a join on the current dataset and the one specified in this command on <column-name>.
            See the Datasets section below for a list of available datasets and the column-names for each dataset.
            The provided <column-name> must be present in both datasets.

        Available Datasets
        <dataset> : city.csv
            <column-name> : [CityID, CityName, CountryCode, CityPop]
            <numeric-column-name> : [CityID, CityPop]
        <dataset> : country.csv
            <column-name> : [CountryCode, CountryName, Continent, CountryPop, Capital]
            <numeric-column-name> : [CountryPop, Capital]
        <dataset> : language.csv
            <column-name> : [CountryCode,Language]
            <numeric-column-name> : []
        ```
    1. `exit` to exit.
