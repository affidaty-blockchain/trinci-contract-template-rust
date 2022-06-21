//! Template for Trinci Smart Contract Development
//!
//! ### Methods
//!
//!  - `init` - Initialize the contract on the account
//!  - `before_transfer - Prevent the account to transfer asset
//!                       if the condition are not satisfied
//!  - `after_transfer` - Method called from an advanced asset
//!                       after the performing of an asset transfer
//!  - `echo` - Repeat the args

use trinci_sdk::{
    tai::AssetTransferArgs as TransferArgs, AppContext, PackedValue, WasmError, WasmResult,
};

mod types;
use types::*;

const INIT_KEY: &str = "INIT_KEY";

trinci_sdk::app_export!(init, before_transfer, after_transfer, echo);

/// Initialize the contract on the account
fn init(ctx: AppContext, _args: PackedValue) -> WasmResult<()> {
    // eg: Allows only the account owner
    // to initialize the contract on the account
    if ctx.caller != ctx.owner {
        return Err(WasmError::new("not authorized"));
    }
    trinci_sdk::store_data(INIT_KEY, &[1]);
    Ok(())
}

/// Prevent the account to transfer asset if the condition are not satisfied
fn before_transfer(ctx: AppContext, _args: TransferArgs) -> WasmResult<bool> {
    // Prevent direct call of this method
    if ctx.depth == 0 {
        return Err(WasmError::new("cannot be called directly"));
    }

    // Check if the account is initialized
    if trinci_sdk::load_data(INIT_KEY).is_empty() {
        return Err(WasmError::new("not initialized"));
    };

    Ok(false)
}

/// Method called from an advanced asset after the performing of a transfer
fn after_transfer(ctx: AppContext, _args: TransferArgs) -> WasmResult<bool> {
    // Prevent direct call of this method
    if ctx.depth == 0 {
        return Err(WasmError::new("cannot be called directly"));
    }

    // Check if the account is initialized
    if trinci_sdk::load_data(INIT_KEY).is_empty() {
        return Err(WasmError::new("not initialized"));
    };

    Ok(true)
}

/// Echo.
fn echo<'a>(_ctx: AppContext, args: EchoArgs<'a>) -> WasmResult<EchoArgs<'a>> {
    Ok(args)
}

#[cfg(test)]
mod tests {

    use super::*;
    use trinci_sdk::not_wasm;

    const CALLER_ID: &str = "Caller_Wd7RqEzdV3gKjqXN1kGBgYxFWsxajjguLkyTjy7";
    const OWNER_ID: &str = "Owner_ijWd7RqEzdV3gKjqXN1kGBgYxFWsxajjguLksdfs";

    #[test]
    fn test_echo() {
        let ctx = not_wasm::create_app_context(CALLER_ID, CALLER_ID);

        let args = EchoArgs {
            data: "Hello, Trinci!",
        };

        let res = not_wasm::call_wrap(echo, ctx, args.clone()).unwrap();

        assert_eq!(res, args);
    }

    #[test]
    fn test_not_authorized_init() {
        let ctx = not_wasm::create_app_context(OWNER_ID, CALLER_ID);

        let args = PackedValue::default();

        let err = not_wasm::call_wrap(init, ctx, args).unwrap_err();

        assert_eq!(err.to_string(), "not authorized");
    }

    #[test]
    fn test_init() {
        let ctx = not_wasm::create_app_context(OWNER_ID, OWNER_ID);

        let args = PackedValue::default();

        not_wasm::call_wrap(init, ctx, args).unwrap();

        let init_data = not_wasm::get_account_data(OWNER_ID, INIT_KEY);

        assert_eq!(init_data, &[1]);
    }

    #[test]
    fn test_direct_call_after_transfer() {
        let ctx = not_wasm::create_app_context(OWNER_ID, OWNER_ID);

        let args = TransferArgs {
            from: "account_1",
            to: "account_2",
            units: 100,
            data: None,
        };

        let err = not_wasm::call_wrap(after_transfer, ctx, args).unwrap_err();

        assert_eq!(err.to_string(), "cannot be called directly");
    }

    #[test]
    fn test_after_transfer() {
        let mut ctx = not_wasm::create_app_context(OWNER_ID, OWNER_ID);
        ctx.depth = 1;

        not_wasm::set_account_data(OWNER_ID, INIT_KEY, &[1]);

        let args = TransferArgs {
            from: "account_1",
            to: "account_2",
            units: 100,
            data: None,
        };

        let res = not_wasm::call_wrap(after_transfer, ctx, args).unwrap();

        assert_eq!(res, true);
    }

    #[test]
    fn test_not_initialized_before_transfer() {
        let mut ctx = not_wasm::create_app_context(OWNER_ID, OWNER_ID);
        ctx.depth = 1;

        let args = TransferArgs {
            from: "account_1",
            to: "account_2",
            units: 100,
            data: None,
        };

        let err = not_wasm::call_wrap(before_transfer, ctx, args).unwrap_err();

        assert_eq!(err.to_string(), "not initialized");
    }

    #[test]
    fn test_before_transfer() {
        let mut ctx = not_wasm::create_app_context(OWNER_ID, OWNER_ID);
        ctx.depth = 1;

        not_wasm::set_account_data(OWNER_ID, INIT_KEY, &[1]);

        let args = TransferArgs {
            from: "account_1",
            to: "account_2",
            units: 100,
            data: None,
        };

        let res = not_wasm::call_wrap(before_transfer, ctx, args).unwrap();

        assert_eq!(res, true);
    }
}
