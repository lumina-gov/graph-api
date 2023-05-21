#[derive(Default)]
pub struct BaseQuery;
use crate::util::crack_seconds::CrackSeconds;
use async_graphql::Object;
use zxcvbn::time_estimates::CrackTimeSeconds;

#[Object(rename_fields = "snake_case", rename_args = "snake_case")]
impl BaseQuery {
    async fn ping(&self) -> String {
        "pong".to_string()
    }

    /// Returns the crack time of a password
    /// Used for password strength estimation
    /// On the frontend
    async fn crack_time(&self, password: String) -> CrackSeconds {
        let guesses = match zxcvbn::zxcvbn(&password, &[]) {
            Ok(entropy) => entropy.guesses(),
            Err(_) => 0,
        } as f64;

        CrackSeconds {
            guesses,
            seconds: guesses / 100_000.0,
            string: CrackTimeSeconds::Float(guesses / 100_000.0).to_string(),
        }
    }
}
