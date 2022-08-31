pub mod wrap;
pub use wrap::*;
use wrap::imported::{ CommonObject, CommonNestedObject };

pub fn abstract_method(
    args: ArgsAbstractMethod
) -> CommonObject {
    let arg1 = args.arg1;
    let arg1_str = arg1.as_str();

    return CommonObject {
        prop1: arg1_str.to_string(),
        prop2: args.arg2,
        prop3: CommonNestedObject {
            prop: format!("{}{}", arg1_str, args.arg2)
        }
    };
}
