use common::data::EncryptionConfig;
use rsa::{
    pkcs1v15::SigningKey, sha2::Sha256, signature::RandomizedSigner, Pkcs1v15Encrypt,
    RsaPrivateKey, RsaPublicKey,
};

const RSA_CHUNK_SIZE: usize = 245;

pub struct RsaKeyPair(
    RsaPublicKey,
    RsaPrivateKey,
    SigningKey<Sha256>,
    RsaPublicKey,
    RsaPrivateKey,
);

impl RsaKeyPair {
    pub fn from_encryption_config(config: &EncryptionConfig) -> Self {
        let client_public_key: RsaPublicKey =
            rsa::pkcs8::DecodePublicKey::from_public_key_der(&config.client_public_key).unwrap();
        let server_private_key: RsaPrivateKey =
            rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(&config.server_private_key).unwrap();
        let signing_key = SigningKey::new(server_private_key.clone());

        let server_public_key: RsaPublicKey =
            rsa::pkcs8::DecodePublicKey::from_public_key_der(&config.server_public_key).unwrap();
        let client_private_key: RsaPrivateKey =
            rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(&config.client_private_key).unwrap();

        Self(
            client_public_key,
            server_private_key,
            signing_key,
            server_public_key,
            client_private_key,
        )
    }

    pub fn client_encrypt(&self, data: &[u8]) -> Box<[u8]> {
        let mut rng = rand::thread_rng();

        let mut result: Vec<u8> = Vec::new();
        for chunk in data.chunks(RSA_CHUNK_SIZE) {
            let encrypted_chunk = self.0.encrypt(&mut rng, Pkcs1v15Encrypt, chunk).unwrap();

            result.extend(encrypted_chunk);
        }

        result.into()
    }

    pub fn client_decrypt(&self, cipher: &[u8]) -> Option<Vec<u8>> {
        self.4.decrypt(Pkcs1v15Encrypt, cipher).ok()
    }

    pub fn sign(&self, data: &[u8]) -> Box<[u8]> {
        self.2.sign_with_rng(&mut rand::thread_rng(), data).into()
    }

    pub fn server_encrypt(&self, data: &[u8]) -> Box<[u8]> {
        let mut rng = rand::thread_rng();

        let mut result: Vec<u8> = Vec::new();
        for chunk in data.chunks(RSA_CHUNK_SIZE) {
            let encrypted_chunk = self.3.encrypt(&mut rng, Pkcs1v15Encrypt, chunk).unwrap();

            result.extend(encrypted_chunk);
        }

        result.into()
    }

    pub fn server_decrypt(&self, cipher: &[u8]) -> Option<Vec<u8>> {
        self.1.decrypt(Pkcs1v15Encrypt, cipher).ok()
    }
}