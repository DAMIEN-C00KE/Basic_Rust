// More info at:
// https://docs.rs/ring/latest/ring/signature/index.html#signing-and-verifying-with-ed25519

use hex;
use ring::{
    rand,
    signature::{self, KeyPair}
};

fn main() -> Result<(), ring::error::Unspecified> {
    // Generate key pair in PKCS#8 (v2) format
    let rng = rand::SystemRandom::new();
    let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)?;

    let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;

    // Sign the message: "Super Secret Message"
    const MESSAGE: &[u8] = b"Super Secret Message";
    let sig = key_pair.sign(MESSAGE);

    // Convert bytes to hexadecimal string
    let sig_hex = hex::encode(sig.as_ref());

    println!("Signature (Hex): {}", sig_hex);

    let pub_key_bytes = key_pair.public_key().as_ref();

    let pub_key = 
        signature::UnparsedPublicKey::new(&signature::ED25519, pub_key_bytes);
    pub_key.verify(MESSAGE, sig.as_ref())?;

    Ok(())
}
