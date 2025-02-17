extern crate solana_sdk;

use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::pubkey::Pubkey;
use hex;

fn main() {
    // Generate a new keypair
    let keypair = Keypair::new();

    // Get the private key (raw bytes)
    let private_key = keypair.to_bytes();

    // Convert the private key bytes to a hexadecimal string
    let private_key_hex = hex::encode(private_key);

    // Get the public key
    let public_key = keypair.pubkey();

    // Display the private key (hex string) and public key
    println!("Private Key (hex): {}", private_key_hex);
    println!("Public Key: {}", public_key);
}
