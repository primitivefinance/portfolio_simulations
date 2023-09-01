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
                    ::std::borrow::ToOwned::to_owned("execute_x_to_y"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute_x_to_y"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x_token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y_token"),
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
                (
                    ::std::borrow::ToOwned::to_owned("execute_y_to_x"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute_y_to_x"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("x_token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("y_token"),
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
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x19W`@Qa\r\xBF\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10\x15a\0qW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x805`\xE0\x1C\x80c-cV\x86\x14a\x05*Wc\x8C\xF5vA\x14a\0\x93WPa\0\x0EV[4a\x05%Wa\x01@6`\x03\x19\x01\x12a\x044Wa\0\xADa\n\x93V[a\0\xB5a\n\xAEV[\x91a\0\xBEa\n\xC4V[a\0\xC6a\n\xDAV[\x92a\0\xD06a\x0B<V[\x93`\x01`\x01`\xA0\x1B\x03\x86\x16;\x15a\x03RW`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01Ra\x01$5`D\x82\x01R\x84\x81`d\x81\x83`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xF1\x80\x15a\x05\x06W\x90\x85\x91a\x05\x11W[PP`\x01`\x01`\xA0\x1B\x03\x86\x16;\x15a\x03RW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Ra\x01$5`$\x82\x01R\x84\x81\x80`D\x81\x01\x03\x81\x83`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xF1\x80\x15a\x05\x06W\x90\x85\x91a\x04\xF2W[PP`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03RW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Ra\x01$5`$\x82\x01R\x90\x84\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\xD3W\x90\x84\x91a\x04\xDEW[PP`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x82`$\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x91\x82\x15a\x04\xD3W\x84\x92a\x04\x9FW[P`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x91\x84\x91\x83\x91`D\x91\x83\x91\x85\x91\x16Z\xF1\x80\x15a\x04xW\x90\x83\x91a\x04\x8BW[P\x93\x92\x93P\x81\x93\x82\x93`\x01`\x01`\x80\x1B\x03` \x83\x01Q\x16\x94`\x01[\x15a\x03WW[\x93\x94\x85\x94\x90`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15a\x03RW`@Qc<\xCE\xB8o`\xE2\x1B\x81R``\x80\x82\x80a\x02\xBF\x89`\x04\x83\x01a\x0C\x98V[\x03\x81\x8B`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xF1\x91\x82a\x03&W[PPa\x03\x18Wa'\x0F\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x03\x04Wa'\x10a\x02\xFD\x91\x04\x97a\x0C\xE5V[\x96\x95a\x02\x85V[cNH{q`\xE0\x1B\x86R`\x11`\x04R`$\x86\xFD[\x96\x90Pa\x02\xFD`\x01\x91a\x0C\xE5V[\x81a\x03E\x92\x90=\x10a\x03KW[a\x03=\x81\x83a\x0B\x1AV[\x81\x01\x90a\x0ClV[Pa\x02\xD5V[P=a\x033V[a\x0C\x19V[`\x14\x87\x10\x80a\x04\x83W[a\x02\x8BW\x90a\x03p\x85\x92a\r\nV[`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x82`$\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x91\x82\x15a\x04xW\x83\x92a\x04DW[P\x81a\x03\xCDW[Pa\x03\xCA\x90a\x01$5\x10a\rLV[\x80\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x83\x91\x82\x90\x82\x90`D\x90\x82\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x049Wa\x04 W[Pa\x03\xBBV[a\x04)\x90a\n\xF0V[a\x044W\x81\x83a\x04\x1AV[a\nCV[`@Q=\x84\x82>=\x90\xFD[\x90\x91P` \x81=` \x11a\x04pW[\x81a\x04`` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ\x90\x83a\x03\xB4V[=\x91Pa\x04SV[`@Q=\x85\x82>=\x90\xFD[P\x80\x15a\x03aV[a\x04\x94\x90a\n\xF0V[a\x044W\x818a\x02jV[\x90\x91P` \x81=` \x11a\x04\xCBW[\x81a\x04\xBB` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ\x908a\x02\x17V[=\x91Pa\x04\xAEV[`@Q=\x86\x82>=\x90\xFD[a\x04\xE7\x90a\n\xF0V[a\x044W\x828a\x01\xD1V[a\x04\xFB\x90a\n\xF0V[a\x044W\x838a\x01~V[`@Q=\x87\x82>=\x90\xFD[a\x05\x1A\x90a\n\xF0V[a\x044W\x838a\x01#V[a\t\xF3V[P4a\t\xF3Wa\x01 6`\x03\x19\x01\x12a\x044Wa\x05Ea\n\x93V[\x90a\x05Na\n\xAEV[\x90a\x05Wa\n\xC4V[a\x05_a\n\xDAV[\x92a\x05i6a\x0B<V[\x80Q\x90\x91\x90`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x91\x90\x91R\x84\x81`d\x81\x83`\x01`\x01`\xA0\x1B\x03\x87\x16Z\xF1\x80\x15a\x05\x06Wa\t\xE0W[P\x81Q`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x90\x85\x90\x82\x90`D\x90\x82\x90\x84\x90\x87\x16Z\xF1\x80\x15a\x05\x06W\x90\x85\x91a\t\xCCW[PP`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\x05\x06W\x85\x91a\t\x9AW[P`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x85\x81\x80`D\x81\x01\x03\x81\x83`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xF1\x80\x15a\t\x8FW\x90\x86\x91a\t{W[P\x96\x95\x96P\x84\x96\x85\x96`\x01`\x01`\x80\x1B\x03` \x86\x01Q\x16\x97`\x01[\x15a\x07\x9AW[\x96\x97\x88\x97\x90`\x01`\x01`\xA0\x1B\x03\x88\x16;\x15a\x03RW`@Qc<\xCE\xB8o`\xE2\x1B\x81R``\x80\x82\x8B\x81\x8E\x81a\x07\x19\x8F`\x04\x83\x01a\x0C\x98V[\x03\x92`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82a\x07~W[PPa\x07pWa'\x0F\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x07\\Wa'\x10a\x07U\x91\x04\x9Aa\x0C\xE5V[\x99\x98a\x06\xDCV[cNH{q`\xE0\x1B\x89R`\x11`\x04R`$\x89\xFD[\x99\x90Pa\x07U`\x01\x91a\x0C\xE5V[\x81a\x07\x94\x92\x90=\x10a\x03KWa\x03=\x81\x83a\x0B\x1AV[Pa\x07-V[`\x14\x8A\x10\x80a\tsW[a\x06\xE2W\x87\x95P\x90a\x07\xB7\x89\x93\x92a\r\nV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80;\x15a\x03RW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x87\x91\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x049Wa\t_W[PP`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x93\x90\x93R\x85\x92\x91\x83\x91\x83\x91`D\x91\x83\x91\x85\x91\x16Z\xF1\x80\x15a\x049Wa\tKW[PP`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91` \x83`$\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x92\x83\x15a\x04\xD3W\x84\x93a\t\x17W[P\x82a\x08\xB0W[Pa\x03\xCA\x91\x11a\rLV[`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x84\x91\x82\x90\x82\x90`D\x90\x82\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x049Wa\t\x03W[Pa\x08\xA5V[a\t\x0C\x90a\n\xF0V[a\x044W\x82\x84a\x08\xFDV[\x90\x92P` \x81=` \x11a\tCW[\x81a\t3` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ\x91\x84a\x08\x9EV[=\x91Pa\t&V[a\tT\x90a\n\xF0V[a\x044W\x82\x84a\x08XV[a\th\x90a\n\xF0V[a\x044W\x85\x87a\x08\x05V[P\x80\x15a\x07\xA4V[a\t\x84\x90a\n\xF0V[a\x044W\x848a\x06\xC1V[`@Q=\x88\x82>=\x90\xFD[\x90P` \x81=` \x11a\t\xC4W[\x81a\t\xB5` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ8a\x06iV[=\x91Pa\t\xA8V[a\t\xD5\x90a\n\xF0V[a\x044W\x838a\x06$V[a\t\xEC\x90\x94\x91\x94a\n\xF0V[\x928a\x05\xC5V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[`d5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0B\x04W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0B\x04W`@RV[`\xA0\x90`\x83\x19\x01\x12a\x0B\xC8W`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90`\xA0\x81\x01\x83\x81\x11\x82\x82\x10\x17a\x0B\x04W`@R\x80\x92`\x845`\x01`\x01`\x80\x1B\x03\x90\x81\x81\x16\x81\x03a\n\xA9W\x83R`\xA45\x90\x81\x16\x81\x03a\n\xA9W` \x83\x01R`\xC45\x80\x15\x15\x81\x03a\n\xA9W`@\x83\x01R`\xE45\x90\x81\x16\x81\x03a\n\xA9W``\x82\x01Ra\x01\x045\x90\x81\x15\x15\x82\x03a\n\xA9W`\x80\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x044W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\n\xA9W\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[`\0\x19\x81\x14a\x0C\xF4W`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x15a\r\x11WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x14\xDD\xD8\\\x08\x1B\x9B\xDD\x08\x1C\xDDX\xD8\xD9\\\xDC\xD9\x9D[`j\x1B`D\x82\x01R`d\x90\xFD[\x15a\rSWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot profitable`\x90\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 \x07\xBE\x13t=\xB5\x8E$]\x8D6pb*5\xEE2^\xE0\x08\\\x81\x05\xBF\xEFh\xD7\xA3\x08'\xD0\x03dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static ATOMICARB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0qW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x805`\xE0\x1C\x80c-cV\x86\x14a\x05*Wc\x8C\xF5vA\x14a\0\x93WPa\0\x0EV[4a\x05%Wa\x01@6`\x03\x19\x01\x12a\x044Wa\0\xADa\n\x93V[a\0\xB5a\n\xAEV[\x91a\0\xBEa\n\xC4V[a\0\xC6a\n\xDAV[\x92a\0\xD06a\x0B<V[\x93`\x01`\x01`\xA0\x1B\x03\x86\x16;\x15a\x03RW`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01Ra\x01$5`D\x82\x01R\x84\x81`d\x81\x83`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xF1\x80\x15a\x05\x06W\x90\x85\x91a\x05\x11W[PP`\x01`\x01`\xA0\x1B\x03\x86\x16;\x15a\x03RW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01Ra\x01$5`$\x82\x01R\x84\x81\x80`D\x81\x01\x03\x81\x83`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xF1\x80\x15a\x05\x06W\x90\x85\x91a\x04\xF2W[PP`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x03RW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x16`\x04\x82\x01Ra\x01$5`$\x82\x01R\x90\x84\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x04\xD3W\x90\x84\x91a\x04\xDEW[PP`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x82`$\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x91\x82\x15a\x04\xD3W\x84\x92a\x04\x9FW[P`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x93\x90\x93R\x91\x84\x91\x83\x91`D\x91\x83\x91\x85\x91\x16Z\xF1\x80\x15a\x04xW\x90\x83\x91a\x04\x8BW[P\x93\x92\x93P\x81\x93\x82\x93`\x01`\x01`\x80\x1B\x03` \x83\x01Q\x16\x94`\x01[\x15a\x03WW[\x93\x94\x85\x94\x90`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15a\x03RW`@Qc<\xCE\xB8o`\xE2\x1B\x81R``\x80\x82\x80a\x02\xBF\x89`\x04\x83\x01a\x0C\x98V[\x03\x81\x8B`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xF1\x91\x82a\x03&W[PPa\x03\x18Wa'\x0F\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x03\x04Wa'\x10a\x02\xFD\x91\x04\x97a\x0C\xE5V[\x96\x95a\x02\x85V[cNH{q`\xE0\x1B\x86R`\x11`\x04R`$\x86\xFD[\x96\x90Pa\x02\xFD`\x01\x91a\x0C\xE5V[\x81a\x03E\x92\x90=\x10a\x03KW[a\x03=\x81\x83a\x0B\x1AV[\x81\x01\x90a\x0ClV[Pa\x02\xD5V[P=a\x033V[a\x0C\x19V[`\x14\x87\x10\x80a\x04\x83W[a\x02\x8BW\x90a\x03p\x85\x92a\r\nV[`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90` \x82`$\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x91\x82\x15a\x04xW\x83\x92a\x04DW[P\x81a\x03\xCDW[Pa\x03\xCA\x90a\x01$5\x10a\rLV[\x80\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x83\x91\x82\x90\x82\x90`D\x90\x82\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x049Wa\x04 W[Pa\x03\xBBV[a\x04)\x90a\n\xF0V[a\x044W\x81\x83a\x04\x1AV[a\nCV[`@Q=\x84\x82>=\x90\xFD[\x90\x91P` \x81=` \x11a\x04pW[\x81a\x04`` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ\x90\x83a\x03\xB4V[=\x91Pa\x04SV[`@Q=\x85\x82>=\x90\xFD[P\x80\x15a\x03aV[a\x04\x94\x90a\n\xF0V[a\x044W\x818a\x02jV[\x90\x91P` \x81=` \x11a\x04\xCBW[\x81a\x04\xBB` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ\x908a\x02\x17V[=\x91Pa\x04\xAEV[`@Q=\x86\x82>=\x90\xFD[a\x04\xE7\x90a\n\xF0V[a\x044W\x828a\x01\xD1V[a\x04\xFB\x90a\n\xF0V[a\x044W\x838a\x01~V[`@Q=\x87\x82>=\x90\xFD[a\x05\x1A\x90a\n\xF0V[a\x044W\x838a\x01#V[a\t\xF3V[P4a\t\xF3Wa\x01 6`\x03\x19\x01\x12a\x044Wa\x05Ea\n\x93V[\x90a\x05Na\n\xAEV[\x90a\x05Wa\n\xC4V[a\x05_a\n\xDAV[\x92a\x05i6a\x0B<V[\x80Q\x90\x91\x90`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x91\x90\x91R\x84\x81`d\x81\x83`\x01`\x01`\xA0\x1B\x03\x87\x16Z\xF1\x80\x15a\x05\x06Wa\t\xE0W[P\x81Q`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x90\x85\x90\x82\x90`D\x90\x82\x90\x84\x90\x87\x16Z\xF1\x80\x15a\x05\x06W\x90\x85\x91a\t\xCCW[PP`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R` \x81`$\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\x05\x06W\x85\x91a\t\x9AW[P`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x16`\x04\x82\x01R`$\x81\x01\x82\x90R\x85\x81\x80`D\x81\x01\x03\x81\x83`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xF1\x80\x15a\t\x8FW\x90\x86\x91a\t{W[P\x96\x95\x96P\x84\x96\x85\x96`\x01`\x01`\x80\x1B\x03` \x86\x01Q\x16\x97`\x01[\x15a\x07\x9AW[\x96\x97\x88\x97\x90`\x01`\x01`\xA0\x1B\x03\x88\x16;\x15a\x03RW`@Qc<\xCE\xB8o`\xE2\x1B\x81R``\x80\x82\x8B\x81\x8E\x81a\x07\x19\x8F`\x04\x83\x01a\x0C\x98V[\x03\x92`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82a\x07~W[PPa\x07pWa'\x0F\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x07\\Wa'\x10a\x07U\x91\x04\x9Aa\x0C\xE5V[\x99\x98a\x06\xDCV[cNH{q`\xE0\x1B\x89R`\x11`\x04R`$\x89\xFD[\x99\x90Pa\x07U`\x01\x91a\x0C\xE5V[\x81a\x07\x94\x92\x90=\x10a\x03KWa\x03=\x81\x83a\x0B\x1AV[Pa\x07-V[`\x14\x8A\x10\x80a\tsW[a\x06\xE2W\x87\x95P\x90a\x07\xB7\x89\x93\x92a\r\nV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x80;\x15a\x03RW`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x87\x91\x82\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x049Wa\t_W[PP`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x93\x90\x93R\x85\x92\x91\x83\x91\x83\x91`D\x91\x83\x91\x85\x91\x16Z\xF1\x80\x15a\x049Wa\tKW[PP`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\x03RW`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x91` \x83`$\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x92\x83\x15a\x04\xD3W\x84\x93a\t\x17W[P\x82a\x08\xB0W[Pa\x03\xCA\x91\x11a\rLV[`\x01`\x01`\xA0\x1B\x03\x81\x16;\x15a\x03RW`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x83\x90R\x84\x91\x82\x90\x82\x90`D\x90\x82\x90\x84\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x80\x15a\x049Wa\t\x03W[Pa\x08\xA5V[a\t\x0C\x90a\n\xF0V[a\x044W\x82\x84a\x08\xFDV[\x90\x92P` \x81=` \x11a\tCW[\x81a\t3` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ\x91\x84a\x08\x9EV[=\x91Pa\t&V[a\tT\x90a\n\xF0V[a\x044W\x82\x84a\x08XV[a\th\x90a\n\xF0V[a\x044W\x85\x87a\x08\x05V[P\x80\x15a\x07\xA4V[a\t\x84\x90a\n\xF0V[a\x044W\x848a\x06\xC1V[`@Q=\x88\x82>=\x90\xFD[\x90P` \x81=` \x11a\t\xC4W[\x81a\t\xB5` \x93\x83a\x0B\x1AV[\x81\x01\x03\x12a\x044WQ8a\x06iV[=\x91Pa\t\xA8V[a\t\xD5\x90a\n\xF0V[a\x044W\x838a\x06$V[a\t\xEC\x90\x94\x91\x94a\n\xF0V[\x928a\x05\xC5V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[`D5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[`d5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\n\xA9WV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0B\x04W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0B\x04W`@RV[`\xA0\x90`\x83\x19\x01\x12a\x0B\xC8W`@Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90`\xA0\x81\x01\x83\x81\x11\x82\x82\x10\x17a\x0B\x04W`@R\x80\x92`\x845`\x01`\x01`\x80\x1B\x03\x90\x81\x81\x16\x81\x03a\n\xA9W\x83R`\xA45\x90\x81\x16\x81\x03a\n\xA9W` \x83\x01R`\xC45\x80\x15\x15\x81\x03a\n\xA9W`@\x83\x01R`\xE45\x90\x81\x16\x81\x03a\n\xA9W``\x82\x01Ra\x01\x045\x90\x81\x15\x15\x82\x03a\n\xA9W`\x80\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x81``\x91\x03\x12a\x044W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\n\xA9W\x91`@` \x83\x01Q\x92\x01Q\x90V[\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[`\0\x19\x81\x14a\x0C\xF4W`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x15a\r\x11WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x14\xDD\xD8\\\x08\x1B\x9B\xDD\x08\x1C\xDDX\xD8\xD9\\\xDC\xD9\x9D[`j\x1B`D\x82\x01R`d\x90\xFD[\x15a\rSWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01RmNot profitable`\x90\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 \x07\xBE\x13t=\xB5\x8E$]\x8D6pb*5\xEE2^\xE0\x08\\\x81\x05\xBF\xEFh\xD7\xA3\x08'\xD0\x03dsolcC\0\x08\x13\x003";
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
        ///Calls the contract's `execute_x_to_y` (0x8cf57641) function
        pub fn execute_x_to_y(
            &self,
            x_token: ::ethers::core::types::Address,
            y_token: ::ethers::core::types::Address,
            portfolio: ::ethers::core::types::Address,
            exchange: ::ethers::core::types::Address,
            order: Order,
            exchange_input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [140, 245, 118, 65],
                    (x_token, y_token, portfolio, exchange, order, exchange_input),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute_y_to_x` (0x2d635686) function
        pub fn execute_y_to_x(
            &self,
            x_token: ::ethers::core::types::Address,
            y_token: ::ethers::core::types::Address,
            portfolio: ::ethers::core::types::Address,
            exchange: ::ethers::core::types::Address,
            order: Order,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [45, 99, 86, 134],
                    (x_token, y_token, portfolio, exchange, order),
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
    ///Container type for all input parameters for the `execute_x_to_y` function with signature `execute_x_to_y(address,address,address,address,(uint128,uint128,bool,uint64,bool),uint256)` and selector `0x8cf57641`
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
        name = "execute_x_to_y",
        abi = "execute_x_to_y(address,address,address,address,(uint128,uint128,bool,uint64,bool),uint256)"
    )]
    pub struct ExecuteXToYCall {
        pub x_token: ::ethers::core::types::Address,
        pub y_token: ::ethers::core::types::Address,
        pub portfolio: ::ethers::core::types::Address,
        pub exchange: ::ethers::core::types::Address,
        pub order: Order,
        pub exchange_input: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `execute_y_to_x` function with signature `execute_y_to_x(address,address,address,address,(uint128,uint128,bool,uint64,bool))` and selector `0x2d635686`
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
        name = "execute_y_to_x",
        abi = "execute_y_to_x(address,address,address,address,(uint128,uint128,bool,uint64,bool))"
    )]
    pub struct ExecuteYToXCall {
        pub x_token: ::ethers::core::types::Address,
        pub y_token: ::ethers::core::types::Address,
        pub portfolio: ::ethers::core::types::Address,
        pub exchange: ::ethers::core::types::Address,
        pub order: Order,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AtomicArbCalls {
        ExecuteXToY(ExecuteXToYCall),
        ExecuteYToX(ExecuteYToXCall),
    }
    impl ::ethers::core::abi::AbiDecode for AtomicArbCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <ExecuteXToYCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteXToY(decoded));
            }
            if let Ok(decoded)
                = <ExecuteYToXCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ExecuteYToX(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AtomicArbCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ExecuteXToY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExecuteYToX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AtomicArbCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ExecuteXToY(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteYToX(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteXToYCall> for AtomicArbCalls {
        fn from(value: ExecuteXToYCall) -> Self {
            Self::ExecuteXToY(value)
        }
    }
    impl ::core::convert::From<ExecuteYToXCall> for AtomicArbCalls {
        fn from(value: ExecuteYToXCall) -> Self {
            Self::ExecuteYToX(value)
        }
    }
}
