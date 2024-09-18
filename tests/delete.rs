use crate::utils::RusticSQLTest;

pub mod utils;

#[test]
fn test_delete_single_row() {
    let test = RusticSQLTest::default();
    let delete_query = "DELETE FROM pokemon WHERE id = 1";
    let result = test.run_for(delete_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM pokemon WHERE id = 1";
    test.assert_row(select_query, &[]);
}

#[test]
fn test_delete_multiple_rows() {
    let test = RusticSQLTest::default();
    let delete_query = "DELETE FROM pokemon WHERE type != 'Electric'";
    let result = test.run_for(delete_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM pokemon WHERE type != 'Electric'";
    test.assert_row(select_query, &[]);
}

#[test]
fn test_delete_all() {
    let test = RusticSQLTest::default();
    let delete_query = "DELETE FROM pokemon";
    let result = test.run_for(delete_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM pokemon'";
    test.assert_row(select_query, &[]);
}

#[test]
fn test_delete_none() {
    let test = RusticSQLTest::default();
    let delete_query = "DELETE FROM pokemon WHERE type = 'Sound'";
    test.verify_no_changes("pokemon.csv".to_string(), delete_query);
}
