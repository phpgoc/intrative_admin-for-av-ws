use crate::admin::structs_types::{AdminError, AdminResult};

pub struct Dummy {}

impl crate::admin::db::traits::AsyncDbTrait for Dummy {
    fn new() -> AdminResult<Self> {
        if let Ok(db) = sled::open("dummy.db") {
            Ok(Dummy {})
        } else {
            Err(AdminError::DbError("Error".to_string()))
        }
    }
}
