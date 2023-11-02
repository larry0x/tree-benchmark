use cosmwasm_std::{Order, Record, Storage};
use rocksdb::DB;
use std::path::Path;

/// Implement `cosmwasm_std::Storage` trait on RocksDB so that it can be used as
/// the DB backend for Tree.
pub struct DBWrapper {
    db: rocksdb::DB,
}

impl DBWrapper {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, rocksdb::Error> {
        let db = DB::open_default(path)?;
        Ok(Self { db })
    }
}

impl Storage for DBWrapper {
    fn set(&mut self, key: &[u8], value: &[u8]) {
        self.db.put(key, value).unwrap_or_else(|err| {
            let key_hex = hex::encode(key);
            let value_hex = hex::encode(value);
            panic!("failed to write key `{key_hex}` and value `{value_hex}` to rocksdb: {err}");
        });
    }

    fn remove(&mut self, key: &[u8]) {
        self.db.delete(key).unwrap_or_else(|err| {
            let key_hex = hex::encode(key);
            panic!("failed to delete key `{key_hex} from rocksdb: {err}`");
        });
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.db.get(key).unwrap_or_else(|err| {
            let key_hex = hex::encode(key);
            panic!("failed to get value for key `{key_hex}` from rocksdb: {err}");
        })
    }

    fn range<'a>(
        &'a self,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Order,
    ) -> Box<dyn Iterator<Item = Record> + 'a> {
        unimplemented!("not necessary for this use case");
    }
}
