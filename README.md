# Vishesh_Gupta_IDS706_Week8_INDIVIDUAL_PROJECT

[![Rust CI/CD Pipeline](https://github.com/nogibjj/Vishesh_Gupta_IDS706_Week8_Individual_Project/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Vishesh_Gupta_IDS706_Week8_Individual_Project/actions/workflows/cicd.yml)

## Project Overview

This project demonstrates a Rust-based command-line interface (CLI) application integrated with an SQLite database. It supports basic CRUD (Create, Read, Update, Delete) operations on the database. Additionally, this project showcases an optimized Rust binary generated through GitLab CI, with automated testing, building, and linting processes. Throughout the development, a Large Language Model (LLM) was utilized to enhance coding efficiency and support Rust's syntax understanding.

```
Vishesh_Gupta_IDS706_Week8_Individual_Project/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   └── workflows/cicd.yml
├── .gitignore
├── data/
│   └── match_results.csv
├── Makefile
├── src/
│   ├── lib.rs
│   ├── main.rs
├── Cargo.toml
├── README.md
├── query_log.md
└── MatchResultsDB.db
```

## 2. **CRUD Operations**
We demonstrate the following CRUD operations:
- **Create:** Insert new records into a table.
- **Read:** Query and retrieve records from the table.
- **Update:** Modify existing records within the table.
- **Delete:** Remove records from the table.

### Set up and Running Rust

#### First, compile the Rust project by running:
```bash
cargo build
```

#### Now we can Run the project by:
- Extract Data: To run the data extraction process, execute:
```bash
cargo run extract
```
- Load Data: To load the extracted data into the SQLite database, run:
```bash
cargo run load
```
Query Data: To query the data, use the following command. Replace [query] with your SQL query:
```bash
cargo run query "[query]"
```
These commands will allow you to extract, load, and query data within the SQLite database using the CLI.

#### We can see how it operates in the screenshots below

![Test Image 1](Rust_build.png)

![Test Image 2](Rust_Crud.png)

![Test Image 3](Rust_Delete.png)

## Youtube Video
