use crate::utils::RusticSQLTest;

pub mod utils;

#[test]
fn test_empty_query() {
    let test = RusticSQLTest::default();
    let result = test.run_for("".to_string());
    assert!(result.is_err_and(|e| e.to_string().contains("empty")));
}

#[test]
fn test_select_no_where() {
    let test = RusticSQLTest::default();
    let query = "SELECT * FROM users";
    let expected_rows = 11; //with header
    let result = test.run_and_get_rows(query.to_string());
    assert_eq!(expected_rows, result.len());
}

#[test]
fn test_select_where_with_order_by() {
    let test = RusticSQLTest::default();
    let query = "SELECT name, email FROM users WHERE age > 30 ORDER BY age DESC";

    let expected_rows: Vec<String> = [
        vec!["Bob Brown", "bob.brown@example.com"],
        vec!["Frank Miller", "frank.miller@example.com"],
        vec!["Henry Clark", "henry.clark@example.com"],
        vec!["Jane Smith", "jane.smith@example.com"],
        vec!["Eve Adams", "eve.adams@example.com"],
        vec!["Charlie Davis", "charlie.davis@example.com"],
    ]
    .iter()
    .map(|r| r.join(","))
    .collect();
    let expected_header: String = ["name", "email"].join(",");
    let result = test.run_and_get_rows(query.to_string());

    assert_eq!(expected_header, result[0]);
    assert_eq!(expected_rows, result[1..]);
}

#[test]
fn test_select_with_nested_where() {
    let test = RusticSQLTest::default();
    let query = "SELECT user_id, name FROM users WHERE age > 30 AND (user_id < 8 OR name = 'Henry Clark') ORDER BY name";

    let expected_rows: Vec<String> = [
        vec!["4", "Bob Brown"],
        vec!["5", "Charlie Davis"],
        vec!["7", "Eve Adams"],
        vec!["10", "Henry Clark"],
        vec!["2", "Jane Smith"],
    ]
    .iter()
    .map(|r| r.join(","))
    .collect();
    let expected_header: String = ["user_id", "name"].join(",");

    let result = test.run_and_get_rows(query.to_string());
    assert_eq!(expected_header, result[0]);
    assert_eq!(expected_rows, result[1..]);
}

#[test]
fn test_select_with_invalid_order_field() {
    let test = RusticSQLTest::default();
    let query = "SELECT user_id, name FROM users ORDER BY psn_id";
    let result = test.run_for(query.to_string());
    assert!(result.is_err_and(|x| x.to_string().contains("exist")))
}

#[test]
fn test_select_with_where_with_invalid_field() {
    let test = RusticSQLTest::default();
    let query = "SELECT user_id, name FROM users WHERE psn_id = 5";
    let result = test.run_for(query.to_string());
    assert!(result.is_err_and(|x| x.to_string().contains("exist")))
}

#[test]
fn test_select_with_invalid_table() {
    let test = RusticSQLTest::default();
    let query = "SELECT user_id, name FROM users2 WHERE user_id = 5";
    let result = test.run_for(query.to_string());
    assert!(result.is_err_and(|x| x.to_string().contains("exist")))
}
