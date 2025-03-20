use hex;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

pub fn derive_swig_address(root_auth_type: &str, root_auth_id: &str) -> String {
    let input = format!("{}:{}", root_auth_type, root_auth_id);
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn sign_commitment(secret_key: &SecretKey, provider: &str, user_id: &str) -> Vec<u8> {
    let secp = Secp256k1::new();
    let commitment = format!("{}:{}", provider, user_id);
    let mut hasher = Sha256::new();
    hasher.update(commitment);
    let hashed_commitment = hasher.finalize();
    let message = Message::from_slice(&hashed_commitment).expect("Message must be 32 bytes");
    let signature = secp.sign_ecdsa(&message, secret_key);
    signature.serialize_compact().to_vec()
}

pub fn verify_signature(
    public_key: &PublicKey,
    provider: &str,
    user_id: &str,
    signature: &[u8],
    expected_swig_address: &str,
) -> bool {
    let derived_address = derive_swig_address(provider, user_id);
    if derived_address != expected_swig_address {
        return false;
    }

    let secp = Secp256k1::new();
    let commitment = format!("{}:{}", provider, user_id);
    let mut hasher = Sha256::new();
    hasher.update(commitment);
    let hashed_commitment = hasher.finalize();
    let message = Message::from_slice(&hashed_commitment).expect("Message must be 32 bytes");
    let sig =
        secp256k1::ecdsa::Signature::from_compact(signature).expect("Invalid signature format");
    secp.verify_ecdsa(&message, &sig, public_key).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use secp256k1::{PublicKey, Secp256k1, SecretKey};

    #[test]
    fn test_derive_swig_address() {
        let address = derive_swig_address("google", "1234567890");
        // Since SHA-256 produces a 32-byte digest, the hex encoding should be 64 characters long.
        assert_eq!(address.len(), 64);
    }

    #[test]
    fn test_sign_and_verify() {
        let provider = "google";
        let user_id = "1234567890";

        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0x01; 32]).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let expected_address = derive_swig_address(provider, user_id);

        let signature = sign_commitment(&secret_key, provider, user_id);
        assert!(verify_signature(
            &public_key,
            provider,
            user_id,
            &signature,
            &expected_address
        ));
    }

    #[test]
    fn test_invalid_signature() {
        let provider = "google";
        let user_id = "1234567890";

        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0x01; 32]).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let expected_address = derive_swig_address(provider, user_id);

        let mut signature = sign_commitment(&secret_key, provider, user_id);
        // Corrupt the signature by flipping a bit.
        signature[0] ^= 0x01;
        assert!(!verify_signature(
            &public_key,
            provider,
            user_id,
            &signature,
            &expected_address
        ));
    }

    #[test]
    fn test_invalid_derived_address() {
        let provider = "google";
        let user_id = "1234567890";

        let secp = Secp256k1::new();
        let secret_key = SecretKey::from_slice(&[0x01; 32]).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &secret_key);
        let signature = sign_commitment(&secret_key, provider, user_id);
        // Provide a wrong expected address.
        let wrong_address = derive_swig_address("github", user_id);
        assert!(!verify_signature(
            &public_key,
            provider,
            user_id,
            &signature,
            &wrong_address
        ));
    }
}

fn main() {
    println!("Deterministic Swig address module.");
}
