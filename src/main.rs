use std::io::Write;

/// Prints an error message about the input being malformed to stdout.
fn print_error_message() {
    println!("Malformed input. Please check your input and try again.");
}

/// The help message to print to stdout when the user uses the `help` command.
const C_HELP_MESSAGE: &str =
    "Available Commands: \n
      FROM <dataset> - Loads the `dataset`. \n
          Maybe chained with other commands. If no other command is specified, will print the `dataset`. \n
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

/// Processes the input received on the command line and carries out the required action.
///
/// # Arguments
/// `input` : the input string to be processed.
/// `out` : A mutable reference to a [`Vec<String>`] where any produced output will be written.
///
/// # Returns
/// A boolean flag to indicate whether the program should exit.
/// false: The program should not exit.
/// true: The user has requested that the program exit.
fn process_input(input: &str, out: &mut Vec<String>) -> bool {
    match input.strip_suffix("\n") {
        Some(val) => match val {
            "help" => {
                out.push(String::from(C_HELP_MESSAGE));
                false
            }
            "exit" => {
                println!("Goodbye!");
                true
            }
            _ => {
                let tokens: Vec<&str> = input.split(" ").into_iter().map(|s| s).collect();
                if tokens.is_empty() || tokens[0] != "FROM" {
                    print_error_message();
                } else {
                    out.push(input.to_string());
                }
                false
            }
        },
        None => false,
    }
}

/// Test for NULL input
#[test]
fn test_process_input_no_input() {
    assert_eq!(process_input("\n", &mut vec![]), false);
}

/// Test 'exit' command as input
#[test]
fn test_process_input_exit() {
    assert_eq!(process_input("exit\n", &mut vec![]), true);
}

/// Test malformed command as input
#[test]
fn test_process_input_malformed1() {
    assert_eq!(process_input("FRM language.csv\n", &mut vec![]), false);
}

/// Test malformed command as input
#[test]
fn test_process_input_malformed2() {
    assert_eq!(process_input("TAKE language.csv\n", &mut vec![]), false);
}

/// Test malformed command as input
#[test]
fn test_process_input_malformed3() {
    assert_eq!(process_input("language.csv\n", &mut vec![]), false);
}

/// Test 'help'command as input
#[test]
fn test_process_input_help() {
    let help_message: Vec<String> = vec![String::from(C_HELP_MESSAGE)];
    let mut out: Vec<String> = Vec::new();
    let should_exit = process_input("help\n", &mut out);
    assert_eq!(should_exit, false);
    assert_eq!(help_message, out);
}

/// Test a well-formed input.
#[test]
fn test_process_input_correct() {
    let mut out: Vec<String> = Vec::new();
    let should_exit = process_input("FROM language.csv\n", &mut out);
    assert_eq!(should_exit, false);
    assert_eq!(out, vec![String::from("FROM language.csv\n")]);
}

fn main() {
    println!("Toy Query Engine v0.1");
    println!("Enter your query, or 'help' for more information or 'exit' to exit.");
    loop {
        print!(">");
        std::io::stdout().flush().expect("Error writing to stdout.");
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            print_error_message();
            continue;
        }
        let mut out: Vec<String> = Vec::new();
        if let true = process_input(&input, &mut out) {
            std::process::exit(0);
        } else {
            // Print whatever is in the output vector and continue;
            for str in out {
                println!("{}", str);
            }
        }
    }
}
