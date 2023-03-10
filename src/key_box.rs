use std::ffi::OsStr;
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
            s if s.starts_with("s ") => self.show_key(&s[2..]),
            s if s.starts_with("show ") => self.show_key(&s[5..]),
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
        password = self.main_key.encrypt(password);
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
        let mut result = String::new();
        for key in &self.keys {
            result += format!("id:{} url:{} login:{} notes:{}\n", key.id, key.url, key.user, key.notes).as_str();
        }
        result
    }
    fn show_key(&self, input: &str) -> String {
        let id: u32 = input.parse().unwrap();
        println!("{}", id);
        // show password
        let pwd = self.decrypt_passwd(id);
        // copy to clipboard
        pwd
    }
    fn decrypt_passwd(&self, id: u32) -> String {
        let key = self.keys.iter().find(|x| x.id == id);

        match key {
            Some(key) => self.main_key.decrypt(&key.password),
            None => String::from("No such key"),
        }
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
        let mut main_key = MainKey::load_key_with_password(key, pwd);
        let mut keys: Vec<Key> = Vec::new();
        load_keys(&mut keys, &mut main_key);
        KeyBox { main_key, keys }
    }
}

fn load_keys(keys: &mut Vec<Key>, main_key: &mut MainKey) {
    for file in std::fs::read_dir("./data").unwrap() {
        let path = file.unwrap().path();
        let is_key = path.is_file() && path.extension().eq(&Some(OsStr::new("key")));
        if is_key && !path.ends_with("main.key") {
            println!("{}", path.display());
            let key: Key = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
            main_key.replace_max_key_id(key.id);
            keys.push(key);
        }
    }
}

impl KeyBox {
    pub fn new_key_box_with_main_password(pwd: String) -> KeyBox {
        let main_key = MainKey::generate_with_password(pwd);
        let keys: Vec<Key> = Vec::new();
        KeyBox { main_key, keys }
    }
}
