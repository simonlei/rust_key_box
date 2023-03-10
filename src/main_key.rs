use rsa::pkcs1::LineEnding;
use rsa::pkcs8::der::zeroize::Zeroizing;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};

pub(crate) struct MainKey {
    password: String,
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
    max_id: u32,
}

impl MainKey {
    pub(crate) fn get_next_id(&mut self) -> u32 {
        self.max_id += 1;
        self.max_id
    }
}

impl MainKey {
    pub(crate) fn load_key_with_password(key: String, password: String) -> MainKey {
        let private_key =
            RsaPrivateKey::from_pkcs8_encrypted_pem(&key, password.as_bytes()).expect("Wrong main password!");
        let public_key = RsaPublicKey::from(&private_key);
        MainKey {
            password,
            private_key,
            public_key,
            max_id: 0,
        }
    }
}

impl MainKey {
    pub(crate) fn generate_with_password(password: String) -> MainKey {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");

        let encrypted_key: Zeroizing<String> = private_key
            .to_pkcs8_encrypted_pem(rng, password.as_bytes(), LineEnding::CRLF)
            .unwrap();
        std::fs::create_dir("data").unwrap();
        std::fs::write("data/main.key", encrypted_key.to_string()).unwrap();

        // priv_key.to_pkcs8_encrypted_der(rng, pwd.as_bytes());
        let public_key = RsaPublicKey::from(&private_key);
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
            password,
            public_key,
            private_key,
            max_id: 0,
        }
    }
}
