// Salt and Hash a password using bcrypt
// Example output: $2b$12$TZAnjfUbUctBQwIXiCjVY.Jo4dln4k4oO9totRGtVzLPe6LEJ/fsu
// $bcrypt_id$cost$salt_and_hashed_password
// - $2b$ is bcrypt algorithm version
// - '12' is the cost factor (in this case, 2**12 iterations)
// - The first 22 characters after the cost factor is the salt: TZAnjfUbUctBQwIXiCjVY.
// - The remaining 31 characters is the hashed password: Jo4dln4k4oO9totRGtVzLPe6LEJ/fsu

extern crate bcrypt;

fn main() {
    const PASSWORD: &str = "mmm salty";

    // Hash password
    let hashed_password = bcrypt::hash(PASSWORD, bcrypt::DEFAULT_COST)
        .expect("Failed to hash password");

    println!("Hashed Password: {}", hashed_password);

    // Verification of password to hashed password
    let success = bcrypt::verify(PASSWORD, &hashed_password);
    let wrong_password = "This is not the correct password";
    let failure = bcrypt::verify(wrong_password, &hashed_password);

    assert!(success.expect("Verification failed"));
    assert!(!failure.expect("Verification should have failed"));
}
