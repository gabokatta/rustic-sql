use crate::utils::RusticSQLTest;

mod utils;

#[test]
fn test_empty_query() {
    let test = RusticSQLTest::new();
    let result = test.run_with_output("".to_string());
    let output = String::from_utf8(result.stdout).unwrap();
    assert!(output.contains("empty"))
}
