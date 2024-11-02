use openssl::hash::{hash, MessageDigest};
use openssl::rsa::{Padding, Rsa};
use openssl::sign::Signer;
use base64::{encode, decode};

// Function to hash data using SHA-256
pub fn hash_sha256(data: &str) -> String {
    let digest = hash(MessageDigest::sha256(), data.as_bytes()).expect("Failed to hash data");
    hex::encode(digest)
}

// Function to generate an RSA key pair
pub fn generate_rsa_keypair() -> (String, String) {
    let rsa = Rsa::generate(2048).expect("Failed to generate RSA key pair");
    let private_key = rsa.private_key_to_pem().expect("Failed to get private key");
    let public_key = rsa.public_key_to_pem().expect("Failed to get public key");
    (
        String::from_utf8(private_key).expect("Invalid UTF-8 sequence in private key"),
        String::from_utf8(public_key).expect("Invalid UTF-8 sequence in public key"),
    )
}

// Function to sign data using an RSA private key
pub fn sign_data(private_key: &str, data: &str) -> String {
    let rsa = Rsa::private_key_from_pem(private_key.as_bytes())
        .expect("Failed to load private key");
    let mut signer = Signer::new(MessageDigest::sha256(), &rsa).expect("Failed to create signer");
    signer.set_rsa_padding(Padding::PKCS1).expect("Failed to set padding");
    let signature = signer.sign_to_vec(data.as_bytes()).expect("Failed to sign data");
    encode(&signature)
}

// Function to verify data signature using an RSA public key
pub fn verify_signature(public_key: &str, data: &str, signature: &str) -> bool {
    let rsa = Rsa::public_key_from_pem(public_key.as_bytes())
        .expect("Failed to load public key");
    let decoded_signature = decode(signature).expect("Failed to decode signature");
    rsa.public_decrypt(&decoded_signature, &mut vec![0; rsa.size() as usize], Padding::PKCS1)
        .is_ok()
}

