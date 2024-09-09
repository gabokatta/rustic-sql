use crate::utils::RusticSQLTest;

pub mod utils;

#[test]
fn test_insert_user_all_fields() {
    let test = RusticSQLTest::default();
    let query = "INSERT INTO users (user_id, name, email, age) VALUES (14, 'Solidus Snake', 'solidus.snake@mgs.com', 40)";
    let result = test.run_for(query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM users WHERE user_id = 14";
    test.assert_row(
        select_query,
        &["14", "Solidus Snake", "solidus.snake@mgs.com", "40"],
    )
}

#[test]
fn test_insert_nothing() {
    let test = RusticSQLTest::default();
    let query = "INSERT INTO users VALUES (14, 'Solidus Snake', 'solidus.snake@mgs.com', 40)";
    let result = test.run_for(query.to_string());
    print!("{:?}", &result);
    assert!(result.is_err());
}

#[test]
fn test_insert_multiple_rows() {
    let test = RusticSQLTest::default();
    let insert_query = "INSERT INTO users (user_id, name, email, age) VALUES (15, 'Raiden', 'raiden@mgs.com', 33), (16, 'Big Boss', 'big.boss@mgs.com', 45)";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM users WHERE user_id = 15";
    test.assert_row(select_query, &["15", "Raiden", "raiden@mgs.com", "33"]);
    let select_query = "SELECT * FROM users WHERE user_id = 16";
    test.assert_row(select_query, &["16", "Big Boss", "big.boss@mgs.com", "45"]);
}

#[test]
fn test_insert_missing_user_id_and_name() {
    let test = RusticSQLTest::default();
    let insert_query = "INSERT INTO users (email, age) VALUES ('liquid.snake@mgs.com', 35)";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM users WHERE email = 'liquid.snake@mgs.com'";
    test.assert_row(select_query, &["", "", "liquid.snake@mgs.com", "35"]);
}

#[test]
fn test_insert_invalid_column() {
    let test = RusticSQLTest::default();
    let insert_query = "INSERT INTO users (user_id, name, email, age, extra_column) VALUES (20, 'Raiden', 'raiden@mgs.com', 33, 'extra')";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_err_and(|e| e.to_string().contains("exist")));
}

#[test]
fn test_insert_invalid_table() {
    let test = RusticSQLTest::default();
    let insert_query =
        "INSERT INTO kojima (user_id, name, email, age) VALUES (21, 'Quiet', 'quiet@mgs.com', 27)";
    let result = test.run_for(insert_query.to_string());
    assert!(result.is_err_and(|e| e.to_string().contains("table")));
}
