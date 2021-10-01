//! Template for Trinci Smart Contract Development
//!
//! ### Methods
//!
//!  - echo

use trinci_sdk::{AppContext, WasmResult};

mod types;
use types::*;

trinci_sdk::app_export!(echo);

/// Echo.
fn echo<'a>(_ctx: AppContext, args: EchoArgs<'a>) -> WasmResult<EchoArgs<'a>> {
    Ok(args)
}

#[cfg(test)]
mod tests {

    use super::*;
    use trinci_sdk::not_wasm;

    const CALLER_ID: &str = "QmT48ijWd7RqEzdV3gKjqXN1kGBgYxFWsxajjguLkyTjy7";

    #[test]
    fn test_echo() {
        let ctx = not_wasm::create_app_context(CALLER_ID, CALLER_ID);

        let args = EchoArgs {
            data: "Hello, Trinci!",
        };

        let res = not_wasm::call_wrap(echo, ctx, args.clone()).unwrap();

        assert_eq!(res, args);
    }
}
