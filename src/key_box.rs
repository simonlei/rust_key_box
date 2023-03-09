pub use crate::errs::KeyBoxErr;
use crate::{errs, main_key};

pub struct KeyBox {}

impl KeyBox {
    pub fn new_key_box_with_main_password(pwd: String) -> KeyBox {
        todo!()
    }
}

impl KeyBox {
    pub fn verify_main_password(&self, pwd: String) {
        todo!()
    }
}

impl KeyBox {
    pub fn load_key_box() -> Result<KeyBox, KeyBoxErr> {
        Err(KeyBoxErr::MainKeyNotExist)
    }
}
