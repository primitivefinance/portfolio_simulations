pub use atomic_arb::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod atomic_arb {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("otherToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("portfolio"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exchange"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("order"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct Order"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("exchangeInput"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static ATOMICARB_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x19W`@Qa\x08\x15\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0rW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x90\x815`\xE0\x1Cc\xB6\xDE\x89\xA9\x14a\0\x8AWPa\0\x0FV[4a\x06\xA3WPa\x01@6`\x03\x19\x01\x12a\x04\x08W`\x01`\x01`\xA0\x1B\x03\x90`\x045\x80\x83\x16\x90\x03a\x06\xA0W`$5\x82\x81\x16\x81\x03a\x06\x9CW\x82`D5\x16`D5\x03a\x06\x9CW\x82`d5\x16`d5\x03a\x06\x9CW`\xA06`\x83\x19\x01\x12a\x06KW`@Q\x90`\xA0\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x067W`@R`\x845`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x03a\x063W\x82R`\xA45`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x03a\x063W` \x83\x01R`\xC45\x80\x15\x15\x81\x03a\x063W`@\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xE45\x16`\xE45\x03a\x06/W`\xE45``\x83\x01Ra\x01\x045\x15\x15a\x01\x045\x03a\x06/Wa\x01\x045`\x80\x83\x01R\x83`\x045\x16;\x15a\x04\x15W`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01Ra\x01$5`D\x82\x01R\x83\x81`d\x81\x83\x89`\x045\x16Z\xF1\x80\x15a\x05\xE9Wa\x06\x1CW[P\x83`\x045\x16;\x15a\x04\x15W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03`d5\x16`\x04\x82\x01Ra\x01$5`$\x82\x01R\x83\x81\x80`D\x81\x01\x03\x81\x83\x89`\x045\x16Z\xF1\x80\x15a\x05\xE9W\x90\x84\x91a\x06\x08W[PP\x83`d5\x16;\x15a\x04\x15W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x04\x805`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01Ra\x01$5`$\x82\x01R\x83\x81`D\x81\x83`d5\x8A\x16Z\xF1\x80\x15a\x05\xE9W\x90\x84\x91a\x05\xF4W[PP\x83\x81\x16;\x15a\x04\x15W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x82`$\x81\x88\x85\x16Z\xFA\x91\x82\x15a\x05\xE9W\x84\x92a\x05\xB5W[P\x84\x81\x16;\x15a\x04\x15W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`D\x805`\x01`\x01`\xA0\x1B\x03\x16`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x91\x84\x91\x83\x91\x90\x82\x90\x84\x90\x89\x16Z\xF1\x80\x15a\x05SW\x90\x83\x91a\x05\xA1W[PP` \x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x92\x82\x91\x82`\x01[\x15a\x04\x1AW[\x84\x95\x83`D5\x16;\x15a\x04\x15W`@Qc<\xCE\xB8o`\xE2\x1B\x81R`\x01`\x01`\x80\x1B\x03\x84Q\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03` \x85\x01Q\x16`$\x82\x01R`@\x84\x01Q\x15\x15`D\x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x85\x01Q\x16`d\x82\x01R`\x80\x84\x01Q\x15\x15`\x84\x82\x01R``\x81`\xA4\x81\x8A\x89`D5\x16Z\xF1\x90\x81a\x03\xCCW[Pa\x03\xC2Wa\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x03\xAEWa\x03\xE8\x90\x04\x93[`\0\x19\x81\x14a\x03\xAEW`\x01\x01\x93\x95a\x02\xF7V[cNH{q`\xE0\x1B\x86R`\x11`\x04R`$\x86\xFD[\x93`\x01\x91Pa\x03\x9BV[``\x81=``\x11a\x04\rW[\x81a\x03\xE5``\x93\x83a\x07jV[\x81\x01\x03\x12a\x04\x08WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x04\x04W8a\x03{V[\x86\x80\xFD[a\x06\xF0V[=\x91Pa\x03\xD8V[a\x07\x8CV[`\n\x84\x10\x80a\x05\x99W[a\x02\xFDW\x82\x85\x91\x15a\x05^W\x80`\x045\x16;\x15a\x04\x15W`@Q\x90cp\xA0\x821`\xE0\x1B\x82R0`\x04\x83\x01R` \x82`$\x81\x84`\x045\x16Z\xFA\x91\x82\x15a\x05SW\x83\x92a\x05\x1FW[P\x81a\x04\xB5W[Pa\x01$5\x10\x15a\x04\x7FW\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot profitable`\x90\x1B`D\x82\x01R`d\x90\xFD[\x80`\x045\x16;\x15a\x04\x15W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x83\x91\x82\x90\x82\x90\x81\x83\x81`D\x81\x01\x03\x92`\x045\x16Z\xF1\x80\x15a\x05\x14Wa\x05\0W[Pa\x04qV[a\x05\t\x90a\x07@V[a\x04\x08W\x81\x83a\x04\xFAV[`@Q=\x84\x82>=\x90\xFD[\x90\x91P` \x81=` \x11a\x05KW[\x81a\x05;` \x93\x83a\x07jV[\x81\x01\x03\x12a\x04\x08WQ\x90\x83a\x04jV[=\x91Pa\x05.V[`@Q=\x85\x82>=\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x14\xDD\xD8\\\x08\x1B\x9B\xDD\x08\x1C\xDDX\xD8\xD9\\\xDC\xD9\x9D[`j\x1B`D\x82\x01R`d\x90\xFD[P\x80\x15a\x04$V[a\x05\xAA\x90a\x07@V[a\x04\x08W\x818a\x02\xE0V[\x90\x91P` \x81=` \x11a\x05\xE1W[\x81a\x05\xD1` \x93\x83a\x07jV[\x81\x01\x03\x12a\x04\x08WQ\x908a\x02\x93V[=\x91Pa\x05\xC4V[`@Q=\x86\x82>=\x90\xFD[a\x05\xFD\x90a\x07@V[a\x04\x08W\x828a\x02[V[a\x06\x11\x90a\x07@V[a\x04\x08W\x828a\x02\x0CV[a\x06(\x90\x93\x91\x93a\x07@V[\x918a\x01\xBAV[\x82\x80\xFD[\x83\x80\xFD[cNH{q`\xE0\x1B\x84R`A`\x04R`$\x84\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[P\x80\xFD[\x80\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07TW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07TW`@RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD\xFE\xA2dipfsX\"\x12 \x8D\xD1$e\xE9JA\xDAD1\xB4\x93\xF9RG\xAE\x0B\xE5\x0CG\xF0\x15\x7Fu\x82>\x8AA\xBC\x08\xB8vdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ATOMICARB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80\x80`@R`\x046\x10\x15a\0rW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x90\x815`\xE0\x1Cc\xB6\xDE\x89\xA9\x14a\0\x8AWPa\0\x0FV[4a\x06\xA3WPa\x01@6`\x03\x19\x01\x12a\x04\x08W`\x01`\x01`\xA0\x1B\x03\x90`\x045\x80\x83\x16\x90\x03a\x06\xA0W`$5\x82\x81\x16\x81\x03a\x06\x9CW\x82`D5\x16`D5\x03a\x06\x9CW\x82`d5\x16`d5\x03a\x06\x9CW`\xA06`\x83\x19\x01\x12a\x06KW`@Q\x90`\xA0\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x067W`@R`\x845`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x03a\x063W\x82R`\xA45`\x01`\x01`\x80\x1B\x03\x81\x16\x81\x03a\x063W` \x83\x01R`\xC45\x80\x15\x15\x81\x03a\x063W`@\x83\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xE45\x16`\xE45\x03a\x06/W`\xE45``\x83\x01Ra\x01\x045\x15\x15a\x01\x045\x03a\x06/Wa\x01\x045`\x80\x83\x01R\x83`\x045\x16;\x15a\x04\x15W`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01Ra\x01$5`D\x82\x01R\x83\x81`d\x81\x83\x89`\x045\x16Z\xF1\x80\x15a\x05\xE9Wa\x06\x1CW[P\x83`\x045\x16;\x15a\x04\x15W`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03`d5\x16`\x04\x82\x01Ra\x01$5`$\x82\x01R\x83\x81\x80`D\x81\x01\x03\x81\x83\x89`\x045\x16Z\xF1\x80\x15a\x05\xE9W\x90\x84\x91a\x06\x08W[PP\x83`d5\x16;\x15a\x04\x15W`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x04\x805`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01Ra\x01$5`$\x82\x01R\x83\x81`D\x81\x83`d5\x8A\x16Z\xF1\x80\x15a\x05\xE9W\x90\x84\x91a\x05\xF4W[PP\x83\x81\x16;\x15a\x04\x15W`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x82`$\x81\x88\x85\x16Z\xFA\x91\x82\x15a\x05\xE9W\x84\x92a\x05\xB5W[P\x84\x81\x16;\x15a\x04\x15W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`D\x805`\x01`\x01`\xA0\x1B\x03\x16`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x91\x84\x91\x83\x91\x90\x82\x90\x84\x90\x89\x16Z\xF1\x80\x15a\x05SW\x90\x83\x91a\x05\xA1W[PP` \x81\x01Q`\x01`\x01`\x80\x1B\x03\x16\x92\x82\x91\x82`\x01[\x15a\x04\x1AW[\x84\x95\x83`D5\x16;\x15a\x04\x15W`@Qc<\xCE\xB8o`\xE2\x1B\x81R`\x01`\x01`\x80\x1B\x03\x84Q\x16`\x04\x82\x01R`\x01`\x01`\x80\x1B\x03` \x85\x01Q\x16`$\x82\x01R`@\x84\x01Q\x15\x15`D\x82\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x85\x01Q\x16`d\x82\x01R`\x80\x84\x01Q\x15\x15`\x84\x82\x01R``\x81`\xA4\x81\x8A\x89`D5\x16Z\xF1\x90\x81a\x03\xCCW[Pa\x03\xC2Wa\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x03\xAEWa\x03\xE8\x90\x04\x93[`\0\x19\x81\x14a\x03\xAEW`\x01\x01\x93\x95a\x02\xF7V[cNH{q`\xE0\x1B\x86R`\x11`\x04R`$\x86\xFD[\x93`\x01\x91Pa\x03\x9BV[``\x81=``\x11a\x04\rW[\x81a\x03\xE5``\x93\x83a\x07jV[\x81\x01\x03\x12a\x04\x08WQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x03a\x04\x04W8a\x03{V[\x86\x80\xFD[a\x06\xF0V[=\x91Pa\x03\xD8V[a\x07\x8CV[`\n\x84\x10\x80a\x05\x99W[a\x02\xFDW\x82\x85\x91\x15a\x05^W\x80`\x045\x16;\x15a\x04\x15W`@Q\x90cp\xA0\x821`\xE0\x1B\x82R0`\x04\x83\x01R` \x82`$\x81\x84`\x045\x16Z\xFA\x91\x82\x15a\x05SW\x83\x92a\x05\x1FW[P\x81a\x04\xB5W[Pa\x01$5\x10\x15a\x04\x7FW\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot profitable`\x90\x1B`D\x82\x01R`d\x90\xFD[\x80`\x045\x16;\x15a\x04\x15W`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x83\x91\x82\x90\x82\x90\x81\x83\x81`D\x81\x01\x03\x92`\x045\x16Z\xF1\x80\x15a\x05\x14Wa\x05\0W[Pa\x04qV[a\x05\t\x90a\x07@V[a\x04\x08W\x81\x83a\x04\xFAV[`@Q=\x84\x82>=\x90\xFD[\x90\x91P` \x81=` \x11a\x05KW[\x81a\x05;` \x93\x83a\x07jV[\x81\x01\x03\x12a\x04\x08WQ\x90\x83a\x04jV[=\x91Pa\x05.V[`@Q=\x85\x82>=\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x14\xDD\xD8\\\x08\x1B\x9B\xDD\x08\x1C\xDDX\xD8\xD9\\\xDC\xD9\x9D[`j\x1B`D\x82\x01R`d\x90\xFD[P\x80\x15a\x04$V[a\x05\xAA\x90a\x07@V[a\x04\x08W\x818a\x02\xE0V[\x90\x91P` \x81=` \x11a\x05\xE1W[\x81a\x05\xD1` \x93\x83a\x07jV[\x81\x01\x03\x12a\x04\x08WQ\x908a\x02\x93V[=\x91Pa\x05\xC4V[`@Q=\x86\x82>=\x90\xFD[a\x05\xFD\x90a\x07@V[a\x04\x08W\x828a\x02[V[a\x06\x11\x90a\x07@V[a\x04\x08W\x828a\x02\x0CV[a\x06(\x90\x93\x91\x93a\x07@V[\x918a\x01\xBAV[\x82\x80\xFD[\x83\x80\xFD[cNH{q`\xE0\x1B\x84R`A`\x04R`$\x84\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[P\x80\xFD[\x80\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x07TW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x07TW`@RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD\xFE\xA2dipfsX\"\x12 \x8D\xD1$e\xE9JA\xDAD1\xB4\x93\xF9RG\xAE\x0B\xE5\x0CG\xF0\x15\x7Fu\x82>\x8AA\xBC\x08\xB8vdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static ATOMICARB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AtomicArb<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AtomicArb<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AtomicArb<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AtomicArb<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AtomicArb<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AtomicArb)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AtomicArb<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ATOMICARB_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                ATOMICARB_ABI.clone(),
                ATOMICARB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `execute` (0xb6de89a9) function
        pub fn execute(
            &self,
            token: ::ethers::core::types::Address,
            other_token: ::ethers::core::types::Address,
            portfolio: ::ethers::core::types::Address,
            exchange: ::ethers::core::types::Address,
            order: Order,
            exchange_input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [182, 222, 137, 169],
                    (token, other_token, portfolio, exchange, order, exchange_input),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AtomicArb<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(address,address,address,address,(uint128,uint128,bool,uint64,bool),uint256)` and selector `0xb6de89a9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "execute",
        abi = "execute(address,address,address,address,(uint128,uint128,bool,uint64,bool),uint256)"
    )]
    pub struct ExecuteCall {
        pub token: ::ethers::core::types::Address,
        pub other_token: ::ethers::core::types::Address,
        pub portfolio: ::ethers::core::types::Address,
        pub exchange: ::ethers::core::types::Address,
        pub order: Order,
        pub exchange_input: ::ethers::core::types::U256,
    }
}
