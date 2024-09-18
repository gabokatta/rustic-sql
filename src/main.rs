use rustic_sql::run;
use std::env;

fn main() {
    let args = env::args().collect();
    if let Err(e) = run(args) {
        println!("{}", e);
    }
}
