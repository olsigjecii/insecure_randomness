
# Insecure Randomness in Rust 🦀

This lesson demonstrates the security vulnerability known as Insecure Randomness within a Rust application built with the actix-web framework. We will explore how predictable "random" values can be exploited and how to implement a secure, truly random alternative.

Lesson Summary
Insecure randomness occurs when a function that is supposed to produce a random value generates a predictable output. Since computers are deterministic, they use Pseudo-Random Number Generators (PRNGs) to simulate randomness. However, the quality of these generators varies significantly. If a PRNG is not cryptographically secure, its output can be predicted, allowing attackers to compromise security systems, such as by guessing password reset tokens. 

This project illustrates this vulnerability by creating two endpoints:

A vulnerable endpoint that uses a predictable PRNG to generate password reset tokens.

A secure endpoint that uses a cryptographically secure method to generate unpredictable tokens.

Setup and Execution
Follow these steps to set up and run the demonstration application.

Prerequisites:

Ensure you have Rust and Cargo installed. If not, you can install them via rustup.

1. Clone the Project:
(Note: As this is a demonstration, you will be copying the code provided above into the correct file structure.)

2. Build the Application:
Navigate to the project's root directory and run the build command:

Bash

cargo build
3. Run the Application:
Once the build is complete, start the actix-web server:

Bash

cargo run
The server will be running at http://127.0.0.1:8080.

Vulnerability Demonstration
The vulnerable endpoint at /vulnerable/forgot-password generates a password reset token using a PRNG (StdRng) that is seeded with a fixed value. Because the seed is always the same, the sequence of generated numbers is predictable.

Steps to Demonstrate:

Open your terminal and use curl to send a POST request to the vulnerable endpoint. We'll request a token for user123.

Bash

curl -X POST -H "Content-Type: application/json" -d '{"user_id": "user123"}' http://127.0.0.1:8080/vulnerable/forgot-password
You will receive a response with a token:

JSON

{"token":"user123-1498038865"}
Run the exact same command again. You will get the exact same token.

An attacker with access to the source code could easily discover the fixed seed and generate the exact same token for any user, allowing them to hijack accounts.

Mitigation Demonstration
The secure endpoint at /secure/forgot-password mitigates this vulnerability by using the uuid crate to generate a Version 4 UUID. This function relies on the operating system's own cryptographically secure random number source, ensuring the generated tokens are unpredictable.

Steps to Demonstrate:

Use curl to send a POST request to the secure endpoint.

Bash

curl -X POST http://127.0.0.1:8080/secure/forgot-password
You will receive a response with a unique, random token:

JSON

{"token":"a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d"}
(Note: Your token will be different.)

Run the command again.

Bash

curl -X POST http://127.0.0.1:8080/secure/forgot-password
You will receive a completely different and unique token:

JSON

{"token":"f0e9d8c7-b6a5-4f4e-3d2c-1b0a9f8e7d6c"}
Each call to this endpoint produces a token that cannot be predicted, effectively securing the password reset process.

Code Analysis
The Vulnerable Code (src/token_generator.rs):

Rust

use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub fn generate_vulnerable_token(user_id: &str) -> String {
    // Using a fixed seed makes the output predictable.
    let seed: [u8; 32] = [1; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let random_number: u32 = rng.gen();
    
    format!("{}-{}", user_id, random_number)
}
The vulnerability lies in using SeedableRng::from_seed with a hardcoded, constant seed. This makes the PRNG's output deterministic and predictable. 

The Mitigated Code (src/token_generator.rs):

Rust

use uuid::Uuid;

pub fn generate_secure_token() -> String {
    // Uuid::new_v4() uses a cryptographically secure source of randomness.
    Uuid::new_v4().to_string()
}
The fix is to replace the predictable generator with a function that is designed for cryptographic security. The 

uuid::Uuid::new_v4() function is an excellent choice for generating secure tokens as it is built for this purpose. This aligns with the best practice of using well-vetted libraries for security-critical functions instead of reinventing the wheel. 