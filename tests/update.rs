use crate::utils::RusticSQLTest;

pub mod utils;

#[test]
fn test_update_pokemon_with_empty_name() {
    let test = RusticSQLTest::default();
    let update_query = "UPDATE pokemon SET name = '' WHERE id = 1";
    let result = test.run_for(update_query.to_string());
    assert!(result.is_ok());
    let select_query = "SELECT * FROM pokemon WHERE id = 1";
    let expected_row = ["1", "", "Electric", "25"];
    test.assert_row(select_query, &expected_row);
}

#[test]
fn test_update_all_pokemon_to_same_level() {
    let test = RusticSQLTest::default();
    let update_query = "UPDATE pokemon SET level = 50, name = '', type = ''";
    let result = test.run_for(update_query.to_string());
    assert!(result.is_ok());
    for id in 1..=10 {
        let select_query = format!("SELECT * FROM pokemon WHERE id = {}", id);
        let expected_row = [&id.to_string(), "", "", "50"];
        test.assert_row(&select_query, &expected_row);
    }
}

#[test]
fn test_update_non_existent_type() {
    let test = RusticSQLTest::default();
    let update_query = "UPDATE pokemon SET level = 50 WHERE type = 'Mythical'";
    test.verify_no_changes("pokemon.csv".to_string(), update_query);
}

#[test]
fn test_missing_set_in_query() {
    let test = RusticSQLTest::default();
    let update_query = "UPDATE pokemon level = 50'";
    let result = test.run_for(update_query.to_string());
    assert!(result.is_err());
}

#[test]
fn test_update_invalid_column() {
    let test = RusticSQLTest::default();
    let update_query = "UPDATE pokemon SET hp = 80 WHERE type = 'Fire'";
    let result = test.run_for(update_query.to_string());
    assert!(result.is_err_and(|e| e.to_string().contains("hp")));
}
