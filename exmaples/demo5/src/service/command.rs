use crate::{pb::abi::{CommandResponse, Hget, Hgetall}, storage, error::KvError};

use super::CommandService;


impl CommandService for Hget {
    fn execute(self, storage :&impl crate::storage::Storage) -> CommandResponse  {
        match storage.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hgetall {
    fn execute(self, storage :&impl crate::storage::Storage) -> CommandResponse  {
        match storage.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}