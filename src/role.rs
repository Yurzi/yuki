#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Role {
    // role uuid
    role_id: Uuid,
}
