pub mod wrap_man;
pub use wrap_man::*;
use wrap::imported::{ CommonObject, CommonNestedObject };

pub fn abstract_method(
    args: ArgsAbstractMethod
) -> CommonObject {
    // NOTE: the below clone shouldn't be required,
    //       but the WRAP codegen uses mutable references
    //       when it shouldn't. This will be fixed.
    let arg1 = args.arg1;
    let arg1_str = arg1.as_str();

    return CommonObject {
        prop1: arg1,
        prop2: args.arg2,
        prop3: CommonNestedObject {
            prop: format!("{}{}", arg1_str, args.arg2)
        }
    };
}
