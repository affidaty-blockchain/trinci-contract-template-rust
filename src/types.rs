use serde_derive::{Deserialize, Serialize};

// Echo Args
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq, Clone))]
pub struct EchoArgs<'a> {
    pub data: &'a str,
}

#[cfg(test)]
pub(crate) mod tests {

    use super::*;

    const ECHO_ARGS_HEX: &str = "91ae48656c6c6f2c205472696e636921";

    #[test]
    fn echo_args_serialize() {
        let args = EchoArgs {
            data: "Hello, Trinci!",
        };

        let buf = trinci_sdk::rmp_serialize(&args).unwrap();

        assert_eq!(hex::encode(&buf), ECHO_ARGS_HEX);
    }

    #[test]
    fn echo_args_deserialize() {
        let expected = EchoArgs {
            data: "Hello, Trinci!",
        };

        let buf = hex::decode(ECHO_ARGS_HEX).unwrap();

        let args: EchoArgs = trinci_sdk::rmp_deserialize(&buf).unwrap();

        assert_eq!(args, expected);
    }
}
