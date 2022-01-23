use crate::operators::Operator;

/// The datasets known to the toy-query-engine.
#[derive(Debug, Clone, PartialEq)]
pub enum Dataset {
    /// city.csv
    City,
    /// country.csv
    Country,
    /// language.csv
    Language,
}

/// Commands parsed from user input.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// The user entered the `exit` command.
    Exit,
    /// The user entered the `help` command.
    Help,
    /// The user entered a FROM <dataset> operator.
    Operator(Operator),
    /// The user's input is erroneous.
    InputError,
}

/// Parses the command entered on the CLI into a [`Command`].
///
/// # Arguments
/// `input` : the input string to be processed.
///
/// # Returns
/// A [`Command`] that represents the parsed input.
pub fn parse_command(input: &str) -> Command {
    match input.strip_suffix("\n") {
        Some(val) => match val {
            "help" => Command::Help,
            "exit" => Command::Exit,
            _ => {
                let tokens: Vec<&str> = val.split(" ").into_iter().map(|s| s).collect();
                if tokens.is_empty() || tokens[0] != "FROM" {
                    Command::InputError
                } else if tokens[0] == "FROM" && tokens.len() > 1 {
                    match tokens[1] {
                        "language.csv" => Command::Operator(Operator::From(Dataset::Language)),
                        "city.csv" => Command::Operator(Operator::From(Dataset::City)),
                        "country.csv" => Command::Operator(Operator::From(Dataset::Country)),
                        _ => Command::InputError,
                    }
                } else {
                    Command::InputError
                }
            }
        },
        None => Command::InputError,
    }
}

/// Test for NULL input
#[test]
fn test_parse_command_no_input() {
    assert_eq!(parse_command("\n"), Command::InputError);
}

/// Test 'exit' command as input
#[test]
fn test_parse_command_exit() {
    assert_eq!(parse_command("exit\n"), Command::Exit);
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed1() {
    assert_eq!(parse_command("FRM language.csv\n"), Command::InputError);
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed2() {
    assert_eq!(parse_command("TAKE language.csv\n"), Command::InputError);
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed3() {
    assert_eq!(parse_command("language.csv\n"), Command::InputError);
}

/// Test 'help'command as input
#[test]
fn test_parse_command_help() {
    assert_eq!(parse_command("help\n"), Command::Help);
}

/// Test well-formed input: `FROM language.csv`.
#[test]
fn test_parse_command_from_language() {
    assert_eq!(
        parse_command("FROM language.csv\n"),
        Command::Operator(Operator::From(Dataset::Language))
    );
}

/// Test well-formed input: `FROM city.csv`.
#[test]
fn test_parse_command_from_city() {
    assert_eq!(
        parse_command("FROM city.csv\n"),
        Command::Operator(Operator::From(Dataset::City))
    );
}

/// Test well-formed input: `FROM country.csv`.
#[test]
fn test_parse_command_from_country() {
    assert_eq!(
        parse_command("FROM country.csv\n"),
        Command::Operator(Operator::From(Dataset::Country))
    );
}
