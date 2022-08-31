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
    deserialize_common_object,
    read_common_object,
    serialize_common_object,
    write_common_object
};

use crate::CommonNestedObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommonObject {
    pub prop1: String,
    pub prop2: u32,
    pub prop3: CommonNestedObject,
}

impl CommonObject {
    pub const URI: &'static str = "ens/interface.common.eth";

    pub fn new() -> CommonObject {
        CommonObject {
            prop1: String::new(),
            prop2: 0,
            prop3: CommonNestedObject::new(),
        }
    }

    pub fn to_buffer(args: CommonObject) -> Result<Vec<u8>, EncodeError> {
        serialize_common_object(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<CommonObject, DecodeError> {
        deserialize_common_object(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: CommonObject, writer: &mut W) -> Result<(), EncodeError> {
        write_common_object(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<CommonObject, DecodeError> {
        read_common_object(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
