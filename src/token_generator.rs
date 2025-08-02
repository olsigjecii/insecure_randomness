use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use uuid::Uuid;

/// # Insecure Token Generation
/// This function uses a Pseudo-Random Number Generator (PRNG) with a fixed seed.
/// Because the seed is constant, the sequence of "random" numbers generated is always the same.
/// An attacker who knows this seed can predict every token.
pub fn generate_vulnerable_token(user_id: &str) -> String {
    // Using a fixed seed makes the output predictable. This is a classic insecure randomness vulnerability.
    let seed: [u8; 32] = [1; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let random_number: u32 = rng.r#gen();

    // The token combines the user_id with a predictable number, making it easy to guess.
    format!("{}-{}", user_id, random_number)
}

/// # Secure Token Generation
/// This function uses the `uuid` crate's `Uuid::new_v4()` method.
/// `v4` UUIDs are generated using a cryptographically secure random number generator provided by the operating system.
/// This makes the generated tokens unpredictable and suitable for security-sensitive features.
pub fn generate_secure_token() -> String {
    Uuid::new_v4().to_string()
}
