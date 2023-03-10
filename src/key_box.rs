use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};

use main_key::MainKey;

pub use crate::errs::KeyBoxErr;
use crate::{errs, main_key};

pub struct KeyBox {
    main_key: MainKey,
}

impl KeyBox {
    pub fn working(&self) {
        println!("Welcome to use rust key box, please input commands, input help to get more infos.");
        print!(">");
    }
}

impl KeyBox {
    pub fn load_with_password(key: String, pwd: String) -> KeyBox {
        let main_key = MainKey::load_key_with_password(key, pwd);
        KeyBox { main_key }
    }
}

impl KeyBox {
    pub fn new_key_box_with_main_password(pwd: String) -> KeyBox {
        let main_key = MainKey::generate_with_password(pwd);
        KeyBox { main_key }
    }
}
