use super::variables::SECRET_VARIABLES;

pub fn get_stripe_client() -> stripe::Client {
    stripe::Client::new(&SECRET_VARIABLES.stripe_secret_key)
}
