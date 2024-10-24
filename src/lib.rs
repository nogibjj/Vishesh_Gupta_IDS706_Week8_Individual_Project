use reqwest::blocking::Client;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";

fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

pub fn extract(url: &str, file_path: &str, directory: &str) {
    if !fs::metadata(directory).is_ok() {
        fs::create_dir_all(directory).expect("Failed to create directory");
    }

    let client = Client::new();
    let mut response = client.get(url).send().expect("Failed to send request");
    let mut file = fs::File::create(file_path).expect("Failed to create file");


    std::io::copy(&mut response, &mut file).expect("Failed to copy content");

    println!("Extraction successful!");
}

pub fn load(dataset: &str) -> Result<String> {
    let conn = Connection::open("MatchResultsDB.db")?;

    conn.execute("DROP TABLE IF EXISTS MatchResultsDB", [])?;

    conn.execute(
        "CREATE TABLE MatchResultsDB (
            Round INTEGER,
            Date TEXT,
            'Team 1' TEXT,
            'Team 2' TEXT,
            FT TEXT
        )",
        [],
    )?;

    let mut rdr = csv::Reader::from_path(dataset).expect("Failed to read dataset");

    let mut stmt = conn.prepare(
        "INSERT INTO MatchResultsDB (
            Round, 
            Date, 
            'Team 1', 
            'Team 2', 
            FT
        ) 
        VALUES (?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(&[
                    &record[0], // Round
                    &record[1], // Date
                    &record[2], // Team 1
                    &record[3], // Team 2
                    &record[4], // FT (Full-Time Score)
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    Ok("MatchResultsDB.db".to_string())
}

pub fn query(query: &str) -> Result<()> {
    let conn = Connection::open("MatchResultsDB.db")?;
    // Read operation
    if query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(query)?;
        let results = stmt.query_map(params![], |row| {
            Ok((
                row.get::<usize, i32>(0)?,    // Round
                row.get::<usize, String>(1)?, // Date
                row.get::<usize, String>(2)?, // Team 1
                row.get::<usize, String>(3)?, // Team 2
                row.get::<usize, String>(4)?, // FT
            ))
        })?;

        for result in results {
            match result {
                Ok((round, date, team1, team2, ft)) => {
                    println!(
                        "Result: round={}, date={}, team1={}, team2={}, ft={}",
                        round, date, team1, team2, ft
                    );
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }
    } else {
        // other CUD operations
        let _num_affected_rows = conn.execute_batch(query)?;
    }
    log_query(query, LOG_FILE);
    Ok(())
}
