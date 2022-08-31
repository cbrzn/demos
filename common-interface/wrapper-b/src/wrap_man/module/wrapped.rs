use polywrap_wasm_rs::{
  wrap_load_env
};

use crate::{
    abstract_method,
    ArgsAbstractMethod,
    deserialize_abstract_method_args,
    serialize_abstract_method_result
};


pub fn abstract_method_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match deserialize_abstract_method_args(args) {
        Ok(args) => {
            let result = abstract_method(ArgsAbstractMethod {
                arg1: args.arg1,
                arg2: args.arg2,
            });
            serialize_abstract_method_result(result).unwrap()
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
