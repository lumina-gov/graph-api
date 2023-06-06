use lazy_static::lazy_static;
use openai::set_key;

pub struct SecretVariables {
    pub jwt_secret: Vec<u8>,
    pub sendgrid_api_key: String,
    pub light_university_product_id: String,
    pub stripe_secret_key: String,
    pub database_url: Option<String>,
    pub app_secret: String,
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
                .expect("JWT_SECRET is not set in env variables")
                .into_bytes(),
            sendgrid_api_key: dotenv::var("SENDGRID_KEY")
                .expect("SENDGRID_KEY is not set in env variables"),
            light_university_product_id: match in_prod {
                true => "price_1MnbOyJRb0ozzDydaCZxbuvY",
                false => "price_1Mc2OQJRb0ozzDydL7R86kGy",
            }
            .into(),
            stripe_secret_key: dotenv::var("STRIPE_SECRET_KEY")
                .expect("STRIPE_SECRET_KEY is not set in env variables"),
            database_url: dotenv::var("DATABASE_URL").ok(),
            app_secret: dotenv::var("LUMINA_APP_SECRET")
                .expect("LUMINA_APP_SECRET is not set in env variables"),
        }
    };
}

#[ignore]
#[test]
fn generate_app_secret() {
    // base64 encode 80 random bytes
    use base64::Engine;
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 80];
    rng.fill_bytes(&mut bytes);
    let secret = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&bytes);
    println!("LUMINA_APP_SECRET={}", secret);
}
