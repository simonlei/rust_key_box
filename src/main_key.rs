use base64ct::{Base64, Encoding};
use rsa::pkcs1::LineEnding;
use rsa::pkcs8::der::zeroize::Zeroizing;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};

pub(crate) struct MainKey {
    public_key: RsaPublicKey,
    private_key: RsaPrivateKey,
    max_id: u32,
}

impl MainKey {
    pub(crate) fn encrypt(&self, pwd: String) -> String {
        let mut rng = rand::thread_rng();
        Base64::encode_string(
            self.public_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, pwd.as_bytes())
                .unwrap()
                .as_slice(),
        )
    }
}

impl MainKey {
    pub(crate) fn decrypt(&self, pwd: &str) -> String {
        let decoded = Base64::decode_vec(pwd).unwrap();
        let x = self.private_key.decrypt(Pkcs1v15Encrypt, decoded.as_slice()).unwrap();
        String::from_utf8_lossy(x.as_slice()).to_string()
    }
}

impl MainKey {
    pub(crate) fn replace_max_key_id(&mut self, id: u32) {
        if id > self.max_id {
            self.max_id = id;
        }
    }
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

        std::fs::create_dir_all("data").unwrap();
        std::fs::write("data/main.key", &encrypted_key).unwrap();
        let public_key = RsaPublicKey::from(&private_key);
        MainKey {
            public_key,
            private_key,
            max_id: 0,
        }
    }
}
