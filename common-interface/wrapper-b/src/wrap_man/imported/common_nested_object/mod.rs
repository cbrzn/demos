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
    deserialize_common_nested_object,
    read_common_nested_object,
    serialize_common_nested_object,
    write_common_nested_object
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommonNestedObject {
    pub prop: String,
}

impl CommonNestedObject {
    pub const URI: &'static str = "ens/interface.common.eth";

    pub fn new() -> CommonNestedObject {
        CommonNestedObject {
            prop: String::new(),
        }
    }

    pub fn to_buffer(args: &CommonNestedObject) -> Result<Vec<u8>, EncodeError> {
        serialize_common_nested_object(args).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn from_buffer(args: &[u8]) -> Result<CommonNestedObject, DecodeError> {
        deserialize_common_nested_object(args).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }

    pub fn write<W: Write>(args: &CommonNestedObject, writer: &mut W) -> Result<(), EncodeError> {
        write_common_nested_object(args, writer).map_err(|e| EncodeError::TypeWriteError(e.to_string()))
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<CommonNestedObject, DecodeError> {
        read_common_nested_object(reader).map_err(|e| DecodeError::TypeReadError(e.to_string()))
    }
}
