use rustic_sql::run;
use rustic_sql::utils::files::get_temp_id;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct RusticSQLTest {
    temp_dir: PathBuf,
}

impl RusticSQLTest {
    pub fn assert_row(&self, query: &str, expected_row: &[&str]) {
        let expected_row_str = expected_row.join(",");
        let select_result = self.run_and_get_rows(query.to_string());
        if select_result.len() == 1 {
            assert_eq!("", expected_row_str);
        } else {
            assert_eq!(select_result[1], expected_row_str);
        }
    }

    fn read_table_to_string(&self, table: &String) -> String {
        let file = File::open(self.temp_dir.join(table)).unwrap();
        let reader = BufReader::new(file);
        let mut content = String::new();
        for line in reader.lines() {
            content.push_str(&line.unwrap());
            content.push('\n');
        }
        content
    }

    pub fn verify_no_changes(&self, table: String, query: &str) {
        let before_query = self.read_table_to_string(&table);
        let result = self.run_for(query.to_string());
        if result.is_err() {
            panic!("error executing query");
        }
        let after_query = self.read_table_to_string(&table);
        assert_eq!(before_query, after_query)
    }

    pub fn tear_down(&self) {
        fs::remove_dir_all(&self.temp_dir).expect("failed to clean up test directory");
    }

    pub fn args_for(&self, query: String) -> Vec<String> {
        vec![
            "target/debug/rustic-sql".to_string(),
            self.temp_dir.to_str().unwrap().to_string(),
            query,
        ]
    }

    pub fn run_for(&self, query: String) -> Result<(), Box<dyn Error>> {
        run(self.args_for(query))
    }

    pub fn run_and_get_rows(&self, query: String) -> Vec<String> {
        let args = self.args_for(query);
        let output = Command::new(&args[0])
            .arg(&args[1])
            .arg(&args[2])
            .output()
            .unwrap();
        let raw = String::from_utf8(output.stdout).unwrap();
        raw.trim()
            .split("\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }
}

impl Default for RusticSQLTest {
    fn default() -> Self {
        let path = "./tests/temp_tables_".to_string() + &get_temp_id().to_string();
        let temp_dir = Path::new(&path);
        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir).expect("failed to clean previous test tables.");
        }
        fs::create_dir(temp_dir).expect("failed to create temp table directory.");

        let og_tables_path = Path::new("./tests/integration_tables");
        let pokemons = og_tables_path.join("pokemon.csv");
        let users = og_tables_path.join("users.csv");
        let people = og_tables_path.join("people.csv");

        let temp_orders = temp_dir.join("pokemon.csv");
        let temp_users = temp_dir.join("users.csv");
        let temp_people = temp_dir.join("people.csv");

        fs::copy(pokemons, &temp_orders).expect("failed to copy order table.");
        fs::copy(users, &temp_users).expect("failed to copy user table.");
        fs::copy(people, &temp_people).expect("failed to copy people table.");

        RusticSQLTest {
            temp_dir: temp_dir.to_path_buf(),
        }
    }
}

impl Drop for RusticSQLTest {
    fn drop(&mut self) {
        self.tear_down()
    }
}
