use rand::RngCore;
use secp256k1::{PublicKey, SecretKey, Secp256k1};
use bech32::{self, ToBase32, Variant};

fn main() {
    let secp = Secp256k1::new();
    let mut rng = rand::thread_rng();

    // Generate a new random secret key
    let mut secret_key_bytes = [0u8; 32];
    rng.fill_bytes(&mut secret_key_bytes);
    let secret_key = SecretKey::from_slice(&secret_key_bytes).unwrap();

    // Get the corresponding public key
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    // Define the human-readable part (hrp) for bech32 encoding
    let hrp = "npub";

    // Encode the public key in bech32 format
    let public_key_bech32 = bech32::encode(hrp, public_key.serialize().as_ref().to_base32(), Variant::Bech32).unwrap();

    println!("Public Key (bech32): {}", public_key_bech32);

    // Define the human-readable part (hrp) for the secret key bech32 encoding
    let hrp_secret = "nsec";

    // Encode the secret key in bech32 format
    let secret_key_bech32 = bech32::encode(hrp_secret, secret_key[..].as_ref().to_base32(), Variant::Bech32).unwrap();

    println!("Secret Key (bech32): {}", secret_key_bech32);

    // Convert the secret key to a hexadecimal string representation
    let secret_key_hex = hex::encode(secret_key[..].to_vec());

    println!("Private Key (hex): {}", secret_key_hex);
}
