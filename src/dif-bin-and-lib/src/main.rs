pub const CONFIG_ENV: &str = "POEM_ENV";
use std::env;
fn main() {
    match env::var(CONFIG_ENV) {
        Ok(s) => {
            if s == "d".to_string() {
                println!("{:?}", "development model");
            } else if s == "s".to_string() {
                println!("{:?}", "staging model");
            } else if s == "p".to_string() {
                println!("{:?}", "production model");
            } else {
                println!("{:?}", "invalid model");
            }
        }
        _ => println!("{:?}", "development model"),
    }

    pub enum Model {
        Integer(i32, i32),
    }

    let m = Model::Integer(42, 43);

    println!("{:?}", m);
}
