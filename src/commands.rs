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

impl Command {
    fn is_operator(&self) -> bool {
        match self {
            Command::Operator(_) => true,
            _ => false,
        }
    }
    fn take_operator(&self) -> Option<Operator> {
        match self {
            Command::Operator(operator) => Some(operator.clone()),
            _ => None,
        }
    }
}

fn process_tokens(tokens: &Vec<&str>) -> Command {
    let mut token_iter = tokens.into_iter();
    let mut command = Command::InputError;
    while let Some(token) = token_iter.next() {
        command = match *token {
            "FROM" => match token_iter.next() {
                Some(&"language.csv") => Command::Operator(Operator::From(Dataset::Language)),
                Some(&"city.csv") => Command::Operator(Operator::From(Dataset::City)),
                Some(&"country.csv") => Command::Operator(Operator::From(Dataset::Country)),
                _ => Command::InputError,
            },
            "SELECT" => match token_iter.next() {
                Some(columns) if command.is_operator() => Command::Operator(Operator::Select {
                    chain: Box::new(command.take_operator().unwrap()),
                    column_names: columns
                        .split(",")
                        .filter(|s| !s.is_empty())
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                }),
                _ => Command::InputError,
            },
            "TAKE" => match token_iter.next() {
                Some(count) if command.is_operator() => Command::Operator(Operator::Take {
                    chain: Box::new(command.take_operator().unwrap()),
                    count: str::parse::<usize>(count).expect(&format!(
                        "Invalid value passed to TAKE operator: {}. Must be a positive integer.",
                        count
                    )),
                }),
                _ => Command::InputError,
            },
            "ORDERBY" => match token_iter.next() {
                Some(column_name) if command.is_operator() => {
                    Command::Operator(Operator::OrderBy {
                        chain: Box::new(command.take_operator().unwrap()),
                        column: column_name.to_string(),
                    })
                }
                _ => Command::InputError,
            },
            "COUNTBY" => match token_iter.next() {
                Some(column_name) if command.is_operator() => {
                    Command::Operator(Operator::CountBy {
                        chain: Box::new(command.take_operator().unwrap()),
                        column: column_name.to_string(),
                    })
                }
                _ => Command::InputError,
            },
            _ => command,
        };
    }
    command
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
                let tokens: Vec<&str> = val.split_whitespace().collect();
                if tokens.is_empty() || tokens[0] != "FROM" {
                    Command::InputError
                } else {
                    process_tokens(&tokens)
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
