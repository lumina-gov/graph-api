use std::env;

pub fn init_non_secret_variables() {
    let in_prod = dotenv::var("production").ok().unwrap_or(String::from("false")) == "true";

    env::set_var("LIGHT_UNIVERSITY_PRICE_ID", match in_prod {
        true => "price_1MnbOyJRb0ozzDydaCZxbuvY",
        false => "price_1Mc2OQJRb0ozzDydL7R86kGy",
    });
}