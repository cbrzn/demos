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

use crate::CommonNestedObject;

pub fn serialize_common_object(args: &CommonObject) -> Result<Vec<u8>, EncodeError> {
    let mut encoder_context = Context::new();
    encoder_context.description = "Serializing (encoding) imported object-type: CommonObject".to_string();
    let mut encoder = WriteEncoder::new(&[], encoder_context);
    write_common_object(args, &mut encoder)?;
    Ok(encoder.get_buffer())
}

pub fn write_common_object<W: Write>(args: &CommonObject, writer: &mut W) -> Result<(), EncodeError> {
    writer.write_map_length(&3)?;
    writer.context().push("prop1", "String", "writing property");
    writer.write_string("prop1")?;
    writer.write_string(&args.prop1)?;
    writer.context().pop();
    writer.context().push("prop2", "u32", "writing property");
    writer.write_string("prop2")?;
    writer.write_u32(&args.prop2)?;
    writer.context().pop();
    writer.context().push("prop3", "CommonNestedObject", "writing property");
    writer.write_string("prop3")?;
    CommonNestedObject::write(&args.prop3, writer)?;
    writer.context().pop();
    Ok(())
}

pub fn deserialize_common_object(args: &[u8]) -> Result<CommonObject, DecodeError> {
    let mut context = Context::new();
    context.description = "Deserializing imported object-type: CommonObject".to_string();
    let mut reader = ReadDecoder::new(args, context);
    read_common_object(&mut reader)
}

pub fn read_common_object<R: Read>(reader: &mut R) -> Result<CommonObject, DecodeError> {
    let mut num_of_fields = reader.read_map_length()?;

    let mut _prop1: String = String::new();
    let mut _prop1_set = false;
    let mut _prop2: u32 = 0;
    let mut _prop2_set = false;
    let mut _prop3: CommonNestedObject = CommonNestedObject::new();
    let mut _prop3_set = false;

    while num_of_fields > 0 {
        num_of_fields -= 1;
        let field = reader.read_string()?;

        match field.as_str() {
            "prop1" => {
                reader.context().push(&field, "String", "type found, reading property");
                _prop1 = reader.read_string()?;
                _prop1_set = true;
                reader.context().pop();
            }
            "prop2" => {
                reader.context().push(&field, "u32", "type found, reading property");
                _prop2 = reader.read_u32()?;
                _prop2_set = true;
                reader.context().pop();
            }
            "prop3" => {
                reader.context().push(&field, "CommonNestedObject", "type found, reading property");
                let object = CommonNestedObject::read(reader)?;
                _prop3 = object;
                _prop3_set = true;
                reader.context().pop();
            }
            err => return Err(DecodeError::UnknownFieldName(err.to_string())),
        }
    }
    if !_prop1_set {
        return Err(DecodeError::MissingField("prop1: String.".to_string()));
    }
    if !_prop2_set {
        return Err(DecodeError::MissingField("prop2: UInt32.".to_string()));
    }
    if !_prop3_set {
        return Err(DecodeError::MissingField("prop3: Common_NestedObject.".to_string()));
    }

    Ok(CommonObject {
        prop1: _prop1,
        prop2: _prop2,
        prop3: _prop3,
    })
}
