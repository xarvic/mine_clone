use serde::{Serialize, Deserialize};
use std::hash::Hash;

struct SerializeError(String);

struct DeserializeError(String);


trait ControlledData: Hash {
    type ControllerData: Serialize + Deserialize<'static>;

    fn serialize(&self, buffer: &mut [u8]) -> Result<usize, SerializeError>;
    fn deserialize(&mut self, buffer: &[u8]) -> Result<(), DeserializeError>;
}