mod errors;
mod db;
mod part_type;

use std::{
    collections::{hash_map::RandomState, HashMap},
    fmt::Debug,
    io::Read,
    ptr,
    str::{self, from_utf8},
};

use jammdb::{Data, KVPair, DB};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use errors::*;
pub use part_type::*;

pub mod elebox {
}
