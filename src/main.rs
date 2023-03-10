use rand;
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};
use scanpw::scanpw;

use rust_key_box::errs::KeyBoxErr;
use rust_key_box::key_box::KeyBox;

fn main() {
    let mut key_box = match std::fs::read_to_string("data/main.key") {
        Ok(key) => {
            let pwd = scanpw!("Password: ");
            KeyBox::load_with_password(key, pwd)
        }
        Err(err) => {
            println!("Key box not init yet, please create main password:");
            let password1 = scanpw!("Password: ");
            let password2 = scanpw!("Password again: ");
            if password1 != password2 {
                panic!("Two passwords do not equal");
            } else {
                KeyBox::new_key_box_with_main_password(password1)
            }
        } // Err(msg) => panic!("Can't open keybox, err is {}", msg),
    };
    key_box.working();
}
