use crate::utils::RusticSQLTest;

mod utils;

fn verify_insert(test: &RusticSQLTest, query: &str, expected_row: &[&str]) {
    let expected_row_str = expected_row.join(",");
    let select_result = test.run_and_get_rows(query.to_string());
    assert_eq!(select_result[1], expected_row_str);
}

#[test]
fn test_insert_user_all_fields() {
    let test = RusticSQLTest::new();
    let query = "INSERT INTO users (user_id, name, email, age) VALUES (14, 'Solidus Snake', 'solidus.snake@mgs.com', 40)";
    let result = test.run_for(query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM users WHERE user_id = 14";
    verify_insert(
        &test,
        select_query,
        &["14", "Solidus Snake", "solidus.snake@mgs.com", "40"],
    )
}

#[test]
fn test_insert_multiple_rows() {
    let test = RusticSQLTest::new();
    let insert_query = "INSERT INTO users (user_id, name, email, age) VALUES (15, 'Raiden', 'raiden@mgs.com', 33), (16, 'Big Boss', 'big.boss@mgs.com', 45)";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM users WHERE user_id = 15";
    verify_insert(
        &test,
        select_query,
        &["15", "Raiden", "raiden@mgs.com", "33"],
    );
    let select_query = "SELECT * FROM users WHERE user_id = 16";
    verify_insert(
        &test,
        select_query,
        &["16", "Big Boss", "big.boss@mgs.com", "45"],
    );
}

#[test]
fn test_insert_missing_user_id_and_name() {
    let test = RusticSQLTest::new();
    let insert_query = "INSERT INTO users (email, age) VALUES ('liquid.snake@mgs.com', 35)";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM users WHERE email = 'liquid.snake@mgs.com'";
    verify_insert(&test, select_query, &["", "", "liquid.snake@mgs.com", "35"]);
}

#[test]
fn test_insert_invalid_column() {
    let test = RusticSQLTest::new();
    let insert_query = "INSERT INTO users (user_id, name, email, age, extra_column) VALUES (20, 'Raiden', 'raiden@mgs.com', 33, 'extra')";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_err_and(|e| e.to_string().contains("exist")));
}

#[test]
fn test_insert_invalid_table() {
    let test = RusticSQLTest::new();
    let insert_query =
        "INSERT INTO kojima (user_id, name, email, age) VALUES (21, 'Quiet', 'quiet@mgs.com', 27)";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_err_and(|e| e.to_string().contains("table")));
}
