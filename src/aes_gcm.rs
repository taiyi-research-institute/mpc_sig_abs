use aes_gcm::{
    aead::{Aead, Payload},
    Aes256Gcm, Key, KeyInit, Nonce,
};
use erreur::*;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AEAD {
    pub ciphertext: Vec<u8>,
    pub tag: Vec<u8>,
}

/// Encrypts a plaintext using AES-GCM.
///
/// * `key` - passkey, passphrase, password
/// * `pt` - plaintext
/// * `aad` - additional associated data
pub fn aes_encrypt(key: &[u8], pt: &[u8], aad: &[u8]) -> Resultat<AEAD> {
    // create cipher
    let mut aes_key: [u8; 32] = [0; 32];
    let begin = std::cmp::max(0, 32 - key.len());
    aes_key[begin..].copy_from_slice(key);
    let aes_key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(aes_key);

    // generate nonce
    let mut nonce = [0u8; 12];
    OsRng.fill_bytes(&mut nonce); // provided by Rng trait
    let nonce = Nonce::from_slice(&nonce);

    // prepare payload
    let payload = Payload { msg: pt, aad };

    // encrypt
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .catch("AesGcmException", "")?;

    Ok(AEAD {
        ciphertext,
        tag: nonce.to_vec(),
    })
}

/// Encrypts a plaintext using AES-GCM.
///
/// * `key` - passkey, passphrase, password
/// * `pt` - plaintext
/// * `aad` - additional associated data
pub fn aes_decrypt(key: &[u8], ct: &AEAD, aad: &[u8]) -> Resultat<Vec<u8>> {
    // create cipher
    let mut aes_key: [u8; 32] = [0; 32];
    let begin = std::cmp::max(0, 32 - key.len());
    aes_key[begin..].copy_from_slice(key);
    let aes_key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(aes_key);

    // prepare nonce and payload
    let nonce = Nonce::from_slice(&ct.tag);
    let payload = Payload {
        msg: ct.ciphertext.as_slice(),
        aad,
    };

    // NOTE: no error reported but return a value NONE when decrypt key is wrong
    let out = cipher
        .decrypt(nonce, payload)
        .catch("AesGcmException", "Wrong password or nonce.")?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm() {
        let mut rng = OsRng;
        let mut key = [0u8; 32];
        rng.fill_bytes(&mut key);
        let pt = "Je ne veux pas travailler, je ne veux pas déjeuner, je veux seulement oublier, et puis je fume.".as_bytes();
        let aad = "Sympathique".as_bytes();

        let ct = aes_encrypt(&key, pt, aad).unwrap();
        let pt2 = aes_decrypt(&key, &ct, aad).unwrap();
        assert_eq!(pt, pt2.as_slice());
    }
}
