use crate::MuzzManSimple;

impl MuzzManSimple {
    pub fn get_filename(&self) -> Result<String, ()> {
        match self.url.split('/').last() {
            Some(filename) => Ok(filename.to_owned()),
            None => Err(()),
        }
    }
}
