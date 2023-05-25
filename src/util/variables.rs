use lazy_static::lazy_static;
use openai::set_key;

pub struct SecretVariables {
    pub jwt_secret: Vec<u8>,
    pub sendgrid_api_key: String,
    pub light_university_product_id: String,
    pub stripe_secret_key: String,
    pub database_url: Option<String>,
}

lazy_static! {
    pub static ref SECRET_VARIABLES: SecretVariables = {
        dotenv::dotenv().ok();

        set_key(dotenv::var("OPENAI_KEY").expect("OPENAI_KEY not set in .env"));

        let in_prod = dotenv::var("PRODUCTION")
            .ok()
            .unwrap_or(String::from("false"))
            == "true";

        SecretVariables {
            jwt_secret: dotenv::var("JWT_SECRET")
                .expect("JWT_SECRET is not set in .env")
                .into_bytes(),
            sendgrid_api_key: dotenv::var("SENDGRID_KEY")
                .expect("SENDGRID_API_KEY is not set in .env"),
            light_university_product_id: match in_prod {
                true => "price_1MnbOyJRb0ozzDydaCZxbuvY",
                false => "price_1Mc2OQJRb0ozzDydL7R86kGy",
            }
            .into(),
            stripe_secret_key: dotenv::var("STRIPE_SECRET_KEY")
                .expect("STRIPE_SECRET_KEY is not set in .env"),
            database_url: dotenv::var("DATABASE_URL").ok(),
        }
    };
}
