use serde::{Serialize, Deserialize};
use std::convert::TryFrom;
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Context,
    DecodeError,
    EncodeError,
    Read,
    ReadDecoder,
    Write,
    WriteEncoder,
    JSON,
};

use crate::CommonObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsAbstractMethod {
    pub arg1: String,
    pub arg2: u32,
}

pub fn deserialize_abstract_method_args(args: &[u8]) -> Result<ArgsAbstractMethod, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing module-type: abstract_method".to_string();

    let mut reader = ReadDecoder::new(args, context);
    let mut num_of_fields = reader.read_map_length()?;

    let mut _arg1: String = String::new();
    let mut _arg1_set = false;
    let mut _arg2: u32 = 0;
    let mut _arg2_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "arg1" => {
                reader.context().push(&field, "String", "type found, reading argument");
                _arg1 = reader.read_string()?;
                _arg1_set = true;
                reader.context().pop();
            }
            "arg2" => {
                reader.context().push(&field, "u32", "type found, reading argument");
                _arg2 = reader.read_u32()?;
                _arg2_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_arg1_set {
        return Err(DecodeError::MissingField("arg1: String.".to_string()));
    }
    if !_arg2_set {
        return Err(DecodeError::MissingField("arg2: UInt32.".to_string()));
    }

    Ok(ArgsAbstractMethod {
        arg1: _arg1,
        arg2: _arg2,
    })
}

pub fn serialize_abstract_method_result(result: &CommonObject) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) module-type: abstract_method".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_abstract_method_result(result, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_abstract_method_result<W: Write>(result: &CommonObject, writer: &mut W) -> Result<(), EncodeError> {
    writer.context().push("abstractMethod", "CommonObject", "writing result");
    CommonObject::write(&result, writer)?;
    writer.context().pop();
    Ok(())
}
