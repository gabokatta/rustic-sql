use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub struct RusticSQLTest {
    temp_dir: PathBuf,
}

impl RusticSQLTest {
    pub fn new() -> Self {
        let temp_dir = Path::new("./tests/temp_tables");
        if temp_dir.exists() {
            fs::remove_dir_all(temp_dir).expect("failed to clean previous test tables.");
        }
        fs::create_dir(temp_dir).expect("failed to create temp table directory.");

        let og_tables_path = Path::new("./tests/integration_tables");
        let orders = og_tables_path.join("orders.csv");
        let users = og_tables_path.join("users.csv");

        let temp_orders = temp_dir.join("orders.csv");
        let temp_users = temp_dir.join("users.csv");

        fs::copy(orders, &temp_orders).expect("failed to copy order table.");
        fs::copy(users, &temp_users).expect("failed to copy user table.");

        RusticSQLTest {
            temp_dir: temp_dir.to_path_buf(),
        }
    }

    pub fn args_for(&self, query: String) -> Vec<String> {
        vec![
            "target/debug/rustic-sql".to_string(),
            self.temp_dir.to_str().unwrap().to_string(),
            query,
        ]
    }

    pub fn run_with_output(&self, query: String) -> Output {
        let args = self.args_for(query);
        Command::new(&args[0])
            .arg(&args[1])
            .arg(&args[2])
            .output()
            .unwrap()
    }
}

impl Drop for RusticSQLTest {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.temp_dir).expect("Failed to clean up test directory");
    }
}
