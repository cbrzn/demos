pub mod wrapped;
pub use wrapped::{
    abstract_method_wrapped
};
pub mod serialization;
pub use serialization::{
    deserialize_abstract_method_args,
    serialize_abstract_method_result,
    ArgsAbstractMethod
};
