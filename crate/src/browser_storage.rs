extern crate plasma_client;
extern crate web_sys;
use plasma_client::error::{Error, ErrorKind};
use plasma_client::Storage;

pub struct BrowserStorage {
    local_storage: web_sys::Storage,
}

impl BrowserStorage {
    pub fn new(name: &str) -> Option<BrowserStorage> {
        let window = web_sys::window()?;
        if let Ok(Some(local_storage)) = window.local_storage() {
            let mut storage = BrowserStorage { local_storage };
            Some(storage)
        } else {
            None
        }
    }
}

impl Storage for BrowserStorage {
    fn get(&self, key: &str) -> Result<Option<String>, Error> {
        if let Ok(Some(v)) = self.local_storage.get_item(key) {
            Ok(Some(v))
        } else {
            Err(Error::from(ErrorKind::Io))
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<(), Error> {
        if let Ok(()) = self.local_storage.set_item(key, value) {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Io))
        }
    }

    fn delete(&self, key: &str) -> Result<(), Error> {
        if let Ok(()) = self.local_storage.remove_item(key) {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Io))
        }
    }
}
