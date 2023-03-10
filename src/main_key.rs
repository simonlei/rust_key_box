use rsa::pkcs1::LineEnding;
use rsa::pkcs8::der::zeroize::Zeroizing;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};

pub(crate) struct MainKey {
    password: String,
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
}

impl MainKey {
    pub(crate) fn load_key_with_password(key: String, pwd: String) -> MainKey {
        let priv_key = RsaPrivateKey::from_pkcs8_encrypted_pem(&key, pwd.as_bytes()).unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);
        MainKey {
            password: pwd,
            private_key: priv_key,
            public_key: pub_key,
        }
    }
}

impl MainKey {
    pub(crate) fn generate_with_password(pwd: String) -> MainKey {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

        let encrypted_key: Zeroizing<String> = priv_key
            .to_pkcs8_encrypted_pem(rng, pwd.as_bytes(), LineEnding::CRLF)
            .unwrap();
        std::fs::create_dir("data").unwrap();
        std::fs::write("data/main.key", encrypted_key.to_string()).unwrap();

        // priv_key.to_pkcs8_encrypted_der(rng, pwd.as_bytes());
        let pub_key = RsaPublicKey::from(&priv_key);
        /*        // Encrypt
                let data = b"hello world";
                let enc_data = pub_key
                    .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
                    .expect("failed to encrypt");
                assert_ne!(&data[..], &enc_data[..]);
        */
        // Decrypt
        /*      let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
              assert_eq!(&data[..], &dec_data[..]);
        */
        MainKey {
            password: pwd,
            public_key: pub_key,
            private_key: priv_key,
        }
    }
}
