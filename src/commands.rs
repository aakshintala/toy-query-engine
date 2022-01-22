// The datasets known to the toy-query-engine.
#[derive(Debug, Clone, PartialEq)]
pub enum Dataset {
    // city.csv
    City,
    // country.csv
    Country,
    // language.csv
    Language,
}

// Commands parsed from user input.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    // The user entered the `exit` command.
    Exit,
    // The user entered the `help` command.
    Help,
    // The user entered a FROM <dataset> operator.
    From(Dataset),
    // The user's input is erroneous.
    InputError,
}

/// Parses the command entered on the CLI into a vector of [`Command`]s.
///
/// # Arguments
/// `input` : the input string to be processed.
///
/// # Returns
/// Vector of [`Command`]s parsed from the input.
pub fn parse_commands(input: &str) -> Vec<Command> {
    match input.strip_suffix("\n") {
        Some(val) => match val {
            "help" => vec![Command::Help],
            "exit" => vec![Command::Exit],
            _ => {
                let tokens: Vec<&str> = val.split(" ").into_iter().map(|s| s).collect();
                if tokens.is_empty() || tokens[0] != "FROM" {
                    vec![Command::InputError]
                } else if tokens[0] == "FROM" && tokens.len() > 1 {
                    match tokens[1] {
                        "language.csv" => vec![Command::From(Dataset::Language)],
                        "city.csv" => vec![Command::From(Dataset::City)],
                        "country.csv" => vec![Command::From(Dataset::Country)],
                        _ => vec![Command::InputError],
                    }
                } else {
                    vec![Command::InputError]
                }
            }
        },
        None => vec![Command::InputError],
    }
}

/// Test for NULL input
#[test]
fn test_parse_commands_no_input() {
    let commands = parse_commands("\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::InputError);
}

/// Test 'exit' command as input
#[test]
fn test_parse_commands_exit() {
    let commands = parse_commands("exit\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::Exit);
}

/// Test malformed command as input
#[test]
fn test_parse_commands_malformed1() {
    let commands = parse_commands("FRM language.csv\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::InputError);
}

/// Test malformed command as input
#[test]
fn test_parse_commands_malformed2() {
    let commands = parse_commands("TAKE language.csv\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::InputError);
}

/// Test malformed command as input
#[test]
fn test_parse_commands_malformed3() {
    let commands = parse_commands("language.csv\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::InputError);
}

/// Test 'help'command as input
#[test]
fn test_parse_commands_help() {
    let commands = parse_commands("help\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::Help);
}

/// Test well-formed input: `FROM language.csv`.
#[test]
fn test_parse_commands_from_language() {
    let commands = parse_commands("FROM language.csv\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::From(Dataset::Language));
}

/// Test well-formed input: `FROM city.csv`.
#[test]
fn test_parse_commands_from_city() {
    let commands = parse_commands("FROM city.csv\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::From(Dataset::City));
}

/// Test well-formed input: `FROM country.csv`.
#[test]
fn test_parse_commands_from_country() {
    let commands = parse_commands("FROM country.csv\n");
    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0], Command::From(Dataset::Country));
}
