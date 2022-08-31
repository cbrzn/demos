use serde::{Serialize, Deserialize};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    Read,
    Write,
    JSON,
    subinvoke,
};
pub mod serialization;
pub use serialization::{
    deserialize_abstract_method_result,
    serialize_abstract_method_args,
    ArgsAbstractMethod
};

use crate::CommonObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommonModule {}

impl CommonModule {
    pub const URI: &'static str = "ens/interface.common.eth";

    pub fn new() -> CommonModule {
        CommonModule {}
    }

    pub fn abstract_method(args: ArgsAbstractMethod) -> Result<CommonObject, String> {
        let uri = CommonModule::URI;
        let args = serialize_abstract_method_args(args).map_err(|e| e.to_string())?;
        let result = subinvoke::wrap_subinvoke(
            uri,
            "abstractMethod",
            args,
        )?;
        deserialize_abstract_method_result(result.as_slice()).map_err(|e| e.to_string())
    }
}
