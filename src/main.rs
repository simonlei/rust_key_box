use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand;
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};
use scanpw::scanpw;

use rust_key_box::errs::KeyBoxErr;
use rust_key_box::key_box::KeyBox;

fn main() {
    let key_box = match KeyBox::load_key_box() {
        Ok(key_box) => {
            let password = scanpw!("Password: ");
            key_box.verify_main_password(password);
            key_box
        }
        Err(KeyBoxErr::MainKeyNotExist) => {
            println!("Key box not init yet, please create main password:");
            let password1 = scanpw!("Password: ");
            let password2 = scanpw!("Password again: ");
            if password1 != password2 {
                panic!("Two passwords not equal");
            } else {
                KeyBox::new_key_box_with_main_password(password1)
            }
        } // Err(msg) => panic!("Can't open keybox, err is {}", msg),
    };

    if rust_key_box::is_main_key_exist() {
    } else {
    }
    let password = scanpw!("Password: ");
    println!("password is :{}", password);
    let text = "'Some strange text'";
    let main_key = new_magic_crypt!(password, 256);
    let encrypt = main_key.encrypt_bytes_to_base64(text.as_bytes());
    println!("encrypt {}", encrypt);
    println!("decrypt {}", main_key.decrypt_base64_to_string(encrypt).unwrap());

    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}
