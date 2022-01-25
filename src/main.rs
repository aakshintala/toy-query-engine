mod commands;
mod data;
mod operators;
mod table;

use commands::*;
use operators::*;

/// Prints an error message about the input being malformed to stdout.
fn print_error_message(error_message: &str) {
    println!("Malformed input. {}", error_message);
}

/// The help message to print to stdout for the `help` command.
const C_HELP_MESSAGE: &str =
    "Available Commands: \n
      FROM <dataset> - Loads the `dataset`. \n
          Maybe chained with other commands. Must always be the first command in a chain.\n
          If no other command is specified, will print the `dataset`. \n
      SELECT <column-name> - used to select particular columns from the specified dataset. \n
          See the Datasets section below for a list of column-names for each dataset. \n
      TAKE <number> - Specifies the number of rows to print from the dataset. \n
          <number> must be greater than or equal to 0. \n
      ORDERBY <numeric-column-name> - Sorts the loaded dataset by the column-name in descending order, if the column contains numeric values. \n
          See the Datasets section below for a list of acceptable values for <numeric-column-name> for each dataset. \n
      COUNTBY <column-name> - Returns the . \n
          <number> must be greater than or equal to 0. \n
      JOIN <dataset> <column-name> - performs a join on the current dataset and the one specified in this command on <column-name>. \n
          See the Datasets section below for a list of available datasets and the column-names for each dataset. \n
          The provided <column-name> must be present in both datasets. \n
    \n
    Available Datasets\n
      <dataset> : city.csv\n
          <column-name> : [CityID, CityName, CountryCode, CityPop]\n
          <numeric-column-name> : [CityID, CityPop]\n
      <dataset> : country.csv\n
          <column-name> : [CountryCode, CountryName, Continent, CountryPop, Capital]\n
          <numeric-column-name> : [CountryPop, Capital]\n
      <dataset> : language.csv\n
          <column-name> : [CountryCode,Language]\n
          <numeric-column-name> : []\n";

fn process_input(input: &str) -> bool {
    let mut should_exit = false;
    match parse_command(input) {
        Command::Exit => {
            println!("Goodbye!");
            should_exit = true;
        }
        Command::Help => println!("{}", C_HELP_MESSAGE),
        Command::Operator(operator) => match process_operator(&operator) {
            Ok(out) => println!("{}", out),
            Err(e) => println!("{}", e),
        },
        Command::InputError(error) => print_error_message(&error),
        Command::NoInput => (),
    }
    should_exit
}

#[test]
fn test_process_input_no_input() {
    assert_eq!(process_input("\n"), false);
}

#[test]
fn test_process_input_exit() {
    assert_eq!(process_input("exit\n"), true);
}

#[test]
fn test_process_input_help() {
    assert_eq!(process_input("help\n"), false);
}

#[test]
fn test_process_input_some_command() {
    assert_eq!(process_input("FROM language.csv\n"), false);
}

#[test]
fn test_process_input_malformed_command() {
    assert_eq!(process_input("FRM language.csv\n"), false);
}

fn main() {
    println!("Toy Query Engine v0.1");
    println!("Enter your query, or 'help' for more information or 'exit' to exit.");
    loop {
        // print!(">");
        // std::io::stdout().flush().expect("Error writing to stdout.");
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            print_error_message(&e.to_string());
            continue;
        }
        let should_exit = process_input(&input);
        if should_exit {
            std::process::exit(0)
        }
    }
}
