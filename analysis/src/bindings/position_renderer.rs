pub use position_renderer::*;
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
pub mod position_renderer {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("uri"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("uri"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("StringsInsufficientHexLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "StringsInsufficientHexLength",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("length"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static POSITIONRENDERER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x19W`@Qa\x17t\x90\x81a\0g\x829\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0rW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`\x005`\xE0\x1Cc\x0E\x894\x1C\x03a\0\x0FW4a\x0FLW` 6`\x03\x19\x01\x12a\r\x05Wb\xFF\xFF\xFF\x80a\0\xA3`\x045a\x17 V[` \x1C\x163;\x15a\r\x1EW`@Q\x90c\x17\x91\xD9\x8F`\xE2\x1B\x82R`\x04\x82\x01R`\x80\x81`$\x813Z\xFA\x90\x81\x15a\r\x12W`\0\x90`\0\x92a\x0F'W[P`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r\x1EW`\0`\x04\x92`@Q\x93\x84\x80\x92c\x95\xD8\x9BA`\xE0\x1B\x82RZ\xFA\x91\x82\x15a\r\x12W`\0\x92a\x0F\nW[P`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r\x1EW`\0`\x04\x92`@Q\x93\x84\x80\x92c\x95\xD8\x9BA`\xE0\x1B\x82RZ\xFA\x91\x82\x15a\r\x12W`\0\x92a\x0E\xE6W[P`8a\x01\xC3\x91`@Q\x93\x84\x91\x7FPrimitive Portfolio LP \0\0\0\0\0\0\0\0\0` \x84\x01Ra\x01\x95\x81Q\x80\x92` `7\x87\x01\x91\x01a\x0F\xE9V[\x82\x01`-`\xF8\x1B`7\x82\x01Ra\x01\xB4\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x0F\xE9V[\x01\x03`\x18\x81\x01\x84R\x01\x82a\x10DV[`@Qa\x01\x80\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x0E\xD0Wa\x03~\x91`@Ra\x01E\x81R\x7F<svg width=\"512\" height=\"512\" fi` \x82\x01R\x7Fll=\"none\" xmlns=\"http://www.w3.o`@\x82\x01R\x7Frg/2000/svg\"><path fill=\"#000\" d``\x82\x01R\x7F=\"M0 0h512v512H0z\"/><path fill-r`\x80\x82\x01R\x7Fule=\"evenodd\" clip-rule=\"evenodd`\xA0\x82\x01R\x7F\" d=\"M339.976 134.664h41.048L256`\xC0\x82\x01R\x7F 340.586 130.976 134.664h41.047V`\xE0\x82\x01R\x7F98H64.143L256 414 447.857 98H339a\x01\0\x82\x01R\x7F.976v36.664Zm-38.759 0V98h-90.43a\x01 \x82\x01R\x7F6v36.664h90.436Z\" fill=\"#fff\"/><a\x01@\x82\x01Rd\x17\xB9\xBB3\x9F`\xD9\x1Ba\x01`\x82\x01Ra\x13lV[\x91a\x03\xD2`:`@Q\x80\x95\x7Fdata:image/svg+xml;base64,\0\0\0\0\0\0` \x83\x01Ra\x03\xC2\x81Q\x80\x92` \x86\x86\x01\x91\x01a\x0F\xE9V[\x81\x01\x03`\x1A\x81\x01\x86R\x01\x84a\x10DV[a\x03\xDD`\x045a\x17 V[` \x1C\x163;\x15a\r\x1EW`@Q\x90c\x17\x91\xD9\x8F`\xE2\x1B\x82R`\x04\x82\x01R`\x80\x81`$\x813Z\xFA\x80\x15a\r\x12W`\0\x91`\0\x91a\x0E\x9CW[P`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\r\x1EW`@Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R`\0\x81`\x04\x81`\x01`\x01`\xA0\x1B\x03\x87\x16Z\xFA\x90\x81\x15a\r\x12W`\0\x91a\x0E\x81W[P`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\r\x1EW`@Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x92`\0\x84`\x04\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\r\x12W`\0\x94a\x0E`W[Pa\x04\x9E\x90a\x16\x1CV[\x90`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\r\x1EW`@Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x91`\0\x83`\x04\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x92\x83\x15a\r\x12W`\0\x93a\x0ECW[P`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\r\x1EW`@Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x92`\0\x84`\x04\x81`\x01`\x01`\xA0\x1B\x03\x89\x16Z\xFA\x90\x81\x15a\r\x12Wa\x06\xA1\x95`\x89\x95`\0\x93a\x0E\x1AW[Pa\x05/\x90a\x16\x1CV[\x92`@Q\x97\x88\x95m\x110\xB9\xB9\xB2\xBA/\xB70\xB6\xB2\x91\x1D\x11`\x91\x1B` \x88\x01Ra\x05a\x81Q\x80\x92` `.\x8B\x01\x91\x01a\x0F\xE9V[a\x08\x8B`\xF2\x1B\x87\x82\x01`.\x81\x01\x91\x90\x91Ro\x110\xB9\xB9\xB2\xBA/\xB9\xBC\xB6\xB17\xB6\x11\x1D\x11`\x81\x1B`0\x82\x01R\x82Q\x92a\x05\xA2\x91\x84\x91`@\x90\x91\x01\x90` \x01a\x0F\xE9V[\x86\x01\x01a\x08\x8B`\xF2\x1B`@\x82\x01Rp\x110\xB9\xB9\xB2\xBA/\xB0\xB2292\xB9\xB9\x91\x1D\x11`y\x1B`B\x82\x01Ra\x05\xDE\x82Q\x80\x93` `S\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`S\x82\x01Rm\x118\xBA\xB7\xBA2\xAF\xB70\xB6\xB2\x91\x1D\x11`\x91\x1B`U\x82\x01Ra\x06\x15\x82Q\x80\x93` `c\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`c\x82\x01Ro\x118\xBA\xB7\xBA2\xAF\xB9\xBC\xB6\xB17\xB6\x11\x1D\x11`\x81\x1B`e\x82\x01Ra\x06N\x82Q\x80\x93` `u\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`u\x82\x01Rp\x118\xBA\xB7\xBA2\xAF\xB0\xB2292\xB9\xB9\x91\x1D\x11`y\x1B`w\x82\x01Ra\x06\x88\x82Q\x80\x93` `\x88\x85\x01\x91\x01a\x0F\xE9V[\x01`\x11`\xF9\x1B`\x88\x82\x01R\x03`i\x81\x01\x84R\x01\x82a\x10DV[3;\x15a\r\x1EW`@Qc\"i|!`\xE2\x1B\x81R`\x04\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90a\x01\0\x82`$\x813Z\xFA\x80\x15a\r\x12W`\0\x90`\0\x91`\0\x94`\0\x92a\r\xC9W[P\x91`r\x91a\x07\x1Ba\x07\x15a\x07\x0Fa\xFF\xFFa\x07\x08\x81a\x08\x1F\x99\x16a\x14\xD9V[\x95\x16a\x14\xD9V[\x97a\x16\x1CV[\x91a\x16\x1CV[\x90`@Q\x96\x87\x93s\x1132\xB2\xAF\xB10\xB9\xB4\xB9\xAF\xB87\xB4\xB7:9\x91\x1D\x11`a\x1B` \x86\x01Ra\x07S\x81Q\x80\x92` `4\x89\x01\x91\x01a\x0F\xE9V[\x84\x01a\x08\x8B`\xF2\x1B`4\x82\x01R\x7F\"priority_fee_basis_points\":\"\0\0\0`6\x82\x01Ra\x07\x9A\x82Q\x80\x93` `S\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`S\x82\x01Rm\x111\xB7\xB7:97\xB662\xB9\x11\x1D\x11`\x91\x1B`U\x82\x01Ra\x07\xD1\x82Q\x80\x93` `c\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`c\x82\x01Rk\x119\xBA90\xBA2\xB3\xBC\x91\x1D\x11`\xA1\x1B`e\x82\x01Ra\x08\x06\x82Q\x80\x93` `q\x85\x01\x91\x01a\x0F\xE9V[\x01`\x11`\xF9\x1B`q\x82\x01R\x03`R\x81\x01\x85R\x01\x83a\x10DV[3;\x15a\r\x1EW`@Qc\"i|!`\xE2\x1B\x81R`\x04\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x01\0\x81`$\x813Z\xFA\x90\x81\x15a\r\x12W`\0\x91a\r\x90W[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\r#W[`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\r\x1EW`\xA0`$\x93`@Q\x94\x85\x80\x92c4\xDB\xC7;`\xE0\x1B\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x16`\x04\x83\x01RZ\xFA\x80\x15a\r\x12W`\0\x93`\0\x91`\0\x95`\0\x92`\0\x92a\x0C\x84W[P\x94a\x0B\xDF\x94a\nz`\x90`\xB3\x96`@\x9B\x96a\x0B\xE4\x9B\x96a\t\x17a\x0CS\x9F\x9Ca\t\x10c\xFF\xFF\xFF\xFFa\t\t\x81a\t\x02`\x01`\x01`\x80\x1B\x03\x82\x96\x16a\x14\xD9V[\x98\x16a\x14\xD9V[\x9B\x16a\x14\xD9V[\x93\x16a\x14\xD9V[\x90\x15a\x0CaW\x8DQa\t(\x81a\x10\x0CV[`\x04\x81Rctrue`\xE0\x1B` \x82\x01R\x91[\x8EQ\x98\x89\x94s\x119\xBA94\xB5\xB2\xAF\xB894\xB1\xB2\xAF\xBB\xB0\xB2\x11\x1D\x11`a\x1B` \x87\x01Ra\tq\x81Q\x80\x92` `4\x8A\x01\x91\x01a\x0F\xE9V[\x85\x01a\x08\x8B`\xF2\x1B`4\x82\x01R\x7F\"volatility_basis_points\":\"\0\0\0\0\0`6\x82\x01Ra\t\xB8\x82Q\x80\x93` `Q\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`Q\x82\x01Rs\x112:\xB90\xBA4\xB7\xB7/\xB9\xB2\xB1\xB7\xB729\x91\x1D\x11`a\x1B`S\x82\x01Ra\t\xF5\x82Q\x80\x93` `g\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`g\x82\x01Ru\x111\xB92\xB0\xBA4\xB7\xB7/\xBA4\xB6\xB2\xB9\xBA0\xB6\xB8\x11\x1D\x11`Q\x1B`i\x82\x01Ra\n4\x82Q\x80\x93` `\x7F\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`\x7F\x82\x01Rn\x114\xB9\xAF\xB82\xB982\xBA:\xB0\xB6\x11\x1D`\x89\x1B`\x81\x82\x01Ra\nk\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x0F\xE9V[\x01\x03`p\x81\x01\x86R\x01\x84a\x10DV[\x88Q\x96\x87\x94h=\x9170\xB6\xB2\x91\x1D\x11`\xB9\x1B` \x87\x01Ra\n\xA5\x81Q\x80\x92` `)\x8A\x01\x91\x01a\x0F\xE9V[\x85\x01j\x11\x16\x114\xB6\xB0\xB3\xB2\x91\x1D\x11`\xA9\x1B`)\x82\x01Ra\n\xCF\x82Q\x80\x93` `4\x85\x01\x91\x01a\x0F\xE9V[\x01\x91\x7F\",\"license\":\"MIT\",\"creator\":\"pri`4\x84\x01Rk\x1BZ]\x1A]\x99K\x99]\x1A\x08\x8B`\xA2\x1B`T\x84\x01R\x7F\"description\":\"Concentrated liqu``\x84\x01R\x7Fidity tokens of a two-token AMM\"`\x80\x84\x01R`\x0B`\xFA\x1B\x92\x83`\xA0\x82\x01Rm\"properties\":{`\x90\x1B`\xA1\x82\x01Ra\x0B\x8F\x82Q\x80\x93` `\xAF\x85\x01\x91\x01a\x0F\xE9V[\x01\x82`\xAF\x82\x01Ra\x0B\xAA\x82Q\x80\x93` `\xB0\x85\x01\x91\x01a\x0F\xE9V[\x01\x90`\xB0\x82\x01Ra\x0B\xC5\x82Q\x80\x93` `\xB1\x85\x01\x91\x01a\x0F\xE9V[\x01a}}`\xF0\x1B`\xB1\x82\x01R\x03`\x93\x81\x01\x84R\x01\x82a\x10DV[a\x13lV[\x81Q\x90a\x0C8`=\x83` \x81\x01\x93\x7Fdata:application/json;base64,\0\0\0\x85Ra\x0C(\x81Q\x80\x92` \x86\x86\x01\x91\x01a\x0F\xE9V[\x81\x01\x03`\x1D\x81\x01\x85R\x01\x83a\x10DV[\x82Q\x93\x84\x92` \x84RQ\x80\x92\x81` \x86\x01R\x85\x85\x01\x90a\x0F\xE9V[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[\x8DQa\x0Cl\x81a\x10\x0CV[`\x05\x81Rdfalse`\xD8\x1B` \x82\x01R\x91a\t;V[\x94\x98\x95\x96PPPPP`\xA0\x81=`\xA0\x11a\r\nW[\x81a\x0C\xA6`\xA0\x93\x83a\x10DV[\x81\x01\x03\x12a\r\x05Wa\x0C\xB7\x81a\x12\x96V[\x92a\x0C\xC4` \x83\x01a\x12\xAAV[\x94a\x0C\xD1`@\x84\x01a\x12\xAAV[\x92`\x80a\x0C\xE0``\x83\x01a\x12\xAAV[\x91\x01Q\x92\x83\x15\x15\x84\x03a\r\0W\x94\x97\x91\x96\x93\x95\x91\x93\x92\x90\x91a\x0B\xDFa\x08\xC4V[`\0\x80\xFD[a\x0F\x99V[=\x91Pa\x0C\x99V[`@Q=`\0\x82>=\x90\xFD[a\x10fV[P3;\x15a\r\x1EW`@QcS\x1E\x17\xB3`\xE0\x1B\x81R` \x81`\x04\x813Z\xFA\x90\x81\x15a\r\x12W`\0\x91a\rVW[Pa\x08oV[\x90P` \x81=` \x11a\r\x88W[\x81a\rq` \x93\x83a\x10DV[\x81\x01\x03\x12a\r\x05Wa\r\x82\x90a\x10\xB9V[\x85a\rPV[=\x91Pa\rdV[a\r\xB4\x91Pa\x01\0=a\x01\0\x11a\r\xC2W[a\r\xAC\x81\x83a\x10DV[\x81\x01\x90a\x12\xCAV[\x96PPPPPPP\x85a\x08^V[P=a\r\xA2V[a\xFF\xFF\x95Pa\x08\x1F\x93P`r\x92Pa\x07\x08\x91Pa\x07\x1Ba\x07\x15a\x07\x0Fa\x0E\0\x89\x94a\x01\0=a\x01\0\x11a\r\xC2Wa\r\xAC\x81\x83a\x10DV[\x90\x9FP\x90\x9C\x90\x9BP\x90\x99P\x97Pa\x06\xE9\x96PPPPPPPV[a\x05/\x91\x93Pa\x0E<\x90=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x81\x01\x90a\x115V[\x92\x90a\x05%V[a\x0EY\x91\x93P=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x91\x87a\x04\xDEV[a\x04\x9E\x91\x94Pa\x0Ez\x90=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x93\x90a\x04\x94V[a\x0E\x96\x91P=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x85a\x04TV[\x90Pa\x0E\xC0\x91P`\x80=`\x80\x11a\x0E\xC9W[a\x0E\xB8\x81\x83a\x10DV[\x81\x01\x90a\x10\xDBV[P\x90P\x84a\x04\x15V[P=a\x0E\xAEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[a\x01\xC3\x91\x92Pa\x0F\x02`8\x91=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x92\x91Pa\x01NV[a\x0F \x91\x92P=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x90\x83a\x01\x15V[\x90Pa\x0FB\x91P`\x80=`\x80\x11a\x0E\xC9Wa\x0E\xB8\x81\x83a\x10DV[P\x91\x90P\x83a\0\xDCV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x0F\xFCWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F\xECV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\0WV[Q\x90`\xFF\x82\x16\x82\x03a\r\0WV[\x91\x90\x82`\x80\x91\x03\x12a\r\x05Wa\x10\xF0\x82a\x10\xB9V[\x91a\x10\xFD` \x82\x01a\x10\xCDV[\x91a\x11\x16``a\x11\x0F`@\x85\x01a\x10\xB9V[\x93\x01a\x10\xCDV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xD0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90` \x92\x83\x81\x83\x03\x12a\r\x05W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x12FW\x01\x81`\x1F\x82\x01\x12\x15a\x11\xEDW\x80Q\x90a\x11m\x82a\x11\x19V[\x92a\x11{`@Q\x94\x85a\x10DV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x11\x98W\x84\x83\x94\x95a\x11\x16\x94\x01\x91\x01a\x0F\xE9V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\x80\x1B\x03\x82\x16\x82\x03a\r\0WV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\0WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\r\0WV[\x91\x90\x82a\x01\0\x91\x03\x12a\r\x05Wa\x12\xE0\x82a\x12\x96V[\x91a\x12\xED` \x82\x01a\x12\x96V[\x91a\x12\xFA`@\x83\x01a\x12\x96V[\x91a\x13\x07``\x82\x01a\x12\xAAV[\x91a\x13\x14`\x80\x83\x01a\x12\xBBV[\x91a\x13!`\xA0\x82\x01a\x12\xBBV[\x91a\x11\x16`\xE0a\x133`\xC0\x85\x01a\x10\xB9V[\x93\x01a\x10\xB9V[\x90a\x13D\x82a\x11\x19V[a\x13Q`@Q\x91\x82a\x10DV[\x82\x81R\x80\x92a\x13b`\x1F\x19\x91a\x11\x19V[\x01\x90` 6\x91\x017V[\x80Q\x15a\x14\xB4W`@Qa\x13\x7F\x81a\x10(V[`@\x81R\x7FABCDEFGHIJKLMNOPQRSTUVWXYZabcdef` \x82\x01R\x7Fghijklmnopqrstuvwxyz0123456789+/`@\x82\x01R\x81Q`\x02\x92\x83\x82\x01\x80\x92\x11a\x14\x9EW`\x03\x91\x82\x90\x04`\x01`\x01`\xFE\x1B\x03\x81\x16\x81\x03a\x14\x9EWa\x14\x01\x90\x85\x94\x95\x1Ba\x13:V[\x93` \x85\x01\x93\x82\x91\x83Q\x84\x01\x92[\x83\x81\x10a\x14MWPPPPQ\x06\x80`\x01\x14a\x14:W`\x02\x14a\x14/WP\x90V[`=\x90`\0\x19\x01S\x90V[P`=\x90\x81`\0\x19\x82\x01S`\x01\x19\x01S\x90V[\x85`\x04\x91\x97\x92\x93\x94\x97\x01\x91\x82Q`\x01\x90`?\x90\x82\x82\x82`\x12\x1C\x16\x88\x01\x01Q\x84S\x82\x82\x82`\x0C\x1C\x16\x88\x01\x01Q\x83\x85\x01S\x82\x82\x82`\x06\x1C\x16\x88\x01\x01Q\x88\x85\x01S\x16\x85\x01\x01Q\x87\x82\x01S\x01\x95\x92\x91\x90a\x14\x0FV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@R`\0\x81R\x90V[\x80`\0\x91z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x80\x82\x10\x15a\x16\x0EW[Pm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x80\x83\x10\x15a\x15\xFFW[Pf#\x86\xF2o\xC1\0\0\x80\x83\x10\x15a\x15\xF0W[Pc\x05\xF5\xE1\0\x80\x83\x10\x15a\x15\xE1W[Pa'\x10\x80\x83\x10\x15a\x15\xD2W[P`d\x82\x10\x15a\x15\xC2W[`\n\x80\x92\x10\x15a\x15\xB8W[`\x01\x90\x81`!a\x15p\x82\x87\x01a\x13:V[\x95\x86\x01\x01\x90[a\x15\x82W[PPPP\x90V[`\0\x19\x01\x90\x83\x90o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x82\x82\x06\x1A\x83S\x04\x91\x82\x15a\x15\xB3W\x91\x90\x82a\x15vV[a\x15{V[\x91`\x01\x01\x91a\x15_V[\x91\x90`d`\x02\x91\x04\x91\x01\x91a\x15TV[`\x04\x91\x93\x92\x04\x91\x01\x918a\x15IV[`\x08\x91\x93\x92\x04\x91\x01\x918a\x15<V[`\x10\x91\x93\x92\x04\x91\x01\x918a\x15-V[` \x91\x93\x92\x04\x91\x01\x918a\x15\x1BV[`@\x93P\x81\x04\x91P8a\x15\x02V[`@Q\x90`\x01`\x01`\xA0\x1B\x03\x16\x80a\x163\x83a\x10(V[`*\x83R` \x80\x84\x01`@6\x827\x84Q\x15a\x17\nW`0\x90S\x83Q\x90`\x01\x91\x82\x10\x15a\x17\nW\x90`x`!\x86\x01S`)\x91[\x81\x83\x11a\x16\x96WPPPa\x16wWP\x90V[`D\x90`@Q\x90c\xE2.'\xEB`\xE0\x1B\x82R`\x04\x82\x01R`\x14`$\x82\x01R\xFD[\x90\x91\x92`\x0F\x81\x16`\x10\x81\x10\x15a\x16\xF5W\x86Q\x85\x10\x15a\x16\xF5Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1A\x86\x85\x01\x83\x01S`\x04\x1C\x92\x80\x15a\x16\xE0W`\0\x19\x01\x91\x90a\x16eV[`$`\0cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$`\0cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[h\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\r\0Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V\xFE\xA2dipfsX\"\x12 `\xA8\xF4nr\xC7(\x95o88\xC50\xB0\xE6}T(\x10h\xAD\xE1g\x8BK\xDB\xDF2\x13\xCA\x0F\x19dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static POSITIONRENDERER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80\x80`@R`\x046\x10\x15a\0rW[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[`\x005`\xE0\x1Cc\x0E\x894\x1C\x03a\0\x0FW4a\x0FLW` 6`\x03\x19\x01\x12a\r\x05Wb\xFF\xFF\xFF\x80a\0\xA3`\x045a\x17 V[` \x1C\x163;\x15a\r\x1EW`@Q\x90c\x17\x91\xD9\x8F`\xE2\x1B\x82R`\x04\x82\x01R`\x80\x81`$\x813Z\xFA\x90\x81\x15a\r\x12W`\0\x90`\0\x92a\x0F'W[P`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r\x1EW`\0`\x04\x92`@Q\x93\x84\x80\x92c\x95\xD8\x9BA`\xE0\x1B\x82RZ\xFA\x91\x82\x15a\r\x12W`\0\x92a\x0F\nW[P`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r\x1EW`\0`\x04\x92`@Q\x93\x84\x80\x92c\x95\xD8\x9BA`\xE0\x1B\x82RZ\xFA\x91\x82\x15a\r\x12W`\0\x92a\x0E\xE6W[P`8a\x01\xC3\x91`@Q\x93\x84\x91\x7FPrimitive Portfolio LP \0\0\0\0\0\0\0\0\0` \x84\x01Ra\x01\x95\x81Q\x80\x92` `7\x87\x01\x91\x01a\x0F\xE9V[\x82\x01`-`\xF8\x1B`7\x82\x01Ra\x01\xB4\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x0F\xE9V[\x01\x03`\x18\x81\x01\x84R\x01\x82a\x10DV[`@Qa\x01\x80\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x0E\xD0Wa\x03~\x91`@Ra\x01E\x81R\x7F<svg width=\"512\" height=\"512\" fi` \x82\x01R\x7Fll=\"none\" xmlns=\"http://www.w3.o`@\x82\x01R\x7Frg/2000/svg\"><path fill=\"#000\" d``\x82\x01R\x7F=\"M0 0h512v512H0z\"/><path fill-r`\x80\x82\x01R\x7Fule=\"evenodd\" clip-rule=\"evenodd`\xA0\x82\x01R\x7F\" d=\"M339.976 134.664h41.048L256`\xC0\x82\x01R\x7F 340.586 130.976 134.664h41.047V`\xE0\x82\x01R\x7F98H64.143L256 414 447.857 98H339a\x01\0\x82\x01R\x7F.976v36.664Zm-38.759 0V98h-90.43a\x01 \x82\x01R\x7F6v36.664h90.436Z\" fill=\"#fff\"/><a\x01@\x82\x01Rd\x17\xB9\xBB3\x9F`\xD9\x1Ba\x01`\x82\x01Ra\x13lV[\x91a\x03\xD2`:`@Q\x80\x95\x7Fdata:image/svg+xml;base64,\0\0\0\0\0\0` \x83\x01Ra\x03\xC2\x81Q\x80\x92` \x86\x86\x01\x91\x01a\x0F\xE9V[\x81\x01\x03`\x1A\x81\x01\x86R\x01\x84a\x10DV[a\x03\xDD`\x045a\x17 V[` \x1C\x163;\x15a\r\x1EW`@Q\x90c\x17\x91\xD9\x8F`\xE2\x1B\x82R`\x04\x82\x01R`\x80\x81`$\x813Z\xFA\x80\x15a\r\x12W`\0\x91`\0\x91a\x0E\x9CW[P`\x01`\x01`\xA0\x1B\x03\x82\x16;\x15a\r\x1EW`@Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R`\0\x81`\x04\x81`\x01`\x01`\xA0\x1B\x03\x87\x16Z\xFA\x90\x81\x15a\r\x12W`\0\x91a\x0E\x81W[P`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\r\x1EW`@Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x92`\0\x84`\x04\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\r\x12W`\0\x94a\x0E`W[Pa\x04\x9E\x90a\x16\x1CV[\x90`\x01`\x01`\xA0\x1B\x03\x83\x16;\x15a\r\x1EW`@Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R\x91`\0\x83`\x04\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x92\x83\x15a\r\x12W`\0\x93a\x0ECW[P`\x01`\x01`\xA0\x1B\x03\x84\x16;\x15a\r\x1EW`@Qc\x95\xD8\x9BA`\xE0\x1B\x81R\x92`\0\x84`\x04\x81`\x01`\x01`\xA0\x1B\x03\x89\x16Z\xFA\x90\x81\x15a\r\x12Wa\x06\xA1\x95`\x89\x95`\0\x93a\x0E\x1AW[Pa\x05/\x90a\x16\x1CV[\x92`@Q\x97\x88\x95m\x110\xB9\xB9\xB2\xBA/\xB70\xB6\xB2\x91\x1D\x11`\x91\x1B` \x88\x01Ra\x05a\x81Q\x80\x92` `.\x8B\x01\x91\x01a\x0F\xE9V[a\x08\x8B`\xF2\x1B\x87\x82\x01`.\x81\x01\x91\x90\x91Ro\x110\xB9\xB9\xB2\xBA/\xB9\xBC\xB6\xB17\xB6\x11\x1D\x11`\x81\x1B`0\x82\x01R\x82Q\x92a\x05\xA2\x91\x84\x91`@\x90\x91\x01\x90` \x01a\x0F\xE9V[\x86\x01\x01a\x08\x8B`\xF2\x1B`@\x82\x01Rp\x110\xB9\xB9\xB2\xBA/\xB0\xB2292\xB9\xB9\x91\x1D\x11`y\x1B`B\x82\x01Ra\x05\xDE\x82Q\x80\x93` `S\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`S\x82\x01Rm\x118\xBA\xB7\xBA2\xAF\xB70\xB6\xB2\x91\x1D\x11`\x91\x1B`U\x82\x01Ra\x06\x15\x82Q\x80\x93` `c\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`c\x82\x01Ro\x118\xBA\xB7\xBA2\xAF\xB9\xBC\xB6\xB17\xB6\x11\x1D\x11`\x81\x1B`e\x82\x01Ra\x06N\x82Q\x80\x93` `u\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`u\x82\x01Rp\x118\xBA\xB7\xBA2\xAF\xB0\xB2292\xB9\xB9\x91\x1D\x11`y\x1B`w\x82\x01Ra\x06\x88\x82Q\x80\x93` `\x88\x85\x01\x91\x01a\x0F\xE9V[\x01`\x11`\xF9\x1B`\x88\x82\x01R\x03`i\x81\x01\x84R\x01\x82a\x10DV[3;\x15a\r\x1EW`@Qc\"i|!`\xE2\x1B\x81R`\x04\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90a\x01\0\x82`$\x813Z\xFA\x80\x15a\r\x12W`\0\x90`\0\x91`\0\x94`\0\x92a\r\xC9W[P\x91`r\x91a\x07\x1Ba\x07\x15a\x07\x0Fa\xFF\xFFa\x07\x08\x81a\x08\x1F\x99\x16a\x14\xD9V[\x95\x16a\x14\xD9V[\x97a\x16\x1CV[\x91a\x16\x1CV[\x90`@Q\x96\x87\x93s\x1132\xB2\xAF\xB10\xB9\xB4\xB9\xAF\xB87\xB4\xB7:9\x91\x1D\x11`a\x1B` \x86\x01Ra\x07S\x81Q\x80\x92` `4\x89\x01\x91\x01a\x0F\xE9V[\x84\x01a\x08\x8B`\xF2\x1B`4\x82\x01R\x7F\"priority_fee_basis_points\":\"\0\0\0`6\x82\x01Ra\x07\x9A\x82Q\x80\x93` `S\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`S\x82\x01Rm\x111\xB7\xB7:97\xB662\xB9\x11\x1D\x11`\x91\x1B`U\x82\x01Ra\x07\xD1\x82Q\x80\x93` `c\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`c\x82\x01Rk\x119\xBA90\xBA2\xB3\xBC\x91\x1D\x11`\xA1\x1B`e\x82\x01Ra\x08\x06\x82Q\x80\x93` `q\x85\x01\x91\x01a\x0F\xE9V[\x01`\x11`\xF9\x1B`q\x82\x01R\x03`R\x81\x01\x85R\x01\x83a\x10DV[3;\x15a\r\x1EW`@Qc\"i|!`\xE2\x1B\x81R`\x04\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x01\0\x81`$\x813Z\xFA\x90\x81\x15a\r\x12W`\0\x91a\r\x90W[P`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\r#W[`\x01`\x01`\xA0\x1B\x03\x16\x91\x82;\x15a\r\x1EW`\xA0`$\x93`@Q\x94\x85\x80\x92c4\xDB\xC7;`\xE0\x1B\x82Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x045\x16`\x04\x83\x01RZ\xFA\x80\x15a\r\x12W`\0\x93`\0\x91`\0\x95`\0\x92`\0\x92a\x0C\x84W[P\x94a\x0B\xDF\x94a\nz`\x90`\xB3\x96`@\x9B\x96a\x0B\xE4\x9B\x96a\t\x17a\x0CS\x9F\x9Ca\t\x10c\xFF\xFF\xFF\xFFa\t\t\x81a\t\x02`\x01`\x01`\x80\x1B\x03\x82\x96\x16a\x14\xD9V[\x98\x16a\x14\xD9V[\x9B\x16a\x14\xD9V[\x93\x16a\x14\xD9V[\x90\x15a\x0CaW\x8DQa\t(\x81a\x10\x0CV[`\x04\x81Rctrue`\xE0\x1B` \x82\x01R\x91[\x8EQ\x98\x89\x94s\x119\xBA94\xB5\xB2\xAF\xB894\xB1\xB2\xAF\xBB\xB0\xB2\x11\x1D\x11`a\x1B` \x87\x01Ra\tq\x81Q\x80\x92` `4\x8A\x01\x91\x01a\x0F\xE9V[\x85\x01a\x08\x8B`\xF2\x1B`4\x82\x01R\x7F\"volatility_basis_points\":\"\0\0\0\0\0`6\x82\x01Ra\t\xB8\x82Q\x80\x93` `Q\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`Q\x82\x01Rs\x112:\xB90\xBA4\xB7\xB7/\xB9\xB2\xB1\xB7\xB729\x91\x1D\x11`a\x1B`S\x82\x01Ra\t\xF5\x82Q\x80\x93` `g\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`g\x82\x01Ru\x111\xB92\xB0\xBA4\xB7\xB7/\xBA4\xB6\xB2\xB9\xBA0\xB6\xB8\x11\x1D\x11`Q\x1B`i\x82\x01Ra\n4\x82Q\x80\x93` `\x7F\x85\x01\x91\x01a\x0F\xE9V[\x01a\x08\x8B`\xF2\x1B`\x7F\x82\x01Rn\x114\xB9\xAF\xB82\xB982\xBA:\xB0\xB6\x11\x1D`\x89\x1B`\x81\x82\x01Ra\nk\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x0F\xE9V[\x01\x03`p\x81\x01\x86R\x01\x84a\x10DV[\x88Q\x96\x87\x94h=\x9170\xB6\xB2\x91\x1D\x11`\xB9\x1B` \x87\x01Ra\n\xA5\x81Q\x80\x92` `)\x8A\x01\x91\x01a\x0F\xE9V[\x85\x01j\x11\x16\x114\xB6\xB0\xB3\xB2\x91\x1D\x11`\xA9\x1B`)\x82\x01Ra\n\xCF\x82Q\x80\x93` `4\x85\x01\x91\x01a\x0F\xE9V[\x01\x91\x7F\",\"license\":\"MIT\",\"creator\":\"pri`4\x84\x01Rk\x1BZ]\x1A]\x99K\x99]\x1A\x08\x8B`\xA2\x1B`T\x84\x01R\x7F\"description\":\"Concentrated liqu``\x84\x01R\x7Fidity tokens of a two-token AMM\"`\x80\x84\x01R`\x0B`\xFA\x1B\x92\x83`\xA0\x82\x01Rm\"properties\":{`\x90\x1B`\xA1\x82\x01Ra\x0B\x8F\x82Q\x80\x93` `\xAF\x85\x01\x91\x01a\x0F\xE9V[\x01\x82`\xAF\x82\x01Ra\x0B\xAA\x82Q\x80\x93` `\xB0\x85\x01\x91\x01a\x0F\xE9V[\x01\x90`\xB0\x82\x01Ra\x0B\xC5\x82Q\x80\x93` `\xB1\x85\x01\x91\x01a\x0F\xE9V[\x01a}}`\xF0\x1B`\xB1\x82\x01R\x03`\x93\x81\x01\x84R\x01\x82a\x10DV[a\x13lV[\x81Q\x90a\x0C8`=\x83` \x81\x01\x93\x7Fdata:application/json;base64,\0\0\0\x85Ra\x0C(\x81Q\x80\x92` \x86\x86\x01\x91\x01a\x0F\xE9V[\x81\x01\x03`\x1D\x81\x01\x85R\x01\x83a\x10DV[\x82Q\x93\x84\x92` \x84RQ\x80\x92\x81` \x86\x01R\x85\x85\x01\x90a\x0F\xE9V[`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[\x8DQa\x0Cl\x81a\x10\x0CV[`\x05\x81Rdfalse`\xD8\x1B` \x82\x01R\x91a\t;V[\x94\x98\x95\x96PPPPP`\xA0\x81=`\xA0\x11a\r\nW[\x81a\x0C\xA6`\xA0\x93\x83a\x10DV[\x81\x01\x03\x12a\r\x05Wa\x0C\xB7\x81a\x12\x96V[\x92a\x0C\xC4` \x83\x01a\x12\xAAV[\x94a\x0C\xD1`@\x84\x01a\x12\xAAV[\x92`\x80a\x0C\xE0``\x83\x01a\x12\xAAV[\x91\x01Q\x92\x83\x15\x15\x84\x03a\r\0W\x94\x97\x91\x96\x93\x95\x91\x93\x92\x90\x91a\x0B\xDFa\x08\xC4V[`\0\x80\xFD[a\x0F\x99V[=\x91Pa\x0C\x99V[`@Q=`\0\x82>=\x90\xFD[a\x10fV[P3;\x15a\r\x1EW`@QcS\x1E\x17\xB3`\xE0\x1B\x81R` \x81`\x04\x813Z\xFA\x90\x81\x15a\r\x12W`\0\x91a\rVW[Pa\x08oV[\x90P` \x81=` \x11a\r\x88W[\x81a\rq` \x93\x83a\x10DV[\x81\x01\x03\x12a\r\x05Wa\r\x82\x90a\x10\xB9V[\x85a\rPV[=\x91Pa\rdV[a\r\xB4\x91Pa\x01\0=a\x01\0\x11a\r\xC2W[a\r\xAC\x81\x83a\x10DV[\x81\x01\x90a\x12\xCAV[\x96PPPPPPP\x85a\x08^V[P=a\r\xA2V[a\xFF\xFF\x95Pa\x08\x1F\x93P`r\x92Pa\x07\x08\x91Pa\x07\x1Ba\x07\x15a\x07\x0Fa\x0E\0\x89\x94a\x01\0=a\x01\0\x11a\r\xC2Wa\r\xAC\x81\x83a\x10DV[\x90\x9FP\x90\x9C\x90\x9BP\x90\x99P\x97Pa\x06\xE9\x96PPPPPPPV[a\x05/\x91\x93Pa\x0E<\x90=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x81\x01\x90a\x115V[\x92\x90a\x05%V[a\x0EY\x91\x93P=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x91\x87a\x04\xDEV[a\x04\x9E\x91\x94Pa\x0Ez\x90=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x93\x90a\x04\x94V[a\x0E\x96\x91P=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x85a\x04TV[\x90Pa\x0E\xC0\x91P`\x80=`\x80\x11a\x0E\xC9W[a\x0E\xB8\x81\x83a\x10DV[\x81\x01\x90a\x10\xDBV[P\x90P\x84a\x04\x15V[P=a\x0E\xAEV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[a\x01\xC3\x91\x92Pa\x0F\x02`8\x91=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x92\x91Pa\x01NV[a\x0F \x91\x92P=\x80`\0\x83>a\x0E4\x81\x83a\x10DV[\x90\x83a\x01\x15V[\x90Pa\x0FB\x91P`\x80=`\x80\x11a\x0E\xC9Wa\x0E\xB8\x81\x83a\x10DV[P\x91\x90P\x83a\0\xDCV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\0[\x83\x81\x10a\x0F\xFCWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x0F\xECV[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\r\0WV[Q\x90`\xFF\x82\x16\x82\x03a\r\0WV[\x91\x90\x82`\x80\x91\x03\x12a\r\x05Wa\x10\xF0\x82a\x10\xB9V[\x91a\x10\xFD` \x82\x01a\x10\xCDV[\x91a\x11\x16``a\x11\x0F`@\x85\x01a\x10\xB9V[\x93\x01a\x10\xCDV[\x90V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E\xD0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90` \x92\x83\x81\x83\x03\x12a\r\x05W\x80Q\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x12FW\x01\x81`\x1F\x82\x01\x12\x15a\x11\xEDW\x80Q\x90a\x11m\x82a\x11\x19V[\x92a\x11{`@Q\x94\x85a\x10DV[\x82\x84R\x85\x83\x83\x01\x01\x11a\x11\x98W\x84\x83\x94\x95a\x11\x16\x94\x01\x91\x01a\x0F\xE9V[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x85\x90R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x86\x90R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90`\x01`\x01`\x80\x1B\x03\x82\x16\x82\x03a\r\0WV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\r\0WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\r\0WV[\x91\x90\x82a\x01\0\x91\x03\x12a\r\x05Wa\x12\xE0\x82a\x12\x96V[\x91a\x12\xED` \x82\x01a\x12\x96V[\x91a\x12\xFA`@\x83\x01a\x12\x96V[\x91a\x13\x07``\x82\x01a\x12\xAAV[\x91a\x13\x14`\x80\x83\x01a\x12\xBBV[\x91a\x13!`\xA0\x82\x01a\x12\xBBV[\x91a\x11\x16`\xE0a\x133`\xC0\x85\x01a\x10\xB9V[\x93\x01a\x10\xB9V[\x90a\x13D\x82a\x11\x19V[a\x13Q`@Q\x91\x82a\x10DV[\x82\x81R\x80\x92a\x13b`\x1F\x19\x91a\x11\x19V[\x01\x90` 6\x91\x017V[\x80Q\x15a\x14\xB4W`@Qa\x13\x7F\x81a\x10(V[`@\x81R\x7FABCDEFGHIJKLMNOPQRSTUVWXYZabcdef` \x82\x01R\x7Fghijklmnopqrstuvwxyz0123456789+/`@\x82\x01R\x81Q`\x02\x92\x83\x82\x01\x80\x92\x11a\x14\x9EW`\x03\x91\x82\x90\x04`\x01`\x01`\xFE\x1B\x03\x81\x16\x81\x03a\x14\x9EWa\x14\x01\x90\x85\x94\x95\x1Ba\x13:V[\x93` \x85\x01\x93\x82\x91\x83Q\x84\x01\x92[\x83\x81\x10a\x14MWPPPPQ\x06\x80`\x01\x14a\x14:W`\x02\x14a\x14/WP\x90V[`=\x90`\0\x19\x01S\x90V[P`=\x90\x81`\0\x19\x82\x01S`\x01\x19\x01S\x90V[\x85`\x04\x91\x97\x92\x93\x94\x97\x01\x91\x82Q`\x01\x90`?\x90\x82\x82\x82`\x12\x1C\x16\x88\x01\x01Q\x84S\x82\x82\x82`\x0C\x1C\x16\x88\x01\x01Q\x83\x85\x01S\x82\x82\x82`\x06\x1C\x16\x88\x01\x01Q\x88\x85\x01S\x16\x85\x01\x01Q\x87\x82\x01S\x01\x95\x92\x91\x90a\x14\x0FV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[P`@Q` \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E\xD0W`@R`\0\x81R\x90V[\x80`\0\x91z\x18O\x03\xE9?\xF9\xF4\xDA\xA7\x97\xEDn8\xEDd\xBFj\x1F\x01\0\0\0\0\0\0\0\0\x80\x82\x10\x15a\x16\x0EW[Pm\x04\xEE-mA[\x85\xAC\xEF\x81\0\0\0\0\x80\x83\x10\x15a\x15\xFFW[Pf#\x86\xF2o\xC1\0\0\x80\x83\x10\x15a\x15\xF0W[Pc\x05\xF5\xE1\0\x80\x83\x10\x15a\x15\xE1W[Pa'\x10\x80\x83\x10\x15a\x15\xD2W[P`d\x82\x10\x15a\x15\xC2W[`\n\x80\x92\x10\x15a\x15\xB8W[`\x01\x90\x81`!a\x15p\x82\x87\x01a\x13:V[\x95\x86\x01\x01\x90[a\x15\x82W[PPPP\x90V[`\0\x19\x01\x90\x83\x90o\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x82\x82\x06\x1A\x83S\x04\x91\x82\x15a\x15\xB3W\x91\x90\x82a\x15vV[a\x15{V[\x91`\x01\x01\x91a\x15_V[\x91\x90`d`\x02\x91\x04\x91\x01\x91a\x15TV[`\x04\x91\x93\x92\x04\x91\x01\x918a\x15IV[`\x08\x91\x93\x92\x04\x91\x01\x918a\x15<V[`\x10\x91\x93\x92\x04\x91\x01\x918a\x15-V[` \x91\x93\x92\x04\x91\x01\x918a\x15\x1BV[`@\x93P\x81\x04\x91P8a\x15\x02V[`@Q\x90`\x01`\x01`\xA0\x1B\x03\x16\x80a\x163\x83a\x10(V[`*\x83R` \x80\x84\x01`@6\x827\x84Q\x15a\x17\nW`0\x90S\x83Q\x90`\x01\x91\x82\x10\x15a\x17\nW\x90`x`!\x86\x01S`)\x91[\x81\x83\x11a\x16\x96WPPPa\x16wWP\x90V[`D\x90`@Q\x90c\xE2.'\xEB`\xE0\x1B\x82R`\x04\x82\x01R`\x14`$\x82\x01R\xFD[\x90\x91\x92`\x0F\x81\x16`\x10\x81\x10\x15a\x16\xF5W\x86Q\x85\x10\x15a\x16\xF5Wo\x18\x18\x99\x19\x9A\x1A\x9B\x1B\x9C\x1C\xB0\xB11\xB22\xB3`\x81\x1B\x90\x1A\x86\x85\x01\x83\x01S`\x04\x1C\x92\x80\x15a\x16\xE0W`\0\x19\x01\x91\x90a\x16eV[`$`\0cNH{q`\xE0\x1B\x81R`\x11`\x04R\xFD[`$`\0cNH{q`\xE0\x1B\x81R`2`\x04R\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[h\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\r\0Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V\xFE\xA2dipfsX\"\x12 `\xA8\xF4nr\xC7(\x95o88\xC50\xB0\xE6}T(\x10h\xAD\xE1g\x8BK\xDB\xDF2\x13\xCA\x0F\x19dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static POSITIONRENDERER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct PositionRenderer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for PositionRenderer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for PositionRenderer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for PositionRenderer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for PositionRenderer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(PositionRenderer))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> PositionRenderer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    POSITIONRENDERER_ABI.clone(),
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
                POSITIONRENDERER_ABI.clone(),
                POSITIONRENDERER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `uri` (0x0e89341c) function
        pub fn uri(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([14, 137, 52, 28], id)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for PositionRenderer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `StringsInsufficientHexLength` with signature `StringsInsufficientHexLength(uint256,uint256)` and selector `0xe22e27eb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "StringsInsufficientHexLength",
        abi = "StringsInsufficientHexLength(uint256,uint256)"
    )]
    pub struct StringsInsufficientHexLength {
        pub value: ::ethers::core::types::U256,
        pub length: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `uri` function with signature `uri(uint256)` and selector `0x0e89341c`
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
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `uri` function with signature `uri(uint256)` and selector `0x0e89341c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UriReturn(pub ::std::string::String);
}
