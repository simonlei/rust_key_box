use scanpw::scanpw;

use rust_key_box::key_box::KeyBox;

fn main() {
    let mut key_box = match std::fs::read_to_string(format!("{}/main.key", KeyBox::get_data_dir())) {
        Ok(key) => {
            let pwd = scanpw!("Password: ");
            KeyBox::load_with_password(key, pwd)
        }
        Err(_) => {
            println!("Key box not init yet, please create main password:");
            let password1 = scanpw!("Password: ");
            let password2 = scanpw!("Password again: ");
            if password1 != password2 {
                panic!("Two passwords do not equal");
            } else {
                KeyBox::new_key_box_with_main_password(password1)
            }
        }
    };
    key_box.working();
}
