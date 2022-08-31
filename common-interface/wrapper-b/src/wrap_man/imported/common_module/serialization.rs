use serde::{Serialize, Deserialize};
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

pub fn serialize_abstract_method_args(args: ArgsAbstractMethod) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported module-type: abstract_method".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_abstract_method_args(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_abstract_method_args<W: Write>(args: ArgsAbstractMethod, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&2)?;
    writer.context().push("arg1", "String", "writing property");
    writer.write_string("arg1")?;
    writer.write_string(&args.arg1)?;
    writer.context().pop();
    writer.context().push("arg2", "u32", "writing property");
    writer.write_string("arg2")?;
    writer.write_u32(&args.arg2)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_abstract_method_result(result: &[u8]) -> Result<CommonObject, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported module-type: abstract_method".to_string();
    let mut reader = ReadDecoder::new(result, context);

    reader.context().push("abstractMethod", "CommonObject", "reading function output");
    let object = CommonObject::read(&mut reader)?;
    let res = object;
    reader.context().pop();
    Ok(res)
}
