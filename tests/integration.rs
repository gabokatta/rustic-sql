use crate::utils::RusticSQLTest;

mod utils;

#[test]
fn test_empty_query() {
    let test = RusticSQLTest::new();
    let result = test.run_for("".to_string());
    assert!(result.is_err_and(|e| e.to_string().contains("empty")));
}

#[test]
fn test_select_no_where() {
    let test = RusticSQLTest::new();
    let query = "SELECT * FROM users";
    let expected_rows = 11; //with header
    let result = test.run_and_get_rows(query.to_string());
    assert_eq!(expected_rows, result.len());
}
