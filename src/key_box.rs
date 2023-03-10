use std::io;
use std::io::Write;

use rand_pwd::RandPwd;
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use main_key::MainKey;

pub use crate::errs::KeyBoxErr;
use crate::{errs, main_key};

pub struct KeyBox {
    main_key: MainKey,
    keys: Vec<Key>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Key {
    id: u32,
    url: String,
    user: String,
    password: String,
    notes: String,
}

impl KeyBox {
    pub fn working(&mut self) {
        println!("Welcome to use rust key box, please input commands, input help to get more infos.");
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    if n == 0 {
                        // get eof, maybe pipeline
                        break;
                    }
                    if input.trim().eq("quit") {
                        break;
                    }
                    let response = self.deal_with_command(&input.trim());
                    println!("{response}");
                }
                Err(err) => {
                    println!("Error:{}, exit", err);
                    break;
                }
            }
        }
    }

    fn deal_with_command(&mut self, input: &str) -> String {
        match input {
            "h" | "help" => String::from("help"),
            "c" | "create" => self.create_new_key(),
            "l" | "list" => self.list_all_keys(),
            _ => String::from("help"),
        }
    }

    fn create_new_key(&mut self) -> String {
        println!("Creating new key, please input:");
        let url = read_line("url :");
        let user = read_line("login name :");
        let notes = read_line("notes :");
        let mut password = read_line("password(empty to auto gen) :");
        if password.trim().is_empty() {
            let mut pwd = RandPwd::new(8, 5, 3);
            pwd.join();
            password = pwd.val().to_string();
        }
        let id = self.main_key.get_next_id();
        let key = Key {
            url,
            user,
            notes,
            password,
            id,
        };
        save_key(&key);
        let result = format!("Key {} is saved.", key.url);
        self.keys.push(key);
        result
    }

    fn list_all_keys(&self) -> String {
        "".to_string()
    }
}

fn save_key(key: &Key) {
    let json = serde_json::to_string(key).unwrap();
    let file = format!("data/{}.key", key.id);
    std::fs::write(file, json).unwrap();
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

impl KeyBox {
    pub fn load_with_password(key: String, pwd: String) -> KeyBox {
        let main_key = MainKey::load_key_with_password(key, pwd);
        let keys: Vec<Key> = Vec::new();
        KeyBox { main_key, keys }
    }
}

impl KeyBox {
    pub fn new_key_box_with_main_password(pwd: String) -> KeyBox {
        let main_key = MainKey::generate_with_password(pwd);
        let keys: Vec<Key> = Vec::new();
        KeyBox { main_key, keys }
    }
}
