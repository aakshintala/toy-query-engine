use assert_cmd::Command;

#[test]
fn test_exit_cmd() {
    Command::cargo_bin("toy-query-engine")
        .unwrap()
        .write_stdin("exit\n")
        .assert()
        .stdout("Toy Query Engine v0.1\nEnter your query, or 'help' for more information or 'exit' to exit.\nGoodbye!\n");
}

#[test]
fn test_help_cmd() {
    Command::cargo_bin("toy-query-engine")
        .unwrap()
        .write_stdin("help\nexit\n")
        .assert()
        .stdout("Toy Query Engine v0.1\nEnter your query, or \'help\' for more information or \'exit\' to exit.\nAvailable Commands: \n\n      FROM <dataset> - Loads the `dataset`. \n\n          Maybe chained with other commands. If no other command is specified, will print the `dataset`. \n\n      SELECT <column-name> - used to select particular columns from the specified dataset. \n\n          See the Datasets section below for a list of column-names for each dataset. \n\n      TAKE <number> - Specifies the number of rows to print from the dataset. \n\n          <number> must be greater than or equal to 0. \n\n      ORDERBY <numeric-column-name> - Sorts the loaded dataset by the column-name in descending order, if the column contains numeric values. \n\n          See the Datasets section below for a list of acceptable values for <numeric-column-name> for each dataset. \n\n      COUNTBY <column-name> - Returns the . \n\n          <number> must be greater than or equal to 0. \n\n      JOIN <dataset> <column-name> - performs a join on the current dataset and the one specified in this command on <column-name>. \n\n          See the Datasets section below for a list of available datasets and the column-names for each dataset. \n\n          The provided <column-name> must be present in both datasets. \n\n    \n\n    Available Datasets\n\n      <dataset> : city.csv\n\n          <column-name> : [CityID, CityName, CountryCode, CityPop]\n\n          <numeric-column-name> : [CityID, CityPop]\n\n      <dataset> : country.csv\n\n          <column-name> : [CountryCode, CountryName, Continent, CountryPop, Capital]\n\n          <numeric-column-name> : [CountryPop, Capital]\n\n      <dataset> : language.csv\n\n          <column-name> : [CountryCode,Language]\n\n          <numeric-column-name> : []\n\nGoodbye!\n");
}

#[test]
fn test_from_take_5_cmd() {
    Command::cargo_bin("toy-query-engine")
        .unwrap()
        .write_stdin("FROM language.csv TAKE 5\nexit\n")
        .assert()
        .stdout("Toy Query Engine v0.1\nEnter your query, or \'help\' for more information or \'exit\' to exit.\nCountryCode,Language\nABW,Dutch\nABW,English\nABW,Papiamento\nABW,Spanish\nAFG,Balochi\n\nGoodbye!\n");
}

#[test]
fn test_from_take_10_cmd() {
    Command::cargo_bin("toy-query-engine")
        .unwrap()
        .write_stdin("FROM language.csv TAKE 10\nexit\n")
        .assert()
        .stdout("Toy Query Engine v0.1\nEnter your query, or \'help\' for more information or \'exit\' to exit.\nCountryCode,Language\nABW,Dutch\nABW,English\nABW,Papiamento\nABW,Spanish\nAFG,Balochi\nAFG,Dari\nAFG,Pashto\nAFG,Turkmenian\nAFG,Uzbek\nAGO,Ambo\n\nGoodbye!\n");
}

#[test]
fn test_from_countby_cmd() {
    Command::cargo_bin("toy-query-engine")
        .unwrap()
        .write_stdin("FROM city.csv ORDERBY CityPop TAKE 10\nexit\n")
        .assert()
        .stdout("Toy Query Engine v0.1\nEnter your query, or \'help\' for more information or \'exit\' to exit.\nCityID,CityName,CountryCode,CityPop\n1024,Mumbai_(Bombay),IND,10500000\n2331,Seoul,KOR,9981619\n206,Sâ€žo_Paulo,BRA,9968485\n1890,Shanghai,CHN,9696300\n939,Jakarta,IDN,9604900\n2822,Karachi,PAK,9269265\n3357,Istanbul,TUR,8787958\n2515,Ciudad_de_MÃˆxico,MEX,8591309\n3580,Moscow,RUS,8389200\n3793,New_York,USA,8008278\n\nGoodbye!\n");
}

#[test]
fn test_join_cmd() {
    Command::cargo_bin("toy-query-engine")
        .unwrap()
        .write_stdin("FROM city.csv JOIN country.csv CountryCode TAKE 10\nexit\n")
        .assert()
        .stdout("Toy Query Engine v0.1\nEnter your query, or \'help\' for more information or \'exit\' to exit.\nCityID,CityName,CountryCode,CityPop,CountryName,Continent,CountryPop,Capital\n1,Kabul,AFG,1780000,Afghanistan,Asia,22720000,1\n2,Qandahar,AFG,237500,Afghanistan,Asia,22720000,1\n3,Herat,AFG,186800,Afghanistan,Asia,22720000,1\n4,Mazar-e-Sharif,AFG,127800,Afghanistan,Asia,22720000,1\n5,Amsterdam,NLD,731200,Netherlands,Europe,15864000,5\n6,Rotterdam,NLD,593321,Netherlands,Europe,15864000,5\n7,Haag,NLD,440900,Netherlands,Europe,15864000,5\n8,Utrecht,NLD,234323,Netherlands,Europe,15864000,5\n9,Eindhoven,NLD,201843,Netherlands,Europe,15864000,5\n10,Tilburg,NLD,193238,Netherlands,Europe,15864000,5\n\nGoodbye!\n");
}
