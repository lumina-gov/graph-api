pub fn get_stripe_client() -> stripe::Client {
    let stripe_secret = dotenv::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

    stripe::Client::new(stripe_secret)
}
