use serde::{Serialize, Deserialize};
pub mod serialization;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    DecodeError,
    EncodeError,
    Read,
    Write,
    JSON,
};
pub use serialization::{
    deserialize_my_object,
    read_my_object,
    serialize_my_object,
    write_my_object
};

use crate::CommonNestedObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MyObject {
    pub prop1: String,
    pub prop2: u32,
    pub prop3: CommonNestedObject,
}

impl MyObject {
    pub fn new() -> MyObject {
        MyObject {
            prop1: String::new(),
            prop2: 0,
            prop3: CommonNestedObject::new(),
        }
    }

    pub fn to_buffer(args: MyObject) -> Result<Vec<u8>, EncodeError> {
        serialize_my_object(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<MyObject, DecodeError> {
        deserialize_my_object(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: MyObject, writer: &mut W) -> Result<(), EncodeError> {
        write_my_object(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<MyObject, DecodeError> {
        read_my_object(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
