pub fn get_stripe_client() -> stripe::Client {
    let stripe_secret = dotenv::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

    stripe::Client::new(stripe_secret)
}

// #[derive(Serialize, Default, Debug)]
// pub struct SearchParams {
//     pub query: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub limit: Option<u64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub page: Option<u64>,
// }

// pub async fn stripe_search<R: DeserializeOwned + 'static + Send>(
//     client: &stripe::Client,
//     resource: &str,
//     params: SearchParams,
// ) -> Result<List<R>, stripe::StripeError> {
//     client
//         .get_query(&format!("/{}/search", resource), &params)
//         .await
// }
