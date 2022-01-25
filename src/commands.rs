use crate::data::Dataset;
use crate::operators::Operator;

/// Commands parsed from user input.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// The user entered the `exit` command.
    Exit,
    /// The user entered the `help` command.
    Help,
    /// The chain of operators processed from the input.
    /// Example:
    /// 'FROM city.csv SELECT CityID TAKE 5' will parse to
    /// ```
    /// Command::Operator(
    ///   Operator::Take(
    ///     Box(Operator::Select(
    ///       Box(Operator::From(
    ///         Dataset::City
    ///       )),
    ///       "CityID"
    ///     )),
    ///     5
    ///   )
    /// )
    /// ```
    Operator(Operator),
    /// The user's input is erroneous.
    InputError(String),
    /// The user didn't enter anything so do nothing.
    NoInput,
}

/// Helper function to parse the token stream of the user input from the CLI into an [`Operator`]
/// chain.
///
/// # Arguments
/// `tokens` : The input string tokenized into a vector of strings to be processed.
///
/// # Usage: This function only processes the input tokens into a chain of [`Operator`]s.
/// The 'exit' and 'help' commands must be handled separetely. Use [`parse_command`] instead.
///
/// # Returns
/// A [`Command::Operator`] chain on successfully parsinig the tokens into [`Operator`]s or
/// [`Command::InputError`] in all other cases.
fn parse_operators(tokens: &Vec<&str>) -> Result<Operator, String> {
    let mut token_iter = tokens.into_iter();

    // This needs to be mutable as we will keep chaining operators onto the preceeding chain.
    let mut chain = None;

    while let Some(token) = token_iter.next() {
        chain = match *token {
            // Expected: FROM <["language.csv", "city.csv", "country.csv"]>
            "FROM" => {
                // FROM must always be the first command.
                if chain.is_some() {
                    return Err("FROM must always be the first operator.".to_string());
                } else {
                    // The token following FROM must be one of
                    // ["language.csv", "city.csv", "country.csv"]
                    match token_iter.next() {
                        Some(&"language.csv") => Some(Operator::From(Dataset::Language)),
                        Some(&"city.csv") => Some(Operator::From(Dataset::City)),
                        Some(&"country.csv") => Some(Operator::From(Dataset::Country)),
                        other => {
                            return Err(format!("Invalid argument to FROM: {:?}", other));
                        }
                    }
                }
            }
            // Expected: ... SELECT <comma_seperated_column_names>
            "SELECT" => match token_iter.next() {
                Some(columns) => {
                    if chain.is_none() {
                        return Err("SELECT can't be the first command; It must be preceded by at least a FROM.".to_string());
                    }

                    Some(Operator::Select {
                        chain: Box::new(chain.unwrap()),
                        column_names: columns
                            .split(",")
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    })
                }
                None => {
                    return Err("SELECT takes at least one column name to select on.".to_string());
                }
            },
            // Expected: ... TAKE <+ve number>
            "TAKE" => match token_iter.next() {
                Some(count) => {
                    if chain.is_none() {
                        // Early termination.
                        return Err("TAKE can't be the first command; It must be preceded by at least a FROM.".to_string());
                    }
                    Some(Operator::Take {
                        chain: Box::new(chain.unwrap()),
                        count: match str::parse::<usize>(count) {
                            Ok(count) => count,
                            Err(e) => {
                                return Err(format!(
                                    "Invalid value passed to TAKE operator: {}. Must be a positive integer.\n Full error message: {}",
                                    count, e.to_string()
                                ));
                            }
                        },
                    })
                }
                None => {
                    return Err("TAKE must be followed by the number of rows to take.".to_string());
                }
            },
            // Expected: ... ORDERBY <column_name>
            "ORDERBY" => match token_iter.next() {
                Some(column_name) => {
                    if chain.is_none() {
                        // Early termination.
                        return Err("ORDERBY can't be the first command; It must be preceded by at least a FROM.".to_string());
                    }
                    Some(Operator::OrderBy {
                        chain: Box::new(chain.unwrap()),
                        column: column_name.to_string(),
                    })
                }
                None => {
                    return Err(
                        "ORDERBY must be followed by the name of the column to order by."
                            .to_string(),
                    );
                }
            },
            // Expected: ... COUNTBY <column_name>
            "COUNTBY" => match token_iter.next() {
                Some(column_name) => {
                    if chain.is_none() {
                        // Early termination.
                        return Err("COUNTBY can't be the first command; It must be preceded by at least a FROM.".to_string());
                    }
                    Some(Operator::CountBy {
                        chain: Box::new(chain.unwrap()),
                        column: column_name.to_string(),
                    })
                }
                None => {
                    return Err(
                        "COUNTBY must be followed by the name of the column to count.".to_string(),
                    );
                }
            },
            // Expected: ... JOIN <["language.csv", "city.csv", "country.csv"]> <column_name>
            "JOIN" => {
                if chain.is_some() {
                    let dataset = match token_iter.next() {
                        Some(&"language.csv") => Dataset::Language,
                        Some(&"city.csv") => Dataset::City,
                        Some(&"country.csv") => Dataset::Country,
                        Some(str) => {
                            return Err(format!("Invalid dataset to JOIN on: {}", str));
                        }
                        None => {
                            return Err(
                                "JOIN must be followed by the dataset and the name of the column to join on."
                                    .to_string(),
                            );
                        }
                    };
                    let column_name = match token_iter.next() {
                        Some(column_name) => column_name,
                        None => {
                            return Err(
                                "JOIN must be followed by the dataset and the name of the column to join on."
                                    .to_string(),
                            );
                        }
                    };
                    Some(Operator::Join {
                        chain: Box::new(chain.unwrap()),
                        right: dataset,
                        column: column_name.to_string(),
                    })
                } else {
                    // Early termination.
                    return Err(
                        "JOIN can't be the first command; It must be preceded by at least a FROM."
                            .to_string(),
                    );
                }
            }
            _ => {
                // Early termination.
                return Err(format!("Invalid Input: {}", tokens.join(" ")));
            }
        };
    }

    if chain.is_some() {
        Ok(chain.unwrap())
    } else {
        Err(format!("Invalid Input: {}", tokens.join(" ")))
    }
}

/// Parses the command entered on the CLI into a [`Command`].
///
/// # Arguments
/// `input` : the input string to be processed.
///
/// # Returns
/// A [`Command`] that represents the parsed input.
pub fn parse_command(input: &str) -> Command {
    // Remove the trailing new line.
    match input.strip_suffix("\n") {
        Some(val) => match val {
            "help" => Command::Help,
            "exit" => Command::Exit,
            _ => {
                // Use split_whitespace to get rid of excess whitespace in the input.
                let tokens: Vec<&str> = val.split_whitespace().collect();
                if tokens.is_empty() {
                    Command::NoInput
                } else {
                    match parse_operators(&tokens) {
                        Ok(operator) => Command::Operator(operator),
                        Err(str) => Command::InputError(str),
                    }
                }
            }
        },
        None => Command::NoInput,
    }
}

/// Test for NULL input
#[test]
fn test_parse_command_no_input() {
    assert_eq!(parse_command("\n"), Command::NoInput);
}

/// Test 'exit' command as input
#[test]
fn test_parse_command_exit() {
    assert_eq!(parse_command("exit\n"), Command::Exit);
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed1() {
    assert_eq!(
        parse_command("FRM language.csv\n"),
        Command::InputError("Invalid Input: FRM language.csv".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed2() {
    assert_eq!(
        parse_command("TAKE language.csv\n"),
        Command::InputError(
            "TAKE can't be the first command; It must be preceded by at least a FROM.".to_string()
        )
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed3() {
    assert_eq!(
        parse_command("language.csv\n"),
        Command::InputError("Invalid Input: language.csv".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed4() {
    assert_eq!(
        parse_command("help FROM language.csv\n"),
        Command::InputError("Invalid Input: help FROM language.csv".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed5() {
    assert_eq!(
        parse_command("FROM ORDERBY CityPop TAKE 7 SELECT CityName,CityPop\n"),
        Command::InputError("Invalid argument to FROM: Some(\"ORDERBY\")".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed6() {
    assert_eq!(
        parse_command("FROM city.csv ORDERBY TAKE 7 SELECT CityName,CityPop\n"),
        Command::InputError(
            "Invalid Input: FROM city.csv ORDERBY TAKE 7 SELECT CityName,CityPop".to_string()
        )
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed7() {
    assert_eq!(
        parse_command("FROM city.csv ORDERBY CityPop TAKE SELECT CityName,CityPop\n"),
        Command::InputError("Invalid value passed to TAKE operator: SELECT. Must be a positive integer.\n Full error message: invalid digit found in string".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed8() {
    assert_eq!(
        parse_command("FROM city.csv ORDERBY CityPop TAKE 7 SELECT\n"),
        Command::InputError("SELECT takes at least one column name to select on.".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed9() {
    assert_eq!(
        parse_command("FROM city.csv TAKE -2\n"),
        Command::InputError("Invalid value passed to TAKE operator: -2. Must be a positive integer.\n Full error message: invalid digit found in string".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed10() {
    assert_eq!(
        parse_command("FROM city.csv TAKE CityID\n"),
        Command::InputError("Invalid value passed to TAKE operator: CityID. Must be a positive integer.\n Full error message: invalid digit found in string".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed11() {
    assert_eq!(
        parse_command("FROM city.cv\n"),
        Command::InputError("Invalid argument to FROM: Some(\"city.cv\")".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed12() {
    assert_eq!(
        parse_command("FROM cit.csv\n"),
        Command::InputError("Invalid argument to FROM: Some(\"cit.csv\")".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed13() {
    assert_eq!(
        parse_command("FROM lungage.csv\n"),
        Command::InputError("Invalid argument to FROM: Some(\"lungage.csv\")".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed14() {
    assert_eq!(
        parse_command("FROM contry.csv\n"),
        Command::InputError("Invalid argument to FROM: Some(\"contry.csv\")".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed15() {
    assert_eq!(
        parse_command("FROM city.csv JOIN country.csv\n"),
        Command::InputError(
            "JOIN must be followed by the dataset and the name of the column to join on."
                .to_string()
        )
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed16() {
    assert_eq!(
        parse_command("FROM city.csv JOIN CountryCode\n"),
        Command::InputError("Invalid dataset to JOIN on: CountryCode".to_string())
    );
}

/// Test malformed command as input
#[test]
fn test_parse_command_malformed17() {
    assert_eq!(
        parse_command("FROM city.csv JOIN country.csv CountryCode JOIN lnguage.csv CountryCode\n"),
        Command::InputError("Invalid dataset to JOIN on: lnguage.csv".to_string())
    );
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
/// Test well-formed input: "FROM city.csv ORDERBY CityPop TAKE 7 SELECT CityName,CityPop\n"
#[test]
fn test_parse_command_complex1() {
    assert_eq!(
        parse_command("FROM city.csv ORDERBY CityPop TAKE 7 SELECT CityName,CityPop\n"),
        Command::Operator(Operator::Select {
            chain: Box::new(Operator::Take {
                chain: Box::new(Operator::OrderBy {
                    chain: Box::new(Operator::From(Dataset::City)),
                    column: "CityPop".to_string()
                }),
                count: 7
            }),
            column_names: vec!["CityName".to_string(), "CityPop".to_string()]
        }),
    );
}
/// Test well-formed input: "FROM city.csv SELECT CityName\n"
#[test]
fn test_parse_command_complex2() {
    assert_eq!(
        parse_command("FROM city.csv SELECT CityName\n"),
        Command::Operator(Operator::Select {
            chain: Box::new(Operator::From(Dataset::City)),
            column_names: vec!["CityName".to_string()]
        })
    );
}

/// Test well-formed input: "FROM country.csv SELECT CountryCode,Continent,CountryPop\n"
#[test]
fn test_parse_command_complex3() {
    assert_eq!(
        parse_command("FROM country.csv SELECT CountryCode,Continent,CountryPop\n"),
        Command::Operator(Operator::Select {
            chain: Box::new(Operator::From(Dataset::Country)),
            column_names: vec![
                "CountryCode".to_string(),
                "Continent".to_string(),
                "CountryPop".to_string()
            ]
        }),
    );
}
/// Test well-formed input: "FROM city.csv TAKE 2\n"
#[test]
fn test_parse_command_complex4() {
    assert_eq!(
        parse_command("FROM city.csv TAKE 2\n"),
        Command::Operator(Operator::Take {
            chain: Box::new(Operator::From(Dataset::City)),
            count: 2
        }),
    );
}
/// Test well-formed input: "FROM city.csv ORDERBY CityPop TAKE 10\n"
#[test]
fn test_parse_command_complex5() {
    assert_eq!(
        parse_command("FROM city.csv ORDERBY CityPop TAKE 10\n"),
        Command::Operator(Operator::Take {
            chain: Box::new(Operator::OrderBy {
                chain: Box::new(Operator::From(Dataset::City)),
                column: "CityPop".to_string()
            }),
            count: 10
        }),
    );
}
/// Test well-formed input: "FROM city.csv JOIN country.csv CountryCode\n"
#[test]
fn test_parse_command_complex6() {
    assert_eq!(
        parse_command("FROM city.csv JOIN country.csv CountryCode\n"),
        Command::Operator(Operator::Join {
            chain: Box::new(Operator::From(Dataset::City)),
            right: Dataset::Country,
            column: "CountryCode".to_string()
        }),
    );
}
/// Test well-formed input: "FROM city.csv JOIN country.csv CountryCode JOIN language.csv
/// CountryCode\n"
#[test]
fn test_parse_command_complex7() {
    assert_eq!(
        parse_command("FROM city.csv JOIN country.csv CountryCode JOIN language.csv CountryCode\n"),
        Command::Operator(Operator::Join {
            chain: Box::new(Operator::Join {
                chain: Box::new(Operator::From(Dataset::City)),
                right: Dataset::Country,
                column: "CountryCode".to_string()
            }),
            right: Dataset::Language,
            column: "CountryCode".to_string()
        }),
    );
}
/// Test well-formed input: "FROM language.csv COUNTBY Language ORDERBY count TAKE 7\n"
#[test]
fn test_parse_command_complex8() {
    assert_eq!(
        parse_command("FROM language.csv COUNTBY Language ORDERBY count TAKE 7\n"),
        Command::Operator(Operator::Take {
            chain: Box::new(Operator::OrderBy {
                chain: Box::new(Operator::CountBy {
                    chain: Box::new(Operator::From(Dataset::Language)),
                    column: "Language".to_string()
                }),
                column: "count".to_string()
            }),
            count: 7
        }),
    );
}
