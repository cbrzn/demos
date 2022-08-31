pub mod entry;
pub mod my_object;
pub use my_object::MyObject;
pub mod imported;
pub use imported::common_object::CommonObject;
pub use imported::common_nested_object::CommonNestedObject;
pub use imported::common_module::CommonModule;
pub mod module;
pub use module::{
    deserialize_abstract_method_args,
    serialize_abstract_method_result,
    abstract_method_wrapped,
    ArgsAbstractMethod
};
