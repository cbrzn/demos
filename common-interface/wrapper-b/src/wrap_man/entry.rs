use crate::{
    abstract_method_wrapped
};
use polywrap_wasm_rs::{
    abort,
    invoke,
    InvokeArgs,
};

#[no_mangle]
pub extern "C" fn _wrap_invoke(method_size: u32, args_size: u32, env_size: u32) -> bool {
    // Ensure the abort handler is properly setup
    abort::wrap_abort_setup();

    let args: InvokeArgs = invoke::wrap_invoke_args(method_size, args_size);

    match args.method.as_str() {
        "abstractMethod" => invoke::wrap_invoke(args, env_size, Some(abstract_method_wrapped)),
        _ => invoke::wrap_invoke(args, env_size, None),
    }
}
