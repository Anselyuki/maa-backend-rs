use tokio::time::Instant;
use tracing::Level;

use crate::MaaResult;

pub struct PasswordEncoder {
    cost: u32,
}

impl Default for PasswordEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordEncoder {
    pub fn new() -> Self {
        let span = tracing::span!(Level::INFO, "Initializing PasswordEncoder");
        let _ = span.enter();
        let mut cost = 4u32;
        let test_password = "password";
        // find a cost that makes it about 1000ms to hash
        loop {
            let now = Instant::now();
            #[allow(clippy::expect_used)]
            bcrypt::hash(test_password, cost).expect("Failed to hash password");
            let elapsed = now.elapsed();
            if elapsed.as_millis() > 1000 {
                tracing::info!(
                    "Found cost: {}, hash time is {} ms",
                    cost,
                    elapsed.as_millis()
                );
                break;
            }
            if cost >= 31 {
                tracing::warn!(
                    "Cost is too high, hash time is {} ms",
                    elapsed.as_millis()
                );
                break;
            }
            cost += 1;
        }
        Self { cost }
    }

    pub fn encode(&self, password: &str) -> MaaResult<String> {
        let encoded = bcrypt::hash(password, self.cost)?;
        Ok(encoded)
    }

    pub fn matches(&self, password: &str, hash: &str) -> MaaResult<bool> {
        let matches = bcrypt::verify(password, hash)?;
        Ok(matches)
    }
}
