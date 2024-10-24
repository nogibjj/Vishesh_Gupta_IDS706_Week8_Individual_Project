use vishesh_gupta::{extract, query, load};

#[test]
fn test_extract() {
    let url = "https://raw.githubusercontent.com/footballcsv/england/refs/heads/master/2010s/2010-11/eng.1.csv";
    let file_path = "data/match_results.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_load() {
    let dataset = "data/match_results.csv";
    let result = load(dataset);

    assert_eq!(result.unwrap(), "MatchResultsDB.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM MatchResultsDB WHERE round = 1;";
    let result = query(select_query);

    assert!(result.is_ok());
}