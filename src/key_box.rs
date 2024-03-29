use std::error::Error;
use std::ffi::OsStr;
use std::io;
use std::io::Write;

use chrono::{NaiveDateTime, Utc};
use copypasta::{ClipboardContext, ClipboardProvider};
use rand_pwd::RandPwd;
use serde::{Deserialize, Serialize};

use main_key::MainKey;

pub use crate::errs::KeyBoxErr;
use crate::main_key;

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
    last_updated: i64,
}

impl ToString for Key {
    fn to_string(&self) -> String {
        format!(
            "id:{} url:{} login:{} notes:{} last_updated:{}",
            self.id,
            self.url,
            self.user,
            self.notes,
            NaiveDateTime::from_timestamp_millis(self.last_updated)
                .unwrap()
                .format("%Y-%m-%d")
        )
    }
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
                    if let Ok(response) = self.deal_with_command(input.trim()) {
                        println!("{response}");
                    }
                }
                Err(err) => {
                    println!("Error:{}, exit", err);
                    break;
                }
            }
        }
    }

    fn deal_with_command(&mut self, input: &str) -> Result<String, Box<dyn Error>> {
        match input {
            "h" | "help" => Ok(self.show_help()),
            "c" | "create" => self.create_new_key(),
            "l" | "list" => self.list_all_keys(),
            s if s.starts_with("s ") => self.show_key(&s[2..]),
            s if s.starts_with("show ") => self.show_key(&s[5..]),
            q if q.starts_with("q ") => self.query_key(&q[2..]),
            q if q.starts_with("query ") => self.query_key(&q[6..]),
            e if e.starts_with("e ") => self.edit_key(&e[2..]),
            e if e.starts_with("edit ") => self.edit_key(&e[5..]),
            d if d.starts_with("d ") => self.delete_key(&d[2..]),
            d if d.starts_with("delete ") => self.delete_key(&d[7..]),
            _ => Ok(self.show_help()),
        }
    }

    fn create_new_key(&mut self) -> Result<String, Box<dyn Error>> {
        println!("Creating new key, please input:");
        let (url, user, notes, password) = read_input_for_key(&self.main_key, "url :", "login name :", "notes :")?;
        let id = self.main_key.get_next_id();
        let key = Key {
            url,
            user,
            notes,
            password,
            id,
            last_updated: Utc::now().timestamp_millis(),
        };
        save_key(&key)?;
        let result = format!("Key {} is saved.", key.url);
        self.keys.push(key);
        Ok(result)
    }

    fn list_all_keys(&self) -> Result<String, Box<dyn Error>> {
        Ok(display_keys(&self.keys.iter().collect()))
    }

    fn show_key(&self, input: &str) -> Result<String, Box<dyn Error>> {
        let id: u32 = input.parse()?;
        let pwd = self.decrypt_passwd(id);
        match ClipboardContext::new() {
            Ok(mut ctx) => {
                ctx.set_contents(pwd.clone()).unwrap();
                Ok(format!("Password is copied to clipboard:{}", pwd))
            }
            Err(_) => Ok(format!("Can't copy password to clipboard:{}", pwd)),
        }
    }
    fn decrypt_passwd(&self, id: u32) -> String {
        let key = self.keys.iter().find(|x| x.id == id);

        match key {
            Some(key) => self.main_key.decrypt(&key.password),
            None => String::from("No such key"),
        }
    }
    fn show_help(&self) -> String {
        r#"c/create         Create a key
l/list           List all keys
s/show id        Show and copy password for the key with id
q/query string   Query keys
d/delete id      Delete the key with id
e/edit id        Edit the key with id"#
            .to_string()
    }
    fn query_key(&self, query: &str) -> Result<String, Box<dyn Error>> {
        let filtered_keys: Vec<&Key> = self
            .keys
            .iter()
            .filter(|x| x.url.contains(query) || x.user.contains(query) || x.notes.contains(query))
            .collect();
        Ok(display_keys(&filtered_keys))
    }
    fn edit_key(&mut self, input: &str) -> Result<String, Box<dyn Error>> {
        let id: u32 = input.parse()?;
        if let Some(key) = self.keys.iter_mut().find(|x| x.id == id) {
            let (url, user, notes, password) = read_input_for_key(
                &self.main_key,
                format!("change url {} to:(empty to keep unchanged)", key.url).as_str(),
                format!("chagne login name {} to:(empty to keep unchanged)", key.user).as_str(),
                format!("chagne notes {} to:(empty to keep unchanged)", key.notes).as_str(),
            )?;

            if !url.is_empty() {
                key.url = url;
            }
            if !user.is_empty() {
                key.user = user;
            }
            if !notes.is_empty() {
                key.notes = notes;
            }
            key.password = password;
            key.last_updated = Utc::now().timestamp_millis();
            save_key(key)?;
            Ok(format!("Key {} changed", key.id))
        } else {
            Ok(String::from("No such key"))
        }
    }

    fn delete_key(&mut self, input: &str) -> Result<String, Box<dyn Error>> {
        println!("Are you sure to delete key {}? Y for sure", input);
        let mut sure = String::new();
        match io::stdin().read_line(&mut sure) {
            Ok(_) if sure.trim() == "Y" => self.real_delete_key(input),
            _ => Ok("".to_string()),
        }
    }
    fn real_delete_key(&mut self, input: &str) -> Result<String, Box<dyn Error>> {
        let id: u32 = input.parse()?;
        if let Some(index) = self.keys.iter().position(|x| x.id == id) {
            self.keys.swap_remove(index);
            std::fs::remove_file(format!("{}/{}.key", KeyBox::get_data_dir(), id))?;
            Ok(format!("{} is deleted", input))
        } else {
            Ok(String::from("No such key"))
        }
    }
}

fn read_input_for_key(
    main_key: &MainKey,
    prompt_url: &str,
    prompt_user: &str,
    prompt_notes: &str,
) -> Result<(String, String, String, String), Box<dyn Error>> {
    let url = read_line(prompt_url)?;
    let user = read_line(prompt_user)?;
    let notes = read_line(prompt_notes)?;
    let mut password = read_line("password(empty to auto gen) :")?;
    if password.trim().is_empty() {
        let mut pwd = RandPwd::new(8, 5, 3);
        pwd.join();
        password = pwd.val().to_string();
    }
    password = main_key.encrypt(password);
    Ok((url, user, notes, password))
}

fn save_key(key: &Key) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string(key)?;
    let file = format!("{}/{}.key", KeyBox::get_data_dir(), key.id);
    std::fs::write(file, json)?;
    Ok(())
}

fn read_line(prompt: &str) -> Result<String, Box<dyn Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

impl KeyBox {
    pub fn load_with_password(key: String, pwd: String) -> KeyBox {
        let mut main_key = MainKey::load_key_with_password(key, pwd);
        let mut keys: Vec<Key> = Vec::new();
        load_keys(&mut keys, &mut main_key);
        println!("Total {} keys loaded", keys.len());
        KeyBox { main_key, keys }
    }
}

fn display_keys(keys: &Vec<&Key>) -> String {
    let mut result = String::new();
    for key in keys {
        result += &key.to_string();
        result += "\n";
    }
    result.trim().to_string()
}

fn load_keys(keys: &mut Vec<Key>, main_key: &mut MainKey) {
    for file in std::fs::read_dir(KeyBox::get_data_dir()).unwrap() {
        let path = file.unwrap().path();
        let is_key = path.is_file() && path.extension().eq(&Some(OsStr::new("key")));
        if is_key && !path.ends_with("main.key") {
            // println!("{}", path.display());
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

    pub fn get_data_dir() -> String {
        if cfg!(debug_assertions) {
            "data/".to_string()
        } else {
            home::home_dir()
                .unwrap()
                .join(".keybox/")
                .into_os_string()
                .into_string()
                .unwrap()
        }
    }
}
