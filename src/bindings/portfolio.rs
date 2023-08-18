pub use portfolio::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod portfolio {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("weth"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("registry"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("positionRenderer"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DEFAULT_STRATEGY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DEFAULT_STRATEGY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("POSITION_RENDERER"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("POSITION_RENDERER"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("REGISTRY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("REGISTRY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VERSION"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("VERSION"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WETH"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("WETH"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("useMax"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maxDeltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maxDeltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOfBatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owners"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("balances"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("changeParameters"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("changeParameters"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priorityFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("claimFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createPair"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createPair"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("asset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("quote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pairId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint24"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("createPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("createPool"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pairId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint24"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveXPerWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveYPerWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeBasisPoints"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priorityFeeBasisPoints",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("controller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategyArgs"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("useMax"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minDeltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minDeltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAmountOut"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("output"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInvariant"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInvariant"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getLiquidityDeltas"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getLiquidityDeltas"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(128usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int128"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMaxLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount0"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount1"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint128"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Order"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNetBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNetBalance"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPairId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPairId"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint24"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPairNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPairNonce"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint24"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolNonce"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint24"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPoolReserves"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolReserves"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReserve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReserve"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("token"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStrategy"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStrategy"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("multicall"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("multicall"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("data"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("results"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pairs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pairs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint24"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("decimalsAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("decimalsQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pools"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("virtualX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("virtualY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("liquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lastTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("feeBasisPoints"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priorityFeeBasisPoints",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("controller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("protocolFee"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("protocolFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("protocolFees"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeBatchTransferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("safeBatchTransferFrom",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setProtocolFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setProtocolFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("fee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("order"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct Order"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("success"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("prevInvariant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("postInvariant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swap"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("args"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(128usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                                ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                ::ethers::core::abi::ethabi::ParamType::Bool,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Order"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("input"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("output"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("uri"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("uri"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Allocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Allocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("asset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ChangeParameters"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ChangeParameters"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("priorityFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("fee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ClaimFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ClaimFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CreatePair"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CreatePair"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("pairId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("asset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("decimalsAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("decimalsQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CreatePool"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("CreatePool"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("asset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reserveXPerWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reserveYPerWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeBasisPoints"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("priorityFeeBasisPoints",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("controller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deallocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("asset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("quote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaQuote"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DecreaseReserveBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("DecreaseReserveBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncreaseReserveBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("IncreaseReserveBalance",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Swap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("sellAsset"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("input"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("output"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("feeAmountDec"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("invariantWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferBatch"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransferBatch"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("ids"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferSingle"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("TransferSingle"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("URI"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("URI"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UpdateProtocolFee"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("UpdateProtocolFee"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("prevFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nextFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EtherTransfer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EtherTransfer"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientReserve"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientReserve",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("delta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidBalance"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_AlreadyCreated"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolLib_AlreadyCreated",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidFee"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolLib_InvalidFee"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidPriorityFee"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolLib_InvalidPriorityFee",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidReserveX"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolLib_InvalidReserveX",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_InvalidReserveY"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolLib_InvalidReserveY",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PoolLib_UpperLiquidityLimit"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PoolLib_UpperLiquidityLimit",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_BeforeSwapFail"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_BeforeSwapFail",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_DuplicateToken"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_DuplicateToken",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_Insolvent"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_Insolvent",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("net"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InsufficientLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InsufficientLiquidity",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidDecimals"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidDecimals",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("decimals"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidInvariant"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidInvariant",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("prev"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("next"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidMulticall"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidMulticall",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidPairNonce"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidPairNonce",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidPool"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidPool",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidProtocolFee"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidProtocolFee",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("protocolFee"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidReentrancy"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidReentrancy",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_InvalidSettlement"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_InvalidSettlement",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MaxAssetExceeded"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_MaxAssetExceeded",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MaxQuoteExceeded"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_MaxQuoteExceeded",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MinAssetExceeded"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_MinAssetExceeded",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_MinQuoteExceeded"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_MinQuoteExceeded",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_NonExistentPool"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_NonExistentPool",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_NotController"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_NotController",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_PairExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_PairExists",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("pairId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(24usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint24"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroAmountsAllocate"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_ZeroAmountsAllocate",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroLiquidityAllocate"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_ZeroLiquidityAllocate",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroLiquidityDeallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_ZeroLiquidityDeallocate",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapInput"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapInput",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapLiquidity",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapOutput"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Portfolio_ZeroSwapOutput",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_OutputExceedsReserves"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SwapLib_OutputExceedsReserves",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ProtocolFeeTooHigh"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SwapLib_ProtocolFeeTooHigh",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroXAdjustment"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SwapLib_ZeroXAdjustment",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroYAdjustment"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SwapLib_ZeroYAdjustment",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenTransfer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TokenTransfer"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenTransferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TokenTransferFrom"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PORTFOLIO_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"a\x01\x004b\0\x01TW`\x01`\x01`@\x1B\x03`\x1Fb\0\x8C#8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x84\x01\x91\x83\x83\x11\x85\x84\x10\x17b\0\x01>W\x80\x85\x92``\x94`@R\x839\x81\x01\x03\x12b\0\x01TWb\0\0O\x82b\0\x01YV[\x91b\0\0l`@b\0\0d` \x84\x01b\0\x01YV[\x92\x01b\0\x01YV[\x92`\x01`\x0CU`\x80R`\xA0R`@Q\x90a5;\x80\x83\x01\x91\x83\x83\x10\x90\x83\x11\x17b\0\x01>W` \x91\x83\x91b\0V\xE8\x8390\x81R\x03\x01\x90`\0\xF0\x80\x15b\0\x012W`\x01`\x01`\xA0\x1B\x03\x16`\xC0R`\xE0R`\x05\x80T`\xFF\x19\x16`\x01\x17\x90U`@QaUy\x90\x81b\0\x01o\x829`\x80Q\x81\x81\x81a\x1F\xD8\x01R\x81\x81a0\xFB\x01R\x81\x81a;\xC8\x01RaA\xE6\x01R`\xA0Q\x81\x81\x81a\x03\x9E\x01R\x81\x81a\x16\xDC\x01Ra$\xD8\x01R`\xC0Q\x81\x81\x81a\x12\xAB\x01R\x81\x81a9\x9E\x01Ra9\xCE\x01R`\xE0Q\x81\x81\x81a\x04|\x01Ra \x1D\x01R\xF3[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03b\0\x01TWV\xFE`\x80`@R`\x046\x10\x15a\0#W[6\x15a\0\x19W`\0\x80\xFD[a\0!a0\xF9V[\0[`\x005`\xE0\x1C\x80b\xFD\xD5\x8E\x14a\x02\xA2W\x80c\x01\xFF\xC9\xA7\x14a\x02\x9DW\x80c\x06C;\x1B\x14a\x02\x98W\x80c\x07\x88\x88\xD6\x14a\x02\x93W\x80c\x0E\x894\x1C\x14a\x02\x8EW\x80c\x19\x05x\x07\x14a\x02\x89W\x80c&z\x0C\xFE\x14a\x02\x84W\x80c*\xFB\x9D\xF8\x14a\x02\x7FW\x80c.\xB2\xC2\xD6\x14a\x02zW\x80c/\x9E8\xE2\x14a\x02uW\x80c0$K\xE7\x14a\x02pW\x80c9CMZ\x14a\x02kW\x80c?\x92\xA39\x14a\x02fW\x80cM\xC6\x8A\x90\x14a\x02aW\x80cN\x12s\xF4\x14a\x02\\W\x80cS\x1E\x17\xB3\x14a\x02WW\x80c[\xC5Td\x14a\x02RW\x80c^Gf<\x14a\x02MW\x80cx}\xCE=\x14a\x02HW\x80c\x80\xAF\x9Dv\x14a\x02CW\x80c\x89\x92\xF2\n\x14a\x02>W\x80c\x89\xA5\xF0\x84\x14a\x029W\x80c\x8Ag\x89g\x14a\x024W\x80c\xA2,\xB4e\x14a\x02/W\x80c\xA5\xCD\x8AI\x14a\x02*W\x80c\xAC\x96P\xD8\x14a\x02%W\x80c\xAD\\FH\x14a\x02 W\x80c\xB0\xC3\xA9P\x14a\x02\x1BW\x80c\xB0\xE2\x1E\x8A\x14a\x02\x16W\x80c\xC9\xA3\x96\xE9\x14a\x02\x11W\x80c\xC9\xC6S\x96\x14a\x02\x0CW\x80c\xD6\xB7\xDE\xC5\x14a\x02\x07W\x80c\xDC\xF8D\xA7\x14a\x02\x02W\x80c\xDD\xA4\x07\x97\x14a\x01\xFDW\x80c\xE31\xBA4\x14a\x01\xF8W\x80c\xE9\x85\xE9\xC5\x14a\x01\xF3W\x80c\xF0{\x87\x9E\x14a\x01\xEEW\x80c\xF2BC*\x14a\x01\xE9W\x80c\xF3:\xE1\xBC\x14a\x01\xE4Wc\xFF\xA1\xADt\x03a\0\x0EWa0\xCDV[a*\x05V[a(\x84V[a'bV[a&\xC3V[a&RV[a$\x8FV[a$RV[a#vV[a \xA7V[a jV[a LV[a \x07V[a\x1F\xC2V[a\x1E\xB0V[a\x1E\x12V[a\x1D}V[a\x1B\xDCV[a\x1B\x02V[a\x19\xF1V[a\x18\xFDV[a\x16\xA2V[a\x16?V[a\x12\xDAV[a\x12\x95V[a\x11\x9DV[a\x111V[a\x10\xE7V[a\x10QV[a\x0F\xF6V[a\x0B\x17V[a\x08\x95V[a\x07lV[a\x06\xD1V[a\x05oV[a\x04LV[a\x03\xCDV[a\x03\x88V[a\x03\x1AV[a\x02\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\xB8WV[`\0\x80\xFD[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x02\xDA\x81a\x02\xA7V[`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 `$5`\0R` R` `@`\0 T`@Q\x90\x81R\xF3[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x03a\x02\xB8WV[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W` `\x045a\x039\x81a\x03\x08V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x81\x14\x90\x81\x15a\x03wW[\x81\x15a\x03fW[P`@Q\x90\x15\x15\x81R\xF3[c\x03\xA2M\x07`\xE2\x1B\x14\x90P8a\x03[V[cl\xDB=\x13`\xE1\x1B\x81\x14\x91Pa\x03TV[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W` b\xFF\xFF\xFF`\x06T\x16`@Q\x90\x81R\xF3[`\0[\x83\x81\x10a\x04\x03WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xF3V[\x90` \x91a\x04,\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xF0V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x04I\x92\x81\x81R\x01\x90a\x04\x13V[\x90V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`@Qc\x03\xA2M\x07`\xE2\x1B\x81R`\x04\x805\x90\x82\x01R`\0\x90\x81\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05OW\x82\x91a\x04\xC6W[`@Q\x80a\x04\xC2\x84\x82a\x048V[\x03\x90\xF3[\x90P=\x80\x83\x83>a\x04\xD7\x81\x83a\x18RV[\x81\x01\x90` \x81\x83\x03\x12a\x05GW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x05KW\x01\x81`\x1F\x82\x01\x12\x15a\x05GW\x80Q\x92a\x05\r\x84a1+V[\x92a\x05\x1B`@Q\x94\x85a\x18RV[\x84\x84R` \x85\x84\x01\x01\x11a\x05DWPa\x04\xC2\x92a\x05>\x91` \x80\x85\x01\x91\x01a\x03\xF0V[8a\x04\xB4V[\x80\xFD[\x82\x80\xFD[\x83\x80\xFD[a1FV[`\x01`\x01`@\x1B\x03\x81\x16\x03a\x02\xB8WV[\x80\x15\x15\x03a\x02\xB8WV[4a\x02\xB8W`\x806`\x03\x19\x01\x12a\x02\xB8Wa\x06\x1B` `\x045a\x05\x91\x81a\x05TV[`$5a\x05\x9D\x81a\x05eV[`d5\x91a\x05\xAA\x83a\x02\xA7V[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n\x85R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16`@Qc\x19\x05x\x07`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x92\x16`\x04\x83\x01R\x91\x15\x15`$\x82\x01R`D\x805\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`d\x83\x01R\x90\x92\x83\x91\x90\x82\x90\x81\x90`\x84\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x05OWa\x04\xC2\x91`\0\x91a\x06BW[P`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x06c\x91P` =\x81\x11a\x06iW[a\x06[\x81\x83a\x18RV[\x81\x01\x90aC\xACV[8a\x061V[P=a\x06QV[`\x045\x90b\xFF\xFF\xFF\x82\x16\x82\x03a\x02\xB8WV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x02\xB8WV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x02\xB8WV[\x91\x81`\x1F\x84\x01\x12\x15a\x02\xB8W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB8W` \x83\x81\x86\x01\x95\x01\x01\x11a\x02\xB8WV[a\x01\x006`\x03\x19\x01\x12a\x02\xB8Wa\x06\xE6a\x06pV[a\xFF\xFF\x90`d5\x82\x81\x16\x81\x03a\x02\xB8W`\x845\x92\x83\x16\x83\x03a\x02\xB8W`\xA45\x91a\x07\x0F\x83a\x02\xA7V[`\xC45a\x07\x1B\x81a\x02\xA7V[`\xE45\x93`\x01`\x01`@\x1B\x03\x85\x11a\x02\xB8Wa\x04\xC2\x95a\x07Ba\x07R\x966\x90`\x04\x01a\x06\xA4V[\x95\x90\x94`D5\x90`$5\x90a9\x14V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x07\x89\x81a\x05TV[`\x01`\x01`@\x1B\x03\x81\x16`\0R`\n` Ra\x07\xA8`@`\0 a3\x9DV[`@\x81\x01\x90`\x01`\x01`\x80\x1B\x03`\x01`\x01`\x7F\x1B\x03\x81\x84Q\x16\x11a\x08SWa\x081a\x08+``a\x088a\x08\x18a\x08\x13a\x07\xF7b\xFF\xFF\xFF\x98a\x07\xF1\x89a\x08A\x9CQ\x16`\x0F\x0Ba4YV[\x90aM\x8BV[\x98\x90\x9A` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[a3YV[\x97\x85a\x081a\x08+` \x8C\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x91\x16aH#V[\x96\x01Q`\xFF\x16\x90V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`@Qc\xAC\xC9@{`\xE0\x1B\x81R`\x04\x90\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x02\xB8W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB8W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x02\xB8WV[4a\x02\xB8W`\xA06`\x03\x19\x01\x12a\x02\xB8W`\x045a\x08\xB2\x81a\x02\xA7V[`$5\x90a\x08\xBF\x82a\x02\xA7V[`\x01`\x01`@\x1B\x03\x90`D5\x82\x81\x11a\x02\xB8Wa\x08\xE0\x906\x90`\x04\x01a\x08eV[\x92\x90\x93`d5\x82\x81\x11a\x02\xB8Wa\x08\xFB\x906\x90`\x04\x01a\x08eV[\x94\x90\x92`\x845\x90\x81\x11a\x02\xB8Wa\t\x16\x906\x90`\x04\x01a\x06\xA4V[\x90a\t\"\x87\x84\x14aRkV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x94\x903\x86\x14\x80\x15a\n\xBAW[a\tB\x90aQgV[\x84`\0[\x89\x89\x8D\x8D\x85\x85\x10a\n.WPPPPPP\x81\x16\x80\x95\x8A\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFBa\t\x91\x8C`@Q\x91\x82\x91\x8D\x8C3\x97\x85aR\xDDV[\x03\x90\xA4;\x15a\n\x1BW\x96a\t\xC6`\0\x92` \x97\x98\x99`@Q\x9A\x8B\x98\x89\x97\x88\x96c\xBC\x19|\x81`\xE0\x1B\x9D\x8E\x89R3`\x04\x8A\x01aS\x04V[\x03\x92Z\xF1\x80\x15a\x05OWa\0!\x92`\0\x91a\t\xEDW[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x14aR,V[a\n\x0E\x91P` =\x81\x11a\n\x14W[a\n\x06\x81\x83a\x18RV[\x81\x01\x90aQ\xE4V[8a\t\xDCV[P=a\t\xFCV[PPP\x92PPPa\0!\x91P\x15\x15aQ\xA4V[a\n~a\n\xB0\x93a\noa\nU\x88a\nL\x81`\x01\x9Ca\n\xA8\x99aR\xA9V[5\x95\x86\x94aR\xA9V[5\x96`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\0R` R`@`\0 \x90V[a\n\x89\x85\x82Ta8\x03V[\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R` \x81\x90R`@\x90 a\noV[\x91\x82Ta8\x10V[\x90U\x01\x85\x90a\tFV[Pa\tBa\n\xFFa\n\xF83a\n\xE1\x8C`\x01\x80`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T`\xFF\x16\x90V[\x90Pa\t9V[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x02\xB8WV[`\xC06`\x03\x19\x01\x12a\x02\xB8W`\x04\x805\x90a\x0B1\x82a\x05eV[`$5a\x0B=\x81a\x02\xA7V[`D5\x90a\x0BJ\x82a\x05TV[`d5\x90a\x0BW\x82a\x0B\x06V[`\x845\x91a\x0Bd\x83a\x0B\x06V[`\xA45\x93a\x0Bq\x85a\x0B\x06V[\x90\x81a\x0B{a2\xB9V[`\x0FT`\xFF\x16\x15a\x0F\xE9W[`\x01`\x01`@\x1B\x03\x80\x91\x16\x15a\x0F\xD1W[`\x01\x80`\xA0\x1B\x03\x97\x88a\x0B\xD1`\x03a\x0B\xC3\x87`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x16\x15a\x0F\xABW\x83a\x0C\x0Ca\x0C\0a\x0C\0`\x03a\x0B\xC3\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x96`@\x97\x8A\x89Q\x80\x92c\xE6\x04{\x19`\xE0\x1B\x82R\x81\x80a\x0C@` \x98\x89\x96\x83\x01\x91\x90\x91`\x01`\x01`@\x1B\x03` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\x05OW`\0\x91a\x0F~W[P\x15a\x0FUWa\x0Cq\x90`\x01`\x01`\x80\x1B\x03\x80\x80\x9B\x16\x91\x16\x87a=yV[a\x0C\x96a\x08\x13b\xFF\xFF\xFF\x89\x86\x95\x96\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93a\x0E\xC4W[\x89\x86\x16\x15a\x0E\xB4W\x89\x90\x81\x80a\x0C\xD9a\x0C\xD0a\x0C\xCB\x8C`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[a3\x9DV[a\x07\xF1\x8BaG\xD7V[\x9D\x90\x9D\x16\x9C\x16\x94\x16\x84\x11a\x0E\xA4W\x16\x89\x11a\x0E\x94W\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x89\x01\x80Q\x90\x98\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90a\r\x15\x88aG\xD7V[\x92a\r\x1Ea\x18sV[\x93B\x85R\x86\x86\x86\x01R\x8D\x8D\x86\x01R``\x85\x01\x90a\r=\x91\x90`\x0F\x0B\x90RV[`\x01`\x01`@\x1B\x03\x8A\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\r\x7F\x90a4\xDDV[\x82\x01Q`\xFF\x16`\xFF\x16a\r\x91\x91aH#V[\x96``\x82\x01Qa\r\xA1\x90`\xFF\x16\x90V[`\xFF\x16a\r\xAD\x91aH#V[\x97\x87\x15\x80a\x0E\x8CW[a\x0E~WP\x97\x7F\xFD\xFF\xEC\xA7Q\xF0\xDC\xAA\xB7U1\xCB\x81<\x12\xBB\xFDV\xEE>\x96L\xC4q\xD7\xEFC\x93$\x02\xEE\x18\x91a\x0E6a\x0E\x07a\r\xF9a\x04\xC2\x9A\x9B\x9CQ`\x01\x80`\xA0\x1B\x03\x16\x90V[\x97Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x88Q\x8B\x81R` \x81\x01\x8D\x90R`\x01`\x01`\x80\x1B\x03\x90\x96\x16`@\x87\x01R\x83\x16\x96\x90\x92\x16\x94\x16\x92\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA4a\x0EMa\x0EH`\x0FT`\xFF\x16\x90V[\x15\x15\x90V[\x15a\x0EqW[a\x0E[a3\x02V[Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[a\x0EyaACV[a\x0ESV[\x86Qce\x8B\x16\xED`\xE0\x1B\x81R\xFD[P\x88\x15a\r\xB6V[\x87Qc!0\x16\x97`\xE2\x1B\x81R\x8A\x90\xFD[\x89QcVr\x0E\x1D`\xE1\x1B\x81R\x8C\x90\xFD[\x88Qc\x90`\x9A}`\xE0\x1B\x81R\x8B\x90\xFD[\x83Q\x90\x95Pa\x0F>\x90a\x0F\x12\x90a\x0E\xE3\x90`\x01`\x01`\xA0\x1B\x03\x16a1RV[\x85\x8B\x01Qa\x0E\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16a1RV[\x90`\0\x81\x12a\x0FMW[`\0\x82\x12a\x0FDW[\x89a=yV[\x8B\x80a\x0F4a\x0C\xCB\x8C`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[\x92\x16\x92\x16\x90aLdV[\x94a\x0C\x9CV[`\0\x91Pa\x0F\x0CV[P`\0a\x0F\x03V[\x87Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16\x81\x8C\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[\x03\x90\xFD[a\x0F\x9E\x91P\x83=\x85\x11a\x0F\xA4W[a\x0F\x96\x81\x83a\x18RV[\x81\x01\x90a3DV[8a\x0CSV[P=a\x0F\x8CV[`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16\x81\x8A\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[`\x0FT\x90\x92P`\x08\x1C`\x01`\x01`@\x1B\x03\x16\x91a\x0B\x98V[a\x0F\xF1a;\xBFV[a\x0B\x87V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W` a\x10?`\x045a\x10\x18\x81a\x05TV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wa\x06\x1B` `\x045a\x10s\x81a\x05TV[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n\x83R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16`@Q\x80\x80\x95\x81\x94c\x1C\xA1\xA6\xAD`\xE1\x1B\x83R`\x04\x83\x01\x91\x90\x91`\x01`\x01`@\x1B\x03` \x82\x01\x93\x16\x90RV[`@\x90`\x03\x19\x01\x12a\x02\xB8W`\x045a\x10\xDA\x81a\x02\xA7V[\x90`$5a\x04I\x81a\x02\xA7V[4a\x02\xB8W` b\xFF\xFF\xFFa\x11'a\x10\xFE6a\x10\xC2V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x0B\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x81R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W` a\x11Z`\x045a\x11S\x81a\x02\xA7V[0\x90aGYV[`@Q\x90\x81R\xF3[` \x90\x81`@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91`\0[\x82\x81\x10a\x11\x89WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x11{V[4a\x02\xB8W`@\x80`\x03\x196\x01\x12a\x02\xB8W`\x01`\x01`@\x1B\x03\x90`\x045\x82\x81\x11a\x02\xB8Wa\x11\xD0\x906\x90`\x04\x01a\x08eV[\x91\x90\x92`$5\x90\x81\x11a\x02\xB8Wa\x11\xEB\x906\x90`\x04\x01a\x08eV[\x93\x90a\x11\xF8\x85\x85\x14aRkV[a\x12\x01\x84a1]V[\x93a\x12\x0E\x84Q\x95\x86a\x18RV[\x80\x85R`\x1F\x19a\x12\x1D\x82a1]V[\x01` \x906\x82\x88\x017`\0\x91\x82[\x81\x81\x10a\x12?W\x86Q\x80a\x04\xC2\x8A\x82a\x11bV[\x80a\x12M`\x01\x92\x84\x89aR\xA9V[5a\x12W\x81a\x02\xA7V[\x82\x80`\xA0\x1B\x03\x16\x85R\x84\x84Ra\x12\x83\x88\x86 a\x12t\x83\x8D\x8AaR\xA9V[5`\0R` R`@`\0 \x90V[Ta\x12\x8E\x82\x8Ba2\xA5V[R\x01a\x12+V[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\xA06`\x03\x19\x01\x12a\x02\xB8W`\x04\x805a\x12\xF3\x81a\x05eV[`$5\x91a\x13\0\x83a\x05TV[`D5\x92a\x13\r\x84a\x0B\x06V[`d5\x90a\x13\x1A\x82a\x0B\x06V[`\x845\x94a\x13'\x86a\x0B\x06V[\x90a\x130a2\xB9V[`\x0FT`\xFF\x16\x15a\x162W[a\x13da\x0C\0a\x0C\0`\x03a\x0B\xC3\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`@\x80Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16\x87\x82\x01\x90\x81R\x91\x95` \x95\x93\x90\x92\x86\x91\x83\x91\x82\x90\x81\x90\x85\x01\x03\x91Z\xFA\x90\x81\x15a\x05OW`\0\x91a\x16\x15W[P\x15a\x15\xF0Wa\x13\xC5\x90`\x01`\x01`\x80\x1B\x03\x80\x80\x9A\x16\x91\x16\x84a=yV[\x93a\x13\xE9a\x08\x13b\xFF\xFF\xFF\x86\x84\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x80Q\x90\x97\x90`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x01Q\x90\x96\x90`\x01`\x01`\xA0\x1B\x03\x16\x99a\x15\xA7W[\x8A\x85\x16\x15a\x15\x98W\x8A\x90\x81\x80a\x14Ma\x14<a\x0C\xCB\x8B`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[a\x07\xF1a\x14H\x8BaG\xD7V[a4YV[\x9E\x90\x9E\x16\x9D\x16\x95\x16\x85\x10a\x15\x88W\x16\x8A\x10a\x15zWPa\x15C\x89a\x154a\x08+``a\x15:\x87\x8E\x9F\x8E\x9Fa\x04\xC2\x9F\x99\x8F\x9A\x8F\x8F\x8F\x7F0\x84\xCA\xF4\x89f\\\xAB\x07E,\xFEO=\x0E\xB5\xE0\xDC\x15\xEA\xC6\xFCe\x80\x98\x85\x8Ec\x9Ep\xE5:\x9F\x95`\x01`\x01`@\x1B\x03\x9Fa\x15,\x94a\x15\x01\x88\x95a\x14\xF1a\x08+\x9Ba\x154\x9Da\x15\x17\x96a\x14\xD4a\x14Ha\x15'\x9BaG\xD7V[\x92a\x14\xDDa\x18sV[B\x81R\x9B\x8C\x01R\x8A\x01R`\x0F\x0B``\x89\x01RV[`\x01`\x01`@\x1B\x03\x16`\x80\x87\x01RV[3`\xA0\x86\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x85\x01RV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[a4\xDDV[\x01Q`\xFF\x16\x90V[\x90aH#V[\x9C\x01Q`\xFF\x16\x90V[\x86Q\x89\x81R` \x81\x01\x82\x90R`\x01`\x01`\x80\x1B\x03\x90\x94\x16`@\x85\x01R\x98`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x16\x93\x16\x91\x80``\x81\x01a\x0E6V[\x86QcVZ\xDE\xF5`\xE1\x1B\x81R\xFD[\x88Qc\x064HC`\xE4\x1B\x81R\x83\x90\xFD[P\x86Qc\nw\xB0/`\xE1\x1B\x81R\xFD[\x93Pa\x15\xEAa\x15\xE4\x86a\x15\xCC3`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\x01`\x01`@\x1B\x03\x16`\0R` R`@`\0 \x90V[TaU+V[\x93a\x14\rV[\x84Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16\x81\x88\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a\x16,\x91P\x85=\x87\x11a\x0F\xA4Wa\x0F\x96\x81\x83a\x18RV[8a\x13\xA7V[a\x16:a;\xBFV[a\x13<V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wb\xFF\xFF\xFFa\x16\\a\x06pV[\x16`\0R`\t` R`\x80`@`\0 `\xFF`\x01\x82T\x92\x01T`@Q\x92\x82`\x01\x80`\xA0\x1B\x03\x91\x82\x81\x16\x86R`\xA0\x1C\x16` \x85\x01R\x81\x16`@\x84\x01R`\xA0\x1C\x16``\x82\x01R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x16\xBEa2\xB9V[`@Qc\xF7|G\x91`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x16Z\xFA\x90\x81\x15a\x05OW`\0\x91a\x17\xA4W[P\x163\x03a\x17\x8FW`\x14\x81\x11\x80\x15a\x17\x85W[a\x17kW\x7F\x81\xC99\x14H\0(v\x03G\x9B\x97\xBB\xA9\xC1\x12\x88\xCEz\xBCZ\xCBH\x90y\xE1Y\xF3\\\xF9\x8B\xD1\x90`\rTa\x17S\x82`\rUV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xA1a\0!a3\x02V[`@QcdYtw`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x90\xFD[P`\x04\x81\x10a\x17!V[`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x90\xFD[a\x17\xC5\x91P` =\x81\x11a\x17\xCBW[a\x17\xBD\x81\x83a\x18RV[\x81\x01\x90aC\x97V[8a\x17\x0EV[P=a\x17\xB3V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[a\x17\xD2V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[a\x01\0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\x03W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[`@Q\x90a\x18\x80\x82a\x18#V[V[`@Q\x90a\x18\x80\x82a\x18\x08V[`\xA0\x90`\x03\x19\x01\x12a\x02\xB8W`@Q\x90a\x18\xA8\x82a\x17\xE8V[\x81`\x045a\x18\xB5\x81a\x0B\x06V[\x81R`$5a\x18\xC3\x81a\x0B\x06V[` \x82\x01R`D5a\x18\xD4\x81a\x05eV[`@\x82\x01R`d5a\x18\xE5\x81a\x05TV[``\x82\x01R`\x80`\x845\x91a\x18\xF9\x83a\x05eV[\x01RV[4a\x02\xB8W`\xE06`\x03\x19\x01\x12a\x02\xB8Wa\x19\x176a\x18\x8FV[``a\x19u`\xE4`\xC45\x93a\x19+\x85a\x02\xA7V[\x80\x84\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x90\x81\x16\x16\x90`@Q\x96\x87\x95\x86\x94c@W\xCE\xBB`\xE1\x1B\x86R`\x04\x86\x01\x90a'\rV[`\xA45`\xA4\x85\x01R\x16`\xC4\x83\x01RZ\xFA\x80\x15a\x05OW`\0\x90\x81\x92\x82\x91a\x19\xBBW[Pa\x04\xC2\x90`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x90Pa\x04\xC2\x92Pa\x19\xE3\x91P``=\x81\x11a\x19\xEAW[a\x19\xDB\x81\x83a\x18RV[\x81\x01\x90aD.V[\x90\x92a\x19\x97V[P=a\x19\xD1V[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x1A\x0E\x81a\x05TV[`$5\x90\x81`\x0F\x0B\x91\x82\x81\x03a\x02\xB8Wb\xFF\xFF\xFF\x90a\x1A}a\x08\x13a\x1AT`\0\x93a\x1AOa\x0C\xCB\x88`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[aM\x8BV[\x94\x90`\x01`\x01`\x80\x1B\x03\x80\x91\x16\x95\x16\x95` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93\x12\x15a\x1A\xCFWa\x1A\xB0\x91a\x154a\x08+``a\x088a\x1A\xABa\x1A\xAB\x96a\x154a\x08+` \x8C\x01Q`\xFF\x16\x90V[aU+V[\x90[`@\x80Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x81R\x92\x90\x91\x16` \x83\x01R\x90\xF3[a\x1A\xFC\x91a\x1A\xF6a\x08+``a\x088a\x1A\xABa\x1A\xAB\x96a\x1A\xF6a\x08+` \x8C\x01Q`\xFF\x16\x90V[\x90aH]V[\x90a\x1A\xB2V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x01`\x01`@\x1B\x03`\x045a\x1B'\x81a\x05TV[\x16`\0R`\n` R`@`\0 \x80Ta\x04\xC2`\x01\x83\x01T\x92a\x1Bg`\x03a\x1BX`\x02\x84\x01T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x92\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x86\x81\x16\x82R`\x80\x96\x87\x1C` \x83\x01R\x87\x16\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x86\x1C\x16``\x82\x01Ra\xFF\xFF`\xA0\x87\x81\x1C\x82\x16\x96\x83\x01\x96\x90\x96R`\xB0\x96\x90\x96\x1C\x90\x95\x16\x93\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x85\x01R\x90\x91\x16`\xE0\x83\x01R\x81\x90a\x01\0\x82\x01\x90V[4a\x02\xB8W``6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x1B\xF9\x81a\x05TV[a\x1C\x01a\x06\x82V[a\x1C\ta\x06\x93V[\x91a\x1C\x12a2\xB9V[a\x1C/\x81`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`\x02\x81\x01T\x90\x92\x903`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x17\x8FWa\xFF\xFF\x80\x85\x16\x94\x85a\x1D\x1FW[P\x81\x16\x92\x83a\x1C\x97W[PP`\x01`\x01`@\x1B\x03\x16\x7F\x80N\x0E\xF8\xEB\xD1\x19o\x98\xB3\xC6\xA20\xDE\xFF\xD8\xCF\xE0<\xB1\x92?\xE0\xE4\x02%-\x06\xD8\xD4v\xDA`\0\x80\xA4a\0!a3\x02V[`\x01\x01a\x1C\xD4a\x1C\xD0a\x1C\xBAa\x1C\xB3\x84Ta\xFF\xFF\x90`\xA0\x1C\x16\x90V[a\xFF\xFF\x16\x90V[\x86\x90\x80\x82\x10\x90\x82\x14\x17\x90`\x01\x80\x82\x11\x91\x14\x17\x16\x90V[\x15\x90V[a\x1D\x03W\x80Ta\xFF\xFF`\xB0\x1B\x19\x16`\xB0\x92\x90\x92\x1Ba\xFF\xFF`\xB0\x1B\x16\x91\x90\x91\x17\x90U`\x01`\x01`@\x1B\x038a\x1C_V[`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`$\x90\xFD[a\x03\xE8\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15a\x1D_W`\x01\x85\x01\x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x92\x90\x92\x1Ba\xFF\xFF`\xA0\x1B\x16\x91\x90\x91\x17\x90U8a\x1CUV[`@Qc\x97\x1B1\t`\xE0\x1B\x81Ra\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x1D\x9A\x81a\x02\xA7V[`$5\x90a\x1D\xA7\x82a\x05eV[3`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x91\x15\x15\x91`\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90U`@Q\x91\x82R`\x01\x80`\xA0\x1B\x03\x16\x90\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\0[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wb\xFF\xFF\xFFa\x1E/a\x06pV[\x16`\0R`\x08` R` c\xFF\xFF\xFF\xFF`@`\0 T\x16`@Q\x90\x81R\xF3[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x83\x01\x92\x81`@\x84`\x05\x1B\x83\x01\x01\x95\x01\x93`\0\x91[\x84\x83\x10a\x1E\x82WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a\x1E\xA0`\x01\x93`?\x19\x86\x82\x03\x01\x87R\x8AQa\x04\x13V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a\x1ErV[` \x80`\x03\x196\x01\x12a\x02\xB8W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB8Wa\x1E\xDC\x906\x90`\x04\x01a\x08eV[\x91a\x1E\xE9`\x0FT`\xFF\x16\x90V[a\x1F\xB0Wa\x1E\xF5a2\xB9V[a\x1F\x07`\x01`\xFF\x19`\x0FT\x16\x17`\x0FUV[a\x1F\x0Fa;\xBFV[a\x1F\x18\x83a1tV[\x92`\0\x90\x81[\x81\x81\x10a\x1FTWa\x04\xC2\x86a\x1F8`\xFF\x19`\x0FT\x16`\x0FUV[a\x1F@aACV[a\x1FHa3\x02V[`@Q\x91\x82\x91\x82a\x1ENV[\x82\x80a\x1Fa\x83\x85\x89a1\xFEV[\x90a\x1Fq`@Q\x80\x93\x81\x93a2DV[\x03\x900Z\xF4a\x1F~a2uV[\x90\x15a\x1F\xA9W\x90a\x1F\xA4\x91a\x1F\x93\x82\x89a2\xA5V[Ra\x1F\x9E\x81\x88a2\xA5V[Pa1\xD4V[a\x1F\x1EV[\x80Q\x90\x85\x01\xFD[`@QcU\xE1\xF7\xC5`\xE0\x1B\x81R`\x04\x90\xFD[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W` `\rT`@Q\x90\x81R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a \x87\x81a\x02\xA7V[`\x01\x80`\xA0\x1B\x03\x16`\0R`\x02` R` `@`\0 T`@Q\x90\x81R\xF3[a \xB06a\x10\xC2V[\x90a \xB9a2\xB9V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x90\x81\x16\x90\x81\x83\x14a#dWa \xFCa \xF3\x85a\n\xE1\x84`\x01\x80`\xA0\x1B\x03\x16`\0R`\x0B` R`@`\0 \x90V[Tb\xFF\xFF\xFF\x16\x90V[\x93b\xFF\xFF\xFF\x94\x85\x81\x16a#EWP`@\x94\x85Q\x92c1<\xE5g`\xE0\x1B\x92\x83\x85R` \x80\x86`\x04\x81\x8AZ\xFA\x95\x86\x15a\x05OW`\0\x96a#&W[P\x88Q\x94\x85R\x80\x85`\x04\x81\x8BZ\xFA\x94\x85\x15a\x05OW`\0\x95a\"\xF7W[P`\xFF\x86\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x15a\"\xDDWa!\x8Fa\x1C\xD0`\xFF\x87\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x90V[a\"\xC3W\x92a\"\x8D\x7F\xC0\xC5\xDF\x98\xA4\xCA\x87\xA3!\xA3;\xF1'|\xF3-1\xA9{l\xE1K\x97G8!I\xB9\xE2c\x1E\xA3\x93a\"fa\x04\xC2\x9B\x99\x97\x94a\"W\x8Aa\"3\x9D\x9B\x99a!\xE3a!\xDE`\x06Tb\xFF\xFF\xFF\x16\x90V[a86V[\x9E\x8Fa!\xFD\x81b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x19`\x06T\x16\x17`\x06UV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x0B` R`@\x90 a\" \x90\x87\x90a\n\xE1V[\x90b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[a\"Ma\">a\x18\x82V[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[\x85\x01\x90`\xFF\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16\x82\x8B\x01RV[`\xFF\x84\x16``\x82\x01Ra\"\x88\x8Ab\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[a8JV[\x86Q`\xFF\x94\x85\x16\x81R\x91\x90\x93\x16` \x82\x01R\x91\x86\x16\x91`@\x90\xA4a\"\xAFa3\x02V[Qb\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x88Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R`$\x90\xFD[\x88Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x87\x16`\x04\x82\x01R`$\x90\xFD[\x81a#\x18\x92\x96P=\x87\x11a#\x1FW[a#\x10\x81\x83a\x18RV[\x81\x01\x90a8\x1DV[\x938a!RV[P=a#\x06V[\x81a#>\x92\x97P=\x88\x11a#\x1FWa#\x10\x81\x83a\x18RV[\x948a!5V[`@Qc\xB0\x98\x8CC`\xE0\x1B\x81Rb\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[`@Qc\x01D\xD33`\xE2\x1B\x81R`\x04\x90\xFD[4a\x02\xB8W``6`\x03\x19\x01\x12a\x02\xB8Wa\x04\xC2a$8`\x045a#\x99\x81a\x05TV[b\xFF\xFF\xFF\x81` \x1C\x16`\0R`\t` R`@`\0 a$3a\x0C\xCBa$\x19a$\x11a\x08+a$\t`@Q\x96a#\xCE\x88a\x18\x08V[`\xFF``\x82T\x99`\x01\x83\x81\x80`\xA0\x1B\x03\x9C\x8D\x81\x16\x84R`\xA0\x1C\x16\x94\x85` \x84\x01R\x01T\x9A\x8B\x16`@\x82\x01R\x01\x98`\xA0\x1C\x16\x88R`$5aG\xE8V[\x95Q`\xFF\x16\x90V[`D5aG\xE8V[\x93`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[aLdV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a$o\x81a\x02\xA7V[`\x01\x80`\xA0\x1B\x03\x16`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a$\xAC\x81a\x02\xA7V[`$5\x90a$\xB8a2\xB9V[`@Qc\xF7|G\x91`\xE0\x1B\x81R` \x91`\x01`\x01`\xA0\x1B\x03\x91\x83\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x16Z\xFA\x80\x15a\x05OW\x83\x91`\0\x91a&5W[P\x163\x03a\x17\x8FW`@Qc1<\xE5g`\xE0\x1B\x81R\x91\x81\x16\x93`\0\x92\x90\x84\x82`\x04\x81\x89Z\xFA\x91\x82\x15a\x05OW\x7F\x1F\xDD\0 5\x88\x93U\x97\x13\xDE\xF8\xB4,\xADf\x1F\xFB\xC7U\xD1\xA2dY@'\x92\x14B\xBBV\xA0\x95a%\xDA\x95\x93a&\x16W[PP`\0\x19\x81\x03a%\xFEWPa%\xCA\x90a%\x9C`\xFFa%\x94\x85`\x01\x80`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[T\x92\x16aH\x0FV[\x81\x04\x92[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x07` R`@\x90 a%\xC3\x83\x82Ta8\x03V[\x90Ua<\xCCV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2`\x0FT`\xFF\x16\x15a%\xF1W[a\0!a3\x02V[a%\xF9aACV[a%\xE9V[\x91a&\x0E`\xFFa%\xCA\x93\x16aH\x0FV[\x83\x02\x90a%\xA0V[a&-\x92\x93P\x80=\x10a#\x1FWa#\x10\x81\x83a\x18RV[\x908\x80a%cV[a&L\x91P\x85=\x87\x11a\x17\xCBWa\x17\xBD\x81\x83a\x18RV[8a%\x0BV[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wa\x06\x1B` `\x045a&t\x81a\x05TV[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n\x83R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16`@Q\x80\x80\x95\x81\x94c8\xCCn\x8D`\xE2\x1B\x83R`\x04\x83\x01\x91\x90\x91`\x01`\x01`@\x1B\x03` \x82\x01\x93\x16\x90RV[4a\x02\xB8W` `\xFFa'\x01a&\xD86a\x10\xC2V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[`\x80\x80\x91`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[`\xA0\x81\x01\x92\x91a\x18\x80\x91\x90a'\rV[4a\x02\xB8W``6`\x03\x19\x01\x12a\x02\xB8W`\x80a(4`\xA0`\x045a'\x86\x81a\x05TV[`$5\x90a'\x93\x82a\x05eV[`D5\x91a'\xA0\x83a\x02\xA7V[`@Qa'\xAC\x81a\x17\xE8V[`\0\x96\x81\x88\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra'\xF0a\x0C\0a\x0C\0`\x03a\x0B\xC3\x86`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`@Qcx=\xC3\xCF`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x93\x16`\x04\x84\x01R\x90\x15\x15`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`D\x82\x01R\x92\x83\x91\x90\x82\x90\x81\x90`d\x82\x01\x90V[\x03\x91Z\xFA\x90\x81\x15a\x05OWa\x04\xC2\x92\x91a(VW[P`@Q\x91\x82\x91\x82a'RV[a(w\x91P`\xA0=\x81\x11a(}W[a(o\x81\x83a\x18RV[\x81\x01\x90aC\xBBV[8a(IV[P=a(eV[4a\x02\xB8W`\xA06`\x03\x19\x01\x12a\x02\xB8W`\x045a(\xA1\x81a\x02\xA7V[`$5a(\xAD\x81a\x02\xA7V[`D5\x91`d5`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB8Wa(\xD3\x906\x90`\x04\x01a\x06\xA4V[\x93\x90\x91`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x92\x903\x84\x14\x80\x15a)\xD7W[a(\xF6\x90aQgV[a)\x16\x88a\no\x88`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a)!\x84\x82Ta8\x03V[\x90Ua)C\x88a\no\x84`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a)N\x84\x82Ta8\x10V[\x90U\x81\x16\x80\x93\x88\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb`@Q\x80a)\x93\x883\x95\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA4;\x15a)\xC6W`\0` \x94\x95\x96a\t\xC6`@Q\x98\x89\x96\x87\x95\x86\x94c\xF2:na`\xE0\x1B\x9B\x8C\x87R3`\x04\x88\x01aQ\xF9V[P\x92PPPa\0!\x91P\x15\x15aQ\xA4V[Pa(\xF6a)\xFEa\n\xF83a\n\xE1\x8A`\x01\x80`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90Pa(\xEDV[`\xA06`\x03\x19\x01\x12a\x02\xB8Wa*\x19a2\xB9V[`\x0FT`\xFF\x16\x15a0\xC0W[a*-a7<V[a*5a7HV[\x90a*Ma*Aa7TV[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90`\x01`\x01`\x80\x1B\x03\x80\x93\x16\x91a*~a*ea7<V[`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[a*\x9Ca\x1C\xD0a*\x8D\x83a3\x9DV[``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[a0\x9FWa*\xB1a*\xAC\x82a3\x9DV[aL:V[a0\x8DWa*\xDDa\x0C\0a\x0C\0`\x03a\x0B\xC3\x87`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`@\x80Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x80\x83\x01\x91\x90\x91R` \x98\x92\x94\x92\x93\x90\x91\x89\x81`$\x81\x85Z\xFA\x90\x81\x15a\x05OW`\0\x91a0pW[P\x15a0KWa+.B\x84aJ\xE2V[a+q\x85a+:a7`V[\x81Qc;\x1C\xDA\x15`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x8B\x16\x86\x82\x01\x90\x81R\x91\x15\x15` \x83\x01R3`@\x83\x01R\x92\x83\x91\x82\x91``\x90\x91\x01\x90V[\x03\x81`\0\x86Z\xF1\x90\x81\x15a\x05OW`\0\x90\x81\x92a0+W[P\x15a0\x1BWa+\xB2a\x08\x13b\xFF\xFF\xFF\x8A\x8D\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x90\x8A\x87a+\xBDa7\x97V[\x93a+\xC6a7`V[\x15a/\xA8W\x90\x81a+\xEEa+\xE3a,8\x95a,(\x95\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x88\x01RV[a,\na+\xFF``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x88\x01RV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x87\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x84\x01RV[a,@a7lV[a/gW[\x90a,]\x91\x81R\x89`\x80\x82\x01R\x87`\xA0\x82\x01RaP\x8CV[\x92`\xA0\x84\x01\x90\x81Q\x15a/WW`\x80\x85\x01\x93\x84Q\x15a/IW\x87a,\xFA\x8Da,\xBCa,\x876a\x18\x8FV[\x91a,\xAFa,\x95\x8BQaU+V[\x91a,\xA0\x8AQaU+V[`\x01`\x01`\x80\x1B\x03\x16\x90\x85\x01RV[`\x01`\x01`\x80\x1B\x03\x16\x82RV[`\x02\x85\x01T`\x01`\x01`\xA0\x1B\x03\x9A\x903\x90\x8C\x16\x03a/7W`\x01\x86\x01T`\xB0\x1Ca\xFF\xFF\x16\x91[\x86T\x90a\xFF\xFF`\rT\x94\x16\x92\x82`\x80\x1C\x92\x16\x90aO\x1FV[\x93\x92``\x8B\x99\x92\x93\x99\x01\x98\x89R\x83\x8B\x01R\x8D\x8AQ\x90a-M\x85Q\x96\x87\x95\x86\x94\x85\x94c\xA4G\x89\x19`\xE0\x1B\x86R\x8B\x86\x01\x90\x94\x93\x92``\x92`\x01`\x01`@\x1B\x03`\x80\x84\x01\x97\x16\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x05OW\x8D`\0\x91\x82\x93a/\x06W[P\x82\x90\x89\x01R\x15a.\xE1WPP\x93\x88\x99\x9A\x93\x7F\xCD\x80T&o\xAE;\xBD#\0\xC8\x9A@t\xC4\x11=\x12\xB9\xA5\x83\xF7\xCD#\x90\xD40\xAFp\xB1*X\x93a-\xF7a.\x06\x94a-\xC4a\x04\xC2\x9D\x99a-\xAEa7`V[a-\xBB\x87Q\x87Q\x90a8\x03V[\x90\x84Q\x92aKqV[`\xC0\x85\x01\x80Q\x90\x94a-\xE1\x91`\x01`\x01`\xA0\x1B\x03\x16\x90Q\x90a<cV[`\xE0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90Q\x90a<\xCCV[Q\x90\x81a.\xB7W[PPaP\xFCV[\x93a.\x0Fa7`V[`\xC0\x86\x01Q`\xE0\x87\x01Q\x88\x88\x01Q\x95\x90\x97\x01Q\x88Q\x92\x15\x15\x83R` \x83\x01\x8D\x90R`@\x83\x01\x8A\x90R``\x83\x01\x95\x90\x95R`\x80\x82\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x95\x82\x16\x86\x16\x95\x91\x90\x93\x16\x16\x92`\x01`\x01`@\x1B\x03\x16\x91`\xA0\x90\xA4a.xa\x0EH`\x0FT`\xFF\x16\x90V[\x15a.\xAAW[a.\x86a3\x02V[Q\x93\x84\x93\x84`@\x91\x94\x93\x92`\x01`\x01`@\x1B\x03``\x83\x01\x96\x16\x82R` \x82\x01R\x01RV[a.\xB2aACV[a.~V[Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x90 a.\xD8\x90a\n\xA8V[\x90U8\x80a-\xFFV[\x86Q\x89Qc\\\x9E\xF7\x05`\xE1\x1B\x81R\x92\x83\x01\x90\x81R` \x81\x01\x91\x90\x91R\x81\x90`@\x01\x03\x90\xFD[\x90\x92Pa/)\x91P\x8A=\x8C\x11a/0W[a/!\x81\x83a\x18RV[\x81\x01\x90a7xV[\x918a-cV[P=a/\x17V[`\x01\x86\x01T`\xA0\x1Ca\xFF\xFF\x16\x91a,\xE2V[\x87Qc\x13\xFD\x9Bm`\xE0\x1B\x81R\xFD[\x86Qc7\xC2\xD9\xA7`\xE1\x1B\x81R\x84\x90\xFD[`\xC0\x82\x01Qa/~\x90`\x01`\x01`\xA0\x1B\x03\x16a1RV[`\0\x81\x13a/\x8DW[Pa,EV[a,]\x92\x91\x9APa*Aa/\xA0\x91aU+V[\x99\x90\x91a/\x87V[a,(\x91Pa/\xE7a/\xDCa0\x16\x94a/\xD3a/\xC8``\x86\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x8A\x01RV[\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x87\x01RV[\x80\x8A\x01Qa0\t\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x87\x01RV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a,8V[\x85Qc.\xD0\xEA\x01`\xE0\x1B\x81R\x83\x90\xFD[\x90Pa0D\x91P\x86=\x88\x11a/0Wa/!\x81\x83a\x18RV[\x908a+\x89V[\x84Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x88\x16\x81\x84\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a0\x87\x91P\x8A=\x8C\x11a\x0F\xA4Wa\x0F\x96\x81\x83a\x18RV[8a+\x1EV[`@Qcz\x95\xCB!`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`$\x90\xFD[a0\xC8a;\xBFV[a*%V[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W` `\0Rk\x0Bv1.4.0-beta`+R```\0\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x02\xB8WV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\x03W`\x1F\x01`\x1F\x19\x16` \x01\x90V[`@Q=`\0\x82>=\x90\xFD[a\x04I\x900\x90aGYV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\x03W`\x05\x1B` \x01\x90V[\x90a1~\x82a1]V[a1\x8B`@Q\x91\x82a\x18RV[\x82\x81R\x80\x92a1\x9C`\x1F\x19\x91a1]V[\x01\x90`\0[\x82\x81\x10a1\xADWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a1\xA1V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x14a1\xE3W`\x01\x01\x90V[a1\xBEV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x91\x90\x81\x10\x15a2?W`\x05\x1B\x81\x015\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x02\xB8W\x01\x90\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB8W` \x01\x826\x03\x81\x13a\x02\xB8W\x91\x90V[a1\xE8V[\x90\x80\x92\x91\x827\x01`\0\x81R\x90V[`@Q\x90` \x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@R`\0\x82RV[=\x15a2\xA0W=\x90a2\x86\x82a1+V[\x91a2\x94`@Q\x93\x84a\x18RV[\x82R=`\0` \x84\x01>V[``\x90V[\x80Q\x82\x10\x15a2?W` \x91`\x05\x1B\x01\x01\x90V[`\x0CT`\x01\x81\x14\x15\x80a2\xEBW[a2\xD9Wa2\xD4\x90a1\xD4V[`\x0CUV[`@Qc\x02\xB8\0-`\xE0\x1B\x81R`\x04\x90\xFD[P`\xFF`\x0FT\x16\x15\x80a2\xC7WP`\x02\x81\x11a2\xC7V[`\x0CT\x80\x15a1\xE3W`\0\x19\x01`\x0CU`\x05T`\xFF\x16\x15\x80a37W[a3%WV[`@Qc2n\xFAC`\xE0\x1B\x81R`\x04\x90\xFD[P`\xFF`\x0FT\x16\x15a3\x1FV[\x90\x81` \x91\x03\x12a\x02\xB8WQa\x04I\x81a\x05eV[\x90`@Qa3f\x81a\x18\x08V[\x82T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\xA0\x92\x83\x1C\x81\x16` \x85\x01R`\x01\x90\x95\x01T\x90\x81\x16`@\x84\x01R\x90\x1C\x90\x92\x16``\x83\x01RV[\x90a\x18\x80`@Qa3\xAD\x81a\x18#V[`\xE0a4K`\x03\x83\x96a44\x81Ta3\xE6`\x01`\x01`\x80\x1B\x03\x91a3\xDC\x83\x82\x16\x8A\x90`\x01`\x01`\x80\x1B\x03\x16\x90RV[`\x80\x1C` \x89\x01RV[a4\x03`\x01\x84\x01T\x91\x82\x16`@\x89\x01\x90`\x01`\x01`\x80\x1B\x03\x16\x90RV[`\x80\x81\x90\x1Cc\xFF\xFF\xFF\xFF\x16``\x88\x01Ra\xFF\xFF`\xA0\x82\x90\x1C\x81\x16`\x80\x89\x01R\x90`\xB0\x1C\x16`\xA0\x87\x01\x90a\xFF\xFF\x16\x90RV[`\x02\x81\x01Ta\x0B\xC3\x90`\x01`\x01`\xA0\x1B\x03\x16a/\xF9V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14a1\xE3W`\0\x03\x90V[`\x0F\x0Bc;\x9A\xC9\xFF\x19\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a1\xE3WV[`\x01`\xFF\x1B\x81\x14a1\xE3W`\0\x03\x90V[\x91\x90\x91`\x01`\x01`\x80\x1B\x03\x80\x80\x94\x16\x91\x16\x01\x91\x82\x11a1\xE3WV[`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x90\x82\x16\x03\x91\x90\x82\x11a1\xE3WV[`\x80\x81\x01\x90a4\xF6a*e\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90a5\x04` \x82\x01QaU+V[\x90a5\x12`@\x82\x01QaU+V[\x93``\x82\x01\x91a5#\x83Q`\x0F\x0B\x90V[`\x01`\x01`\x80\x1B\x03\x92\x83a5A`\x01\x89\x01T`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x15a6\xFAW[a*ea5\xAA\x91a5\xB5\x93`\x0F\x0B`\0\x81\x13`\0\x14a6\xB8W`\xA0\x86\x01Qa5\x9D\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`@\x1B\x03a5\x8E\x85Q`\x01`\x01`@\x1B\x03\x16\x90V[\x16a5\x97a2RV[\x92aS\x85V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x84Q`\x0F\x0B\x90aK\x15V[`\0a5\xF2a5\xECa5\xE4`\xE0a5\xD5`\xC0\x87\x01Q`\x01\x80`\xA0\x1B\x03\x16\x90V[\x95\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x12\x15a6vW\x83a6 a\x18\x80\x97\x94\x84a6\x18a60\x95a6L\x97a6V\x9A\x16\x90a<\xCCV[\x86\x16\x90a<\xCCV[\x85T`\x01`\x01`\x80\x1B\x03\x16a4\xC4V[\x84T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x84UV[\x82T`\x80\x1Ca4\xC4V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x91\x90\x91\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x90UV[\x83a6\x9Ea\x18\x80\x97\x94\x84a6\x96a60\x95a6\xAE\x97a6V\x9A\x16\x90a<cV[\x86\x16\x90a<cV[\x85T`\x01`\x01`\x80\x1B\x03\x16a4\xA9V[\x82T`\x80\x1Ca4\xA9V[`\xA0\x86\x01Qa6\xF5\x91\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`@\x1B\x03a6\xEDa6\xE7\x86Q`\x01`\x01`@\x1B\x03\x16\x90V[\x93a4\x98V[\x92\x16\x90aT]V[a5\x9DV[`\0\x87Uc;\x9A\xCA\0`\x0F\x83\x90\x0B\x12a7*Wa*ea5\xAA\x91a7 a5\xB5\x94a4qV[\x93P\x91PPa5HV[`@Qc\xCBm\xABu`\xE0\x1B\x81R`\x04\x90\xFD[`d5a\x04I\x81a\x05TV[`\x045a\x04I\x81a\x0B\x06V[`$5a\x04I\x81a\x0B\x06V[`\x845a\x04I\x81a\x05eV[`D5a\x04I\x81a\x05eV[\x91\x90\x82`@\x91\x03\x12a\x02\xB8W` \x82Qa7\x91\x81a\x05eV[\x92\x01Q\x90V[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[`\0\x19\x81\x01\x91\x90\x82\x11a1\xE3WV[\x91\x90\x82\x03\x91\x82\x11a1\xE3WV[\x91\x90\x82\x01\x80\x92\x11a1\xE3WV[\x90\x81` \x91\x03\x12a\x02\xB8WQ`\xFF\x81\x16\x81\x03a\x02\xB8W\x90V[b\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a1\xE3W`\x01\x01\x90V[\x81Q\x81T` \x84\x01Q`\xFF`\xA0\x1B`\xA0\x91\x90\x91\x1B\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x17\x82U`@\x83\x01Q`\x01\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x91\x16\x91\x90\x91\x17\x81Ua\x18\x80\x91`\xFF\x90``\x01Q\x82T`\xFF`\xA0\x1B\x19\x16\x91\x16`\xA0\x1B`\xFF`\xA0\x1B\x16\x17\x90UV[c\xFF\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a1\xE3W`\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90`\x01`\x01`@\x1B\x03a\x04I\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a8\xD3V[\x96\x98\x97\x90\x94\x95\x92\x95a9$a2\xB9V[b\xFF\xFF\xFF\x97\x80\x89\x16a;\xB9WP`\x06Tb\xFF\xFF\xFF\x16\x97[\x88\x16\x15a;\xA7Wa9[\x88b\xFF\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x90a9\x8Ba9ua9p\x84Tc\xFF\xFF\xFF\xFF\x16\x90V[a8\xBEV[\x83Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x82\x16\x17\x90\x93UV[`\x01`\x01`\xA0\x1B\x03\x94\x80\x86\x16a;\xA1WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91[a9\xF6\x90\x8A\x83\x88\x16\x15\x15\x85\x89\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x16\x14\x15aH\xA2V[\x9A\x8B\x93a:\x16\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[\x84\x84\x8C\x8C\x8A\x8Da\xFF\xFF\x80\x91\x16\x93\x16\x91a:.\x96aH\xF6V[`\x0F\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x08\x87\x90\x1Bh\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x17\x90Ua:n\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`\x03\x01T`@Qc\xE0hx\x7F`\xE0\x1B\x81R\x95`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x86\x92\x83\x92a:\x9E\x92\x90`\x04\x85\x01a8\xF4V[\x03\x81Z` \x94`\0\x91\xF1\x95\x86\x15a\x05OW\x8B\x98a;$`\x01a\x0B\xC3a;\x0Ea;\x01\x8Fa;x\x9A\x7Fa<|I?\x9A\xA3\x97M\xCD\xBB\x87\xE6\xFE\xA5L\xCD\xB9\x10>\xA7\x83.m\xF0\x8D\xB8\x1Co\x90\xD2\x9D\x9Ea;\x83W[Pb\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x9Db\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[`@\x80Q\x9A\x8BR` \x8B\x01\x97\x90\x97Ra\xFF\xFF\x91\x82\x16\x96\x8A\x01\x96\x90\x96R\x16``\x88\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x88\x01R\x16`\xA0\x86\x01R\x90\x82\x16\x95\x90\x91\x16\x93`\x01`\x01`@\x1B\x03\x16\x92\x90\x81\x90`\xC0\x82\x01\x90V[\x03\x90\xA4a\x18\x80a3\x02V[a;\x9A\x90` =\x81\x11a\x0F\xA4Wa\x0F\x96\x81\x83a\x18RV[P8a:\xEBV[\x91a9\xBFV[`@Qc\xCCzs\x9B`\xE0\x1B\x81R`\x04\x90\xFD[\x97a9;V[4a;\xC6WV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a;\xF0\x81aFlV[`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xB8W`\0`\x04\x91`@Q\x92\x83\x80\x92c\r\x0E0\xDB`\xE4\x1B\x82R4\x90Z\xF1\x80\x15a\x05OWa<TW[P`@Q4\x81R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C` 3\x92\xA2V[a<]\x90a\x18?V[8a<%V[a<l\x81aFlV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 \x80T\x91\x83\x83\x01\x80\x93\x11a1\xE3W\x91\x90U`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x80\xB2\x17H\xC7\x87\xC5.\x87\xA6\xB2\"\x01\x1E\n\x0E\xD0\xF9\xCC \x15\xF0\xCE\xD4gHd-\xC6.\xE9\xF8\x90` \x90\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x80\x83\x11a=VWPa<\xF6\x81aFlV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 \x80T\x91\x83\x83\x03\x92\x83\x11a1\xE3W\x91\x90U`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x1Cq\x1E\xCA\x8D\x0BiK\xBC\xB0\xA1Db\xA7\0b\"\xE7!\x95K,_\xF7\x98\xF6\x06\x81~\xB1\x102\x90` \x90\xA2V[`@Qc1Rv\xC9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[b\xFF\xFF\xFF\x90\x93\x91\x92\x93` \x1C\x16`\0R`\t` R`@`\0 \x92`@Qa=\xA0\x81a\x18\x08V[`\xFF``\x86T\x92`\x01\x83\x81\x80`\xA0\x1B\x03\x95\x86\x81\x16\x84R`\xA0\x1C\x16\x98\x89` \x84\x01R\x01T\x93\x84\x16`@\x82\x01R\x01\x91`\xA0\x1C\x16\x81Ra=\xDC\x84aU+V[\x94`\x01`\x01`\x80\x1B\x03\x94\x85\x81\x03a>\"W[PPa=\xF9\x82aU+V[\x93\x82\x03a>\x04WPPV[a\x04I\x92\x93P\x90a>\x1Ca\x08+a\x1A\xAB\x93Q`\xFF\x16\x90V[\x90aG\xE8V[a>2\x92\x96P\x90a\x1A\xAB\x91aG\xE8V[\x938\x80a=\xEEV[`@Q\x90`\x04T\x80\x83R\x82` \x91\x82\x82\x01\x90`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93`\0\x90[\x82\x82\x10a>\x8DWPPPa\x18\x80\x92P\x03\x83a\x18RV[\x85T`\x01`\x01`\xA0\x1B\x03\x16\x84R`\x01\x95\x86\x01\x95\x88\x95P\x93\x81\x01\x93\x90\x91\x01\x90a>wV[`\x0ET\x90`\x01`@\x1B\x82\x10\x15a\x18\x03W`\x01\x82\x01\x80`\x0EU\x82\x10\x15a2?W`\x0E`\0R\x80Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD`\x02\x93\x90\x93\x1B\x92\x83\x01U` \x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x83\x01U`@\x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFF\x83\x01U``\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\0\x91\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x04T\x80\x15a?\xF5W`\0\x19\x81\x01\x90\x80\x82\x10\x15a2?W\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9A\x90`\x04`\0R\x01k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x81T\x16\x90U`\x04UV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\x0ET\x90a@\x18\x82a1]V[\x91`@a@'\x81Q\x94\x85a\x18RV[\x81\x84R\x83` \x80\x91\x01\x91`\x0E`\0R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90`\0\x93[\x85\x85\x10a@kWPPPPPPV[`\x04\x84`\x01\x92\x84Qa@|\x81a\x18\x08V[\x86T\x81R\x84\x87\x01T\x83\x82\x01R`\x02\x87\x01T\x86\x82\x01R\x84\x80`\xA0\x1B\x03`\x03\x88\x01T\x16``\x82\x01R\x81R\x01\x93\x01\x94\x01\x93\x91a@\\V[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a1\xE3WV[`\x0ET`\0\x80`\x0EU\x81a@\xDBWPPV[`\x01`\x01`\xFE\x1B\x03\x82\x16\x82\x03a1\xE3W`\x0E\x81R`\x02\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x92\x81\x1B\x83\x01\x92[\x83\x81\x10aA&WPPPPV[\x80\x83`\x04\x92U\x83`\x01\x82\x01U\x83\x83\x82\x01U\x83`\x03\x82\x01U\x01aA\x19V[aAKa>:V[\x80Q\x80\x15aC\x8DW\x90`\x01\x91\x82\x80[aB\xE7W[PPPPaAka@\x0BV[\x80Q\x80[aA\x85WPPaA}aG(V[a\x18\x80a@\xC9V[aA\x8E\x81a7\xF4V[aA\xAC``aA\x9D\x83\x86a2\xA5V[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[aA\xB6\x82\x85a2\xA5V[QQ` aA\xC4\x84\x87a2\xA5V[Q\x01Q\x81\x15aB\x97WPaB4aA\xE0\x93`@\x94\x85\x91\x88a2\xA5V[Q\x01Q\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x83\x16\x03aB\x86WaB$\x913\x90aD\xE5V[aB.0\x85aDRV[\x92a8\x03V[\x90\x81\x81\x10aBKWPPPP[`\0\x19\x01\x80aAoV[a\x0Fz\x91aBX\x91a@\xB0V[\x92QcU\x13N\xF1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x90\x81\x90`D\x82\x01\x90V[aB\x92\x91P3\x86aN\xCDV[aB$V[\x80\x91PaB\xA7W[PPPaBAV[aB\xD9aB\xB9\x93`@\x94\x85\x91\x88a2\xA5V[Q\x01Q\x91aB\xC9\x8103\x87aNvV[aB\xD30\x85aDRV[\x92a8\x10V[\x90\x81\x81\x10aBKWPaB\x9FV[\x15aC\x82W[`\0\x90aC\x05a0\taB\xFF\x83a7\xF4V[\x85a2\xA5V[aC\x0F0\x82aEUV[\x81\x15\x80\x15\x90aCyW[aC4W[PPPaC)a?\x9CV[`\0\x19\x01\x90\x83aAZV[aCq\x92aCl\x91aCF0\x83aDRV[\x90aCOa\x18\x82V[\x94\x85R` \x85\x01R`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01RV[a>\xB0V[8\x80\x80aC\x1EV[P\x80\x15\x15aC\x19V[\x80aB\xEDW\x80aA_V[PPa\x18\x80aG(V[\x90\x81` \x91\x03\x12a\x02\xB8WQa\x04I\x81a\x02\xA7V[\x90\x81` \x91\x03\x12a\x02\xB8WQ\x90V[\x90\x81`\xA0\x91\x03\x12a\x02\xB8W`\x80`@Q\x91aC\xD5\x83a\x17\xE8V[\x80QaC\xE0\x81a\x0B\x06V[\x83R` \x81\x01QaC\xF0\x81a\x0B\x06V[` \x84\x01R`@\x81\x01QaD\x03\x81a\x05eV[`@\x84\x01R``\x81\x01QaD\x16\x81a\x05TV[``\x84\x01R\x01QaD&\x81a\x05eV[`\x80\x82\x01R\x90V[\x90\x81``\x91\x03\x12a\x02\xB8W\x80QaDD\x81a\x05eV[\x91`@` \x83\x01Q\x92\x01Q\x90V[`@Qcp\xA0\x821`\xE0\x1B` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`$\x80\x83\x01\x91\x90\x91R\x81R\x91``\x83\x01\x91\x90`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x18\x03W`\0\x93\x84\x93`@RQ\x91Z\xFAaD\xA7a2uV[\x90\x15\x80\x15aD\xD9W[aD\xC7W\x80` \x80a\x04I\x93Q\x83\x01\x01\x91\x01aC\xACV[`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x90\xFD[P` \x81Q\x14\x15aD\xB0V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\xB8W`@Q\x80\x92c.\x1A}M`\xE0\x1B\x82R\x81`$`\0\x95\x86\x80\x94\x89`\x04\x84\x01RZ\xF1\x80\x15a\x05OWaEBW[P\x81\x80\x93\x81\x92Z\xF1\x15aE0WV[`@Qc5e\x94\xAB`\xE0\x1B\x81R`\x04\x90\xFD[aEN\x90\x92\x91\x92a\x18?V[\x908aE!V[\x91`\0\x80\x81\x93aEw\x86`\x01\x80`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T`@Qc1<\xE5g`\xE0\x1B\x81R\x90` \x82`\x04\x81`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xFA\x90\x81\x15a\x05OW`\xFFaE\xB8\x92aE\xBF\x94\x88\x91aFNW[P\x16\x90aH]V[\x91\x87aDRV[`\x01`\x01`\xFF\x1B\x03\x80\x83\x11aFJW\x81\x11a\x05KW\x90aE\xDE\x91a@\xB0V[\x91\x80\x83\x13\x15aF\rWPP`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x90\x81R`\x03` R`@\x90 [\x80T`\xFF\x19\x16\x90UV[\x82\x91\x95\x92\x12aF4W[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 aF\x03V[aF\x03\x91\x93PaFC\x90a4\x98V[\x92\x90aF\x17V[\x84\x80\xFD[aFf\x91P` =\x81\x11a#\x1FWa#\x10\x81\x83a\x18RV[8aE\xB0V[`\x05T`\xFF\x81\x16aG\x1BW[P`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x03` R`@\x90 T\x90\x91\x90`\xFF\x16\x15aF\xA1WPPV[`\x04T\x90`\x01`@\x1B\x82\x10\x15a\x18\x03W`\x01\x82\x01\x80`\x04U\x82\x10\x15a2?W\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\0R`\x03` R`@`\0 `\x01`\xFF\x19\x82T\x16\x17\x90UV[`\xFF\x19\x16`\x05U8aFxV[`\x04TaGCW`\x01`\xFF\x19`\x05T\x16\x17`\x05U`\0`\x04UV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 T\x90Qc1<\xE5g`\xE0\x1B\x81R\x92\x96\x95\x92\x93\x90\x91\x90\x84\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x05OW`\xFFaG\xB1\x92aG\xB7\x95\x89\x91aFNWP\x16\x90aH]V[\x92aDRV[`\x01`\x01`\xFF\x1B\x03\x80\x83\x11aFJW\x81\x11a\x05KWa\x04I\x92\x93Pa@\xB0V[`\x01`\x01`\x7F\x1B\x03\x81\x11a\x02\xB8W\x90V[\x90`\x12\x03`\x12\x81\x11a1\xE3WaG\xFD\x90aH\x01V[\x02\x90V[`M\x81\x11a1\xE3W`\n\n\x90V[`\x12\x03`\x12\x81\x11a1\xE3Wa\x04I\x90aH\x01V[\x90`\x12\x03`\x12\x81\x11a1\xE3WaH8\x90aH\x01V[\x90\x04\x90V[\x81\x15aHGW\x04\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x90\x81\x15aH\x9BW`\x12\x03`\x12\x81\x11a1\xE3WaHx\x90aH\x01V[`\0\x19\x82\x01\x91\x82\x11a1\xE3WaH\x8D\x91aH=V[`\x01\x81\x01\x80\x91\x11a1\xE3W\x90V[PP`\0\x90V[g\xFF\0\0\0\0\0\0\0\x91\x90`\0\x90\x15aH\xF0WP`\x01\x90[`\0\x90\x15aH\xEAWP`\x01\x92[` \x1B\x92`\x0F`\xF8\x1B\x90`\xF8\x1B\x16\x90`\x0F`\xFC\x1B\x90`\xFC\x1B\x16\x17`\xC0\x1C\x16\x17\x17\x90V[\x92aH\xC7V[\x90aH\xBAV[\x93\x96\x95\x94\x91\x90aI\x08a*\x8D\x86a3\x9DV[aJ\xBEWaI\x16B\x86aJ\xE2V[\x80\x15aJ\xACWaI(aID\x91aU+V[\x85T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x85UV[\x80\x15aJ\x9AWaIVaIv\x91aU+V[\x84T`\x01`\x01`\x80\x1B\x03\x16`\x80\x91\x90\x91\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x84UV[a\x03\xE8\x80\x83\x10\x90\x83\x14\x17`\x01\x83\x11`\x01\x84\x14\x17\x16\x15aJ\x81WaI\x98\x82aJ\xD0V[`\x01\x84\x01\x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x92\x90\x92\x1Ba\xFF\xFF`\xA0\x1B\x16\x91\x90\x91\x17\x81U\x91`\x01`\x01`\xA0\x1B\x03\x82\x16aI\xF0W[PPP`\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x90PV[\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15aJhW\x82\x91aJCa\x18\x80\x96\x97aJ>aJ_\x94`\x02`\x03\x98\x01\x90`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90UV[aJ\xD0V[\x81Ta\xFF\xFF`\xB0\x1B\x19\x16`\xB0\x91\x90\x91\x1Ba\xFF\xFF`\xB0\x1B\x16\x17\x90UV[\x90\x84\x938aI\xC9V[`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x90\xFD[`@Qc\x97\x1B1\t`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`@Qc(i\xC5\xF3`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x17O\xD4\x1B`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\xE90\xCE\xDF`\xE0\x1B\x81R`\x04\x90\xFD[b\x01\0\0\x81\x10\x15a\x02\xB8Wa\xFF\xFF\x16\x90V[d\x01\0\0\0\0\x82\x10\x15a\x02\xB8W`\x01\x01\x80Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x80\x92\x90\x92\x1Bc\xFF\xFF\xFF\xFF`\x80\x1B\x16\x91\x90\x91\x17\x90UV[\x90`\x01a\x18\x80\x92\x01\x90`\x01`\x01`\x80\x1B\x03\x80\x83T\x16\x90`\0\x83`\x0F\x0B\x13`\0\x14aK^WaKD\x92\x16\x90a4\xA9V[`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x19\x82T\x16\x17\x90UV[aKjaKD\x93a4YV[\x16\x90a4\xC4V[\x92\x91\x90\x15aL\x02WaK\xBEaK\xE8\x92a\x1A\xABaK\xA2aK\x92aK\xC8\x95aU+V[\x87T`\x01`\x01`\x80\x1B\x03\x16a4\xA9V[\x86T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x86UV[\x83T`\x80\x1Ca4\xC4V[\x82T`\x01`\x01`\x80\x1B\x03\x16`\x80\x91\x90\x91\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UV[T`\x01`\x01`\x80\x1B\x03\x81\x16\x15aJ\xACW`\x80\x1C\x15aJ\x9AWV[aL+aK\xC8\x91a\x1A\xABaK\xA2aL\x1BaL5\x96aU+V[\x87T`\x01`\x01`\x80\x1B\x03\x16a4\xC4V[\x83T`\x80\x1Ca4\xA9V[aK\xE8V[c\xFF\xFF\xFF\xFF``\x82\x01Q\x16\x15\x15\x90\x81aLQWP\x90V[`\x01`\x01`\x80\x1B\x03\x91P`@\x01Q\x16\x15\x90V[\x81\x15\x80aMiW[aM<W\x82\x15\x80aMDW[aM<Wa\x04I\x92`\0\x92\x83\x92aL\x8E\x81aL:V[\x15aM\x1FWg\r\xE0\xB6\xB3\xA7d\0\0\x91[`\x01`\x01`\x80\x1B\x03aL\xB7\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x90\x81aM\x01W[PP` \x01QaL\xD7\x90`\x01`\x01`\x80\x1B\x03\x16a*AV[\x91\x82aL\xEFW[PPP\x80\x82\x11\x90\x82\x03\x02\x90\x03aU+V[aL\xF9\x93PaT\xC5V[8\x80\x80aL\xDEV[aL\xD7\x92\x96PaM\x17a*A\x92\x85` \x93aT\xC5V[\x96\x92PaL\xBFV[aM6a*A`@\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91aL\x9EV[PPP`\0\x90V[P`\x01`\x01`\x80\x1B\x03aMa` \x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x15\x15aLxV[P`\x01`\x01`\x80\x1B\x03aM\x83\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x15\x15aLlV[\x91\x90\x80`\x0F\x0B\x90\x81\x15aNjW`\0aM\xB1a*A`@\x87\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x92\x13\x15aN%Wa\x04I\x91a\x1A\xAB\x91aM\xC9\x86aL:V[aN\x15W[`\x01`\x01`\x80\x1B\x03\x16aN\x0Fa*A` aN\0a\x1A\xAB\x86aM\xFAa*A\x8DQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87aU\x03V[\x98\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90aU\x03V[g\r\xE0\xB6\xB3\xA7d\0\0\x91PaM\xCEV[a\x04I\x91aN;a*Aa*Aa\x1A\xAB\x94a4YV[aNda*A` aN\0a\x1A\xAB\x86aN^a*A\x8DQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87aT\xC5V[\x90aT\xC5V[PP\x90P`\0\x90`\0\x90V[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15aN\xBEWPV[cn\x89\xEC\xA5`\xE0\x1B\x81R`\x04\x90\xFD[\x90\x91` \x90`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15aO\x10WPV[c:\xCB=?`\xE2\x1B\x81R`\x04\x90\xFD[\x91\x94\x92\x94`\0aO:`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16aT\xDDV[\x95\x80aPoW[P\x80\x95\x80\x97`\x80\x86\x01\x92aO\x8CaOX\x85Q\x15\x15\x90V[\x93\x84\x15aPhW\x87\x94[\x15aP^WaO\x87\x87\x95[aO\x81a*A\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a8\x10V[a8\x03V[\x90\x81\x81\x11aPLWaO\xA1` \x91\x85\x93a8\x03V[\x97\x01\x91aO\xB5\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11aP:WaO\xE3\x91aO\xD6a*AaO\xDC\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a8\x03V[\x91Q\x15\x15\x90V[\x90\x81\x15aP1W\x84\x85\x92[\x15aP)WP\x92[\x14aP\x17W\x81\x14aP\x05W\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92aO\xF6V[\x80\x94\x85\x92aO\xEEV[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[aO\x87\x88\x95aOmV[\x86\x94aObV[aPz\x91P\x86aH=V[\x94\x85\x81\x03\x90\x81\x11a1\xE3W\x948aOAV[aP\x94a7\x97V[P`@\x81\x01\x80Q\x90aP\xB0a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aG\xE8V[\x90R``\x82\x01aP\xC6\x81Q`\xFF\x84Q\x16\x90aG\xE8V[\x90RaP\xDD`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aG\xE8V[\x90R`\xA0\x81\x01aP\xF7\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aG\xE8V[\x90R\x90V[aQ\x04a7\x97V[P`@\x81\x01\x80Q\x90aQ a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aH#V[\x90R``\x82\x01aQ6\x81Q`\xFF\x84Q\x16\x90aH#V[\x90RaQM`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aH#V[\x90R`\xA0\x81\x01aP\xF7\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aH#V[\x15aQnWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x90\xFD[\x15aQ\xABWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\x02\xB8WQa\x04I\x81a\x03\x08V[\x91\x92a\x04I\x96\x94\x91`\xA0\x94`\x01\x80\x87\x1B\x03\x80\x92\x16\x85R\x16` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x91a8\xD3V[\x15aR3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`D\x82\x01R`d\x90\xFD[\x15aRrWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\t\x88\xA9\xC8\xEA\x89\x0B\xE9\xA9*i\xA8*\x88i`\x8B\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a2?W`\x05\x1B\x01\x90V[\x81\x83R\x90\x91`\x01`\x01`\xFB\x1B\x03\x83\x11a\x02\xB8W` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x92\x90aR\xF6\x90a\x04I\x95\x93`@\x86R`@\x86\x01\x91aR\xB9V[\x92` \x81\x85\x03\x91\x01RaR\xB9V[\x96\x94\x92aSF\x94aS8\x92a\x04I\x9A\x98\x94`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x8BR\x16` \x8A\x01R`\xA0`@\x8A\x01R`\xA0\x89\x01\x91aR\xB9V[\x91\x86\x83\x03``\x88\x01RaR\xB9V[\x92`\x80\x81\x85\x03\x91\x01Ra8\xD3V[\x90\x92`\xA0\x92a\x04I\x95\x94`\x01\x80\x86\x1B\x03\x16\x83R`\0` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x90a\x04\x13V[\x92\x90\x91aS\xA8\x83a\no\x86`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[aS\xB3\x82\x82Ta8\x10V[\x90U`@\x80Q\x84\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x95\x91\x86\x91`\0\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x90\xA4;\x15aTOW\x90aT)\x93` \x92`\0`@Q\x80\x97\x81\x95\x82\x94c\xF2:na`\xE0\x1B\x99\x8A\x85R3`\x04\x86\x01aSTV[\x03\x92Z\xF1\x80\x15a\x05OWa\x18\x80\x92`\0\x91a\t\xEDWP`\x01`\x01`\xE0\x1B\x03\x19\x16\x14aR,V[PPPa\x18\x80\x90\x15\x15aQ\xA4V[`\x01\x80`\xA0\x1B\x03\x16\x90`\0\x92\x82\x84R\x83` R`@\x84 \x82\x85R` R`@\x84 \x80T\x91\x80\x83\x03\x92\x83\x11a1\xE3W\x91\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R3\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x90\xA4V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xB8W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB8W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x91\x90\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\xB8W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01`\x80\x1B\x81\x10\x15a\x02\xB8W`\x01`\x01`\x80\x1B\x03\x16\x90V\xFE\xA2dipfsX\"\x12 \x14K\xAD\x9DW[\xEAk\x12\xB6\xCF\x8B\xF0\x84\x8E\\\x9A\xEFH\xF7\x18\xADY\x16\xD8QB\0\xF2k\xBB\xBDdsolcC\0\x08\x13\x003`\xA04b\0\0\xD6W`\x1Fb\x005;8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\0\xDBW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12b\0\0\xD6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03b\0\0\xD6W`\x80R`@Q\x90\x7F;5\xCD\xE5\x85\"y\xBF\xD1\xCE\x80S\x8F\x94\xC2\xE4*S\x0F\x11\x1FVB\x9B\x91,\x85\xCF\xA6P\xD4\xD3`\0\x80\xA2a4I\x90\x81b\0\0\xF2\x829`\x80Q\x81\x81\x81`\xFD\x01R\x81\x81a\x01\xDF\x01R\x81\x81a\x04\xC0\x01R\x81\x81a\x08\xB9\x01R\x81\x81a\nV\x01R\x81\x81a\x0B\xAE\x01R\x81\x81a\x0CN\x01R\x81\x81a\x0F:\x01R\x81\x81a\x12(\x01Ra\x15/\x01R\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x16\xED\xE0\x16\x14a\0\xE7W\x80c\x19\x05x\x07\x14a\0\xE2W\x80c4\xDB\xC7;\x14a\0\xDDW\x80c9CMZ\x14a\0\xD8W\x80cE-/\x18\x14a\0\xD3W\x80cK\xF3F\xBF\x14a\0\xCEW\x80c\x80\xAF\x9Dv\x14a\0\xC9W\x80c\xA4G\x89\x19\x14a\0\xC4W\x80c\xE0hx\x7F\x14a\0\xBFW\x80c\xE31\xBA4\x14a\0\xBAW\x80c\xE6\x04{\x19\x14a\0\xB5W\x80c\xECshT\x14a\0\xB0Wc\xF0{\x87\x9E\x14a\0\xABW`\0\x80\xFD[a\x0C\x0BV[a\x0B\x9DV[a\x0B<V[a\n\rV[a\t\x9CV[a\x08yV[a\x07\xCFV[a\x07TV[a\x05\xA3V[a\x04|V[a\x04\x07V[a\x01\x87V[4a\x01,W`\x006`\x03\x19\x01\x12a\x01,W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\0\x80\xFD[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01,WV[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01,WV[\x80\x15\x15\x03a\x01,WV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01,WV[`\xC45\x90a\x01\x85\x82a\x01gV[V[4a\x01,W`\x806`\x03\x19\x01\x12a\x01,Wa\x01\xA0a\x011V[`$5a\x01\xAC\x81a\x01]V[`d5\x91a\x01\xB9\x83a\x01gV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93a\x01\0\x91\x82\x81`$\x81\x89Z\xFA\x92\x83\x15a\x03\x8CW`\0\x93a\x03\xD8W[PP`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x84\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x91`\x80\x83`$\x81\x8AZ\xFA\x92\x83\x15a\x03\x8CW`\0\x93a\x03\xA5W[P`\x04\x90\x86\x15a\x03\x91W\x82a\x02v`\xFFa\x02p\x83\x88\x01Q`\xFF\x16\x90V[\x16a\x14JV[\x98a\x02\xDCa\x02\xA4a\x02\x9A\x8A`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x9B`D5\x02a3TV[\x98a\x02\xBFa\x02\xB0a\x06\xF4V[`\x01`\x01`\x80\x1B\x03\x90\x9B\x16\x8BRV[`\x01\x8A\x85\x01R`\0`@\x8B\x01R`\x01`\x01`@\x1B\x03\x16``\x8A\x01RV[\x88\x15\x15`\x80\x89\x01R`@QcXq\x0FE`\xE1\x1B\x81R\x93\x84\x91\x82\x90Z\xFA\x80\x15a\x03\x8CWa\x03J\x98a\x039\x97`\xFF\x97a\x03&\x95`\0\x94a\x03]W[Pa\x03 B\x93a\x10\xBEV[\x90a$\xEDV[\x94\x15a\x03NWP``\x01Q`\xFF\x16a\x02pV[`@Q\x91\x04\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x01Q`\xFF\x16a\x02pV[a\x02pV[a\x03~\x91\x94P\x87=\x89\x11a\x03\x85W[a\x03v\x81\x83a\x06\xD3V[\x81\x01\x90a\x13\xE2V[\x928a\x03\x15V[P=a\x03lV[a\x11\xF2V[\x82a\x02v`\xFFa\x03X``\x88\x01Q`\xFF\x16\x90V[`\x04\x91\x93Pa\x03\xCA\x90`\x80=\x81\x11a\x03\xD1W[a\x03\xC2\x81\x83a\x06\xD3V[\x81\x01\x90a\x13tV[\x92\x90a\x02SV[P=a\x03\xB8V[a\x03\xF8\x92\x93P\x80=\x10a\x04\0W[a\x03\xF0\x81\x83a\x06\xD3V[\x81\x01\x90a\x11CV[\x908\x80a\x02\x1BV[P=a\x03\xE6V[4a\x01,W` 6`\x03\x19\x01\x12a\x01,W`\x01`\x01`@\x1B\x03a\x04(a\x011V[\x16`\0R`\0` R`\xA0`@`\0 T`\xFF`@Q\x91`\x01`\x01`\x80\x1B\x03\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x80\x82`\x80\x1C\x16` \x85\x01R\x80\x82\x86\x1C\x16`@\x85\x01R\x81`\xC0\x1C\x16``\x84\x01R`\xE0\x1C\x16\x15\x15`\x80\x82\x01R\xF3[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\x04\x95a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x91\x90\x91\x16`\x04\x82\x01\x81\x90Ra\x01\0\x90\x81\x83`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x8CWa\x03J\x93a\x05\x1E\x93`\0\x93a\x05.W[PP`\0R`\0` Ra\x05\x18`@`\0 a\x10\xBEV[\x90a\x1F\xA0V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x05E\x92\x93P\x80=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x908\x80a\x05\x01V[\x90\x93\x92\x91\x93``\x82R\x80Q\x90\x81``\x84\x01R`\0[\x82\x81\x10a\x05\x8DWPP\x90`\x80\x82`\0\x82`@\x95\x85\x01\x01R`\x1F\x80\x19\x91\x01\x16\x82\x01\x01\x94` \x82\x01R\x01RV[\x80` \x80\x92\x84\x01\x01Q`\x80\x82\x87\x01\x01R\x01a\x05bV[4a\x01,W`\xA06`\x03\x19\x01\x12a\x01,Wa\x03J`d5a\x05\xC3\x81a\x01]V[a\x06sa\x05\xD1`\x045a3TV[a\x05\xDC`$5a3lV[\x92a\x05\xE8`D5a3lV[`\x01`\x01`\x80\x1B\x03`@Q\x93a\x05\xFD\x85a\x06\x98V[\x16\x94\x85\x84R` \x84\x01\x90c\xFF\xFF\xFF\xFF\x92\x83\x80\x92\x16\x83R\x81`@\x87\x01\x91\x16\x81R\x81``\x87\x01\x93`\0\x85R`\x80\x88\x01\x96\x15\x15\x87R`@Q\x99` \x8B\x01RQ\x16`@\x89\x01RQ\x16``\x87\x01RQ\x16`\x80\x85\x01RQ\x15\x15`\xA0\x84\x01R`\xA0\x83Ra\x06b\x83a\x06\xB8V[a\x06n`\x845\x91a\x1CsV[a\x19\xDCV[`@\x93\x91\x93Q\x93\x84\x93\x84a\x05MV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[a\x06\x82V[`\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[`@Q\x90a\x01\x85\x82a\x06\x98V[`@Q\x90a\x01\x85\x82a\x06\xB8V[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x06\xB3W`@Q\x91a\x077`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x06\xD3V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01,W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[4a\x01,W`@6`\x03\x19\x01\x12a\x01,W`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01,W6`#\x82\x01\x12\x15a\x01,Wa\x07\xAC\x90a\x06na\x07\xA7a\x07\xA2`\x045\x936\x90`$\x81`\x04\x015\x91\x01a\x07\x0EV[a\x1F\nV[a\x1CsV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x01,WV[4a\x01,W6`\x03\x19\x01`\xE0\x81\x12a\x01,W`\xA0\x13a\x01,Wa\x03Ja\x08Z`@Qa\x07\xFA\x81a\x06\x98V[`\x045a\x08\x06\x81a\x07\xBEV[\x81R`$5a\x08\x14\x81a\x07\xBEV[` \x82\x01R`D5a\x08%\x81a\x01]V[`@\x82\x01Ra\x082a\x01GV[``\x82\x01R`\x845a\x08C\x81a\x01]V[`\x80\x82\x01Ra\x08Pa\x01xV[\x90`\xA45\x90a\x15\0V[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x01,W`\x806`\x03\x19\x01\x12a\x01,Wa\x08\x92a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01Ra\x01\0\x91\x82\x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03\x8CWa\tV\x93`\0\x93a\tuW[PPa\tQa\x05\x18\x91a\t\x1Aa\t\r`D5a3TV[`\x01`\x01`\x80\x1B\x03\x16\x85RV[a\t8a\t(`d5a3TV[`\x01`\x01`\x80\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x10\xBEV[a\tb\x81`$5a\x13KV[`@\x80Q\x91\x15\x15\x82R` \x82\x01\x92\x90\x92R\xF3[a\x05\x18\x92\x93Pa\tQ\x91\x81a\t\x95\x92\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x92\x91a\x08\xF6V[4a\x01,W`@6`\x03\x19\x01\x12a\x01,Wa\t\xB5a\x011V[`$5`\x01`\x01`@\x1B\x03\x80\x82\x11a\x01,W6`#\x83\x01\x12\x15a\x01,W\x81`\x04\x015\x90\x81\x11a\x01,W6`$\x82\x84\x01\x01\x11a\x01,Wa\x03J\x92`$a\t\xFB\x93\x01\x90a\x0F7V[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\n&a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R\x90a\x01\0\x90\x81\x83`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03\x8CWa\x03J\x93a\x05\x1E\x93`\0\x93a\x0B\x11W[PPa\n\xB4a\tQa\x0B\x0B\x92`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91a\n\xC9a\n\xC1\x84a\x1CsV[\x93B\x90a\x1E\xA7V[`\x80\x84\x01Ra\x0B\x05a\n\xEA`@a\n\xF6a\n\xEA\x85Q`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x93\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a1FV[\x90a\x18\xFFV[a\x0B\x0B\x92\x93Pa\x0B4a\n\xB4\x92\x82a\tQ\x93\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x93\x92Pa\n\x8FV[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\x0BUa\x011V[P` `@Q`\x01\x81R\xF3[``\x90`\x03\x19\x01\x12a\x01,W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x01,W\x90`$5a\x0B\x8D\x81a\x01]V[\x90`D5a\x0B\x9A\x81a\x01gV[\x90V[4a\x01,Wa\x0B\xAB6a\x0BaV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0B\xF9Wa\x0B\xE5\x91a\x11\xFEV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`@Qc:#%k`\xE2\x1B\x81R`\x04\x90\xFD[4a\x01,Wa\x0C\x196a\x0BaV[Pa\x0C\"a\x14xV[P`@\x80Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01Ra\x01\0\x93`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x81`$\x81\x86Z\xFA\x95\x86\x15a\x03\x8CW`\0\x96a\x0F\x18W[PP\x82Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x82\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x92`\x80\x90\x84\x90\x81\x80`$\x81\x01\x03\x91Z\xFA\x80\x15a\x03\x8CWa\x03J\x96\x85\x94`\0\x92a\x0E\xF8W[Pa\x0C\xF9a\x0C\xEFa\x07\xA7a\tQ\x87`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[`@\x01Q\x90`\0\x90V[\x95\x90a\r\x03a\x06\xF4V[\x98`\0\x8ARa\r+\x86\x8B\x01\x97`\0\x89R`\0\x85\x8D\x01R``\x8C\x01\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x15\x80\x15`\x80\x8B\x01Ra\x0EyWPa\r\xF5``a\x0E\x01\x94a\x0E\x13a\x0E\x06a\x0E\x01\x87a\r\xFBa\r\xF5a\r\xEBa\x0E(\x9Fa\n\xEAa\x0E\x1B\x9F\x9Da\r\xE4\x8F\x93a\r\xFB\x9Fa\n\xEAa\r\xD0a\r\xBC\x97a\r\xC1a\r\xBC\x88a\r\xB6a\n\xEAa\r\xA8a\r\xA3a\r\xDE\x9A`\x01`\x01`\x80\x1B\x03\x9E\x01\x9E\x8FQ`\x01`\x01`\x80\x1B\x03\x16\x90V[a1\nV[\x92Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x14/V[a\x14\nV[\x9C\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a1%V[\x91\x16a\x14/V[\x9A\x87\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x90a\x14^V[a3TV[`\x01`\x01`\x80\x1B\x03\x16\x8CRV[\x01Q`\xFF\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90RV[Q\x91\x82\x91\x82\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[a\x0E\xF3\x96P\x84a\x0E\x01\x94a\x0E\x13a\x0E\x06a\x0E\x01a\x0E\xE1`\x01`\x01`\x80\x1B\x03a\x0E\xDBa\x0E\xCDa\r\xBC\x8Ca\x0E\x1B\x9Fa\n\xEAa\r\xB6\x91a\r\xFB\x9Fa\r\xF5\x9Fa\r\xDEa\n\xEAa\n\xF6\x93\x88\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16a\x14\nV[\x97a\r\xFBa\r\xF5``\x87\x01Q`\xFF\x16\x90V[a\x0E(V[a\x0F\x11\x91\x92P`\x80=\x81\x11a\x03\xD1Wa\x03\xC2\x81\x83a\x06\xD3V[\x908a\x0C\xC8V[a\x0F/\x92\x96P\x80=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x938\x80a\x0C\x86V[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x913\x83\x90\x03a\x0B\xF9Wa\x07\xA2a\x0F}\x91a\x0F\xEA\x936\x91a\x07\x0EV[a\x0F\x9A\x84`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x81Q`\x01`\x01`\x80\x1B\x03\x16` \x83\x01Qc\xFF\xFF\xFF\xFF\x16\x91`\x01`\x01`\x80\x1B\x03a\x0F\xD7`\x80a\x0F\xCF`@\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x96\x01Q\x15\x15\x90V[\x94c\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x92\x16\x90a\x1C\xFDV[\x7F\x8Di\xFAt\x9A\xF4\x15\xF9<\x96\x86>\xBF6\x92\x93&#\xC0\x10\xEB\xA8\x11\x8B\x02\xF0DXZ\xABe\x13`\x01`\x01`@\x1B\x03a\x103a\tQ\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x93a\x10\xB6a\x10H\x86Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95a\x10Z` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x10y`\x80a\x10q`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92\x01Q\x15\x15\x90V[\x91`@Q\x95\x86\x95\x16\x98\x85\x92\x91``\x92\x95\x94\x91\x95`\x01`\x01`\x80\x1B\x03`\x80\x86\x01\x97\x16\x85Rc\xFF\xFF\xFF\xFF\x80\x92\x16` \x86\x01R\x16`@\x84\x01R\x15\x15\x91\x01RV[\x03\x90\xA3`\0\x90V[\x90`@Qa\x10\xCB\x81a\x06\x98V[`\x80`\xFF\x82\x94T`\x01`\x01`\x80\x1B\x03\x81\x16\x84Rc\xFF\xFF\xFF\xFF\x80\x82\x85\x1C\x16` \x86\x01R\x80\x82`\xA0\x1C\x16`@\x86\x01R\x81`\xC0\x1C\x16``\x85\x01R`\xE0\x1C\x16\x15\x15\x91\x01RV[Q\x90a\x01\x85\x82a\x07\xBEV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01,WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\x01,WV[Q\x90a\x01\x85\x82a\x01gV[\x80\x91a\x01\0\x92\x83\x91\x03\x12a\x01,W`@Q\x91\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3Wa\x11\xEA\x91`\xE0\x91`@Ra\x11|\x81a\x11\rV[\x84Ra\x11\x8A` \x82\x01a\x11\rV[` \x85\x01Ra\x11\x9B`@\x82\x01a\x11\rV[`@\x85\x01Ra\x11\xAC``\x82\x01a\x11\x18V[``\x85\x01Ra\x11\xBD`\x80\x82\x01a\x11)V[`\x80\x85\x01Ra\x11\xCE`\xA0\x82\x01a\x11)V[`\xA0\x85\x01Ra\x11\xDF`\xC0\x82\x01a\x118V[`\xC0\x85\x01R\x01a\x118V[`\xE0\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01Ra\x01\0\x93\x92\x91\x90\x84\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x94\x85\x15a\x03\x8CW`\0\x95a\x12\xAAW[PP\x90a\x12\x94a\x12\x8Aa\tQa\x12\x9A\x94`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91B\x90\x83\x87a!NV[\x93a(hV[a\x12\xA4W`\x01\x91\x90V[`\0\x91\x90V[a\x12\x9A\x93\x92\x95Pa\tQa\x12\xD1a\x12\x94\x93\x83a\x12\x8A\x94\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x96\x93\x94PPa\x12aV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x13\nWV[a\x12\xDBV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x13\nWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x13\nWV[a\x13W\x90`\x01\x92a\x132V[\x12a\x13aW`\x01\x90V[`\0\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x01,WV[\x90\x81`\x80\x91\x03\x12a\x01,W`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3Wa\x13\xDA\x91``\x91`@R\x80Qa\x13\xAE\x81a\x01gV[\x84Ra\x13\xBC` \x82\x01a\x13fV[` \x85\x01R`@\x81\x01Qa\x13\xCF\x81a\x01gV[`@\x85\x01R\x01a\x13fV[``\x82\x01R\x90V[\x90\x81` \x91\x03\x12a\x01,WQ\x90V[\x90`\x12\x03`\x12\x81\x11a\x13\nWa\x14\x06\x90a\x14<V[\x02\x90V[`\0\x19\x81\x01\x91\x90\x82\x11a\x13\nWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x13\nWV[\x91\x90\x82\x03\x91\x82\x11a\x13\nWV[`M\x81\x11a\x13\nW`\n\n\x90V[`\x12\x03`\x12\x81\x11a\x13\nWa\x0B\x9A\x90a\x14<V[\x90`\x12\x03`\x12\x81\x11a\x13\nWa\x14s\x90a\x14<V[\x90\x04\x90V[`@Q\x90a\x14\x85\x82a\x06\x98V[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[``\x81\x01\x80Q`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R\x93\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x90\x91a\x01\0\x80\x87`$\x81\x88Z\xFA\x91\x82\x15a\x03\x8CWa\x15\xCB\x97`\0\x93a\x17\xCDW[Pa\x15\x9Fa\x15\x94a\x15\x88\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[` \x1Cb\xFF\xFF\xFF\x16\x90V[`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x80\x98\x90\x91\x90\x89\x90\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x8AZ\xFA\x90\x81\x15a\x03\x8CWa\x16\xDC` \x94a\x0E\x1B`\xA0`\x04\x9Da\x16\xD5a\x0E\x06a\x16\xCC\x8B\x8Ea\t8\x9Ba\x16\xE9\x9D`\0\x91a\x17\xB0W[Pa\x16\ta\x14\xA3V[\x80\x9Aa\x16\x17\x89\x85\x01Q\x15\x15\x90V[\x15a\x17TW\x82a\x16?a\x16\x85\x93`@\x93a\x167\x89a\x16\x95\x98\x01Q`\xFF\x16\x90V[`\xFF\x16\x91\x01RV[a\x16T\x8Da\x01 a\x167``\x85\x01Q`\xFF\x16\x90V[\x80Qa\x16w\x90\x8E\x90`\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x8B\x01RV[a\x16\xA9a\n\xEA\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x8A\x01R\x01\x96a\x16\xC3a\n\xEA\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x82\x01Ra\x17\xECV[\x92\x83\x01Qa3TV[\x01Qa3TV[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x94`@Q\x97\x88\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x93\x84\x15a\x03\x8CWa\x17 \x96`\0\x95a\x170W[Pa\x17\x1A\x90a\x10\xBEV[\x90a\"\x1BV[\x90\x91Pa\x17-\x81\x83a\x13KV[\x92V[a\x17\x1A\x91\x95Pa\x17M\x90` =\x81\x11a\x03\x85Wa\x03v\x81\x83a\x06\xD3V[\x94\x90a\x17\x10V[\x91a\x17na\x16\x85\x92a\x17\xAB\x94a\x167``\x85\x01Q`\xFF\x16\x90V[a\x17\x82\x8Ca\x01 a\x167\x88\x85\x01Q`\xFF\x16\x90V[`@\x81\x01Qa\x17\x9E\x90\x8D\x90`\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16a\x16iV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x16\x95V[a\x17\xC7\x91P\x87=\x89\x11a\x03\xD1Wa\x03\xC2\x81\x83a\x06\xD3V[8a\x16\0V[a\x17\xE5\x91\x93P\x82=\x84\x11a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x918a\x15pV[a\x17\xF4a\x14\xA3V[P`@\x81\x01\x80Q\x90a\x18\x10a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90a\x13\xF1V[\x90R``\x82\x01a\x18&\x81Q`\xFF\x84Q\x16\x90a\x13\xF1V[\x90Ra\x18=`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90a\x13\xF1V[\x90R`\xA0\x81\x01a\x18W\x81Q`\xFFa\x01 \x85\x01Q\x16\x90a\x13\xF1V[\x90R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x13\nW`\0\x19\x83\x05\x03a\x13\nWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x13\nW\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x13\nWV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x80\x83\x10\x15a\x19\x9AW\x82\x15a\x19\x8CWa\x19#`\x80\x83\x01Qa0\xE3V[\x90a\x19-\x83a\x19\xA4V[\x93\x81\x03\x90\x80\x82\x11a\x13\nWa\x19}`@\x93g\x1B\xC1mgN\xC8\0\0a\x19va\x19ba\x0B\x9A\x99a\x19]a\x19\x83\x98a*\x03V[a\x18\xB3V[\x92a\x19q``\x8A\x01Q\x80a\x18\xD6V[a\x18\xD6V[\x04\x90a\x132V[\x05a-\x86V[\x91\x01Q\x90a1%V[PPP`\x01`\x01`\x80\x1B\x03\x90V[P`@\x91P\x01Q\x90V[a\x19\xB9a\x19\xB4`\x80\x83\x01Qa0\xE3V[a2.V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x13\nW``a\x0B\x9A\x92\x01Qa1%V[`@\x81\x01Q`\0\x93\x92a\x19\xEF\x91\x90a1FV[\x80a\x1A\x02W[Pa\x0B\x9A\x90\x83\x81Ra\x1B\xF8V[a\x1A\x11a\x1A;\x91\x94\x92\x94a/BV[a\x1A[a\x1A!`\x80\x87\x01Qa0\xE3V[a\x1AU``\x88\x01Qg\x1B\xC1mgN\xC8\0\0\x95\x81\x87\x92a\x18\xD6V[\x04\x91a\x1AOa\x1AI\x8Aa\x19\xA4V[\x95a\x18\\V[\x92a\x18\xB3V[\x90a\x1B\xDCV[\x90\x80\x15a\x1A\xC9W`\x01`\xFF\x1B\x82\x14`\0\x19\x82\x14\x16a\x13\nWa\x1A\x96g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1A\x90a\x1A\x9B\x93a\x1A\xA0\x95\x05a\x18\\V[\x05a(\x94V[a(\xCDV[a\x18\\V[\x05g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x03\x92\x90\x91\x12\x80\x15\x82\x84\x13\x16\x91\x83\x12\x16\x17a\x13\nW\x91a\x0B\x9Aa\x19\xF5V[a\x18\xE9V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x13\nWV[`@\x81\x01\x80Q\x82Q\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x10\x15a\x1CiW\x15a\x1CbWPa\x1C\"\x83a\x19\xA4V[\x90\x83Q\x81\x03\x90\x81\x11a\x13\nWa\x0B\x9A\x93`\xA0a\x1CLa\x1CZ\x94a\x1CGa\x1CU\x95a*\x03V[a\x132V[\x91\x01Q\x90a\x1B\xDCV[a(\xA5V[\x90Q\x90a1%V[\x92PPP\x90V[PPPPP`\0\x90V[`\xA0`@Qa\x1C\x81\x81a\x06\xB8V[`\0\x91\x81\x83\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x01R`\x01`\x01`\x80\x1B\x03\x82Q\x16\x91c\xFF\xFF\xFF\xFF`@a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x83` \x86\x01Q\x16\x02\x04\x92\x01Q\x16\x90`@Q\x93a\x1C\xDD\x85a\x06\xB8V[\x83\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[\x92\x93\x91\x90\x93c\xFF\xFF\xFF\xFF\x92\x83a\x1D\x1B\x86Tc\xFF\xFF\xFF\xFF\x90`\xC0\x1C\x16\x90V[\x16a\x1E\x95W\x80\x15a\x1E4W\x84T`\xFF`\xE0\x1B\x19\x16\x90\x15\x15`\xE0\x1B`\xFF`\xE0\x1B\x16\x17\x84UP\x82Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16c\x01\xE1\x85Y`\xA0\x1B\x17\x83U[aa\xA8\x80\x82\x10\x90\x82\x14\x17`\x01\x82\x11`\x01\x83\x14\x17\x16\x15a\x1E\"Wa\x1D|a\x1D\x9C\x91a3lV[\x83Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x80\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\x80\x1B\x16\x17\x83UV[`\x01`\x01`\x80\x1B\x03\x80\x84\x10\x90\x84\x14\x17`\x01\x84\x11`\x01\x85\x14\x17\x16\x15a\x1E\x10Wa\x1D\xEEa\x1D\xC9a\x01\x85\x94a3TV[\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x83UV[\x81Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16B\x91\x90\x91\x16`\xC0\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x17\x90UV[`@Qc\xB2B\xE3A`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9]8\x19`\xE0\x1B\x81R`\x04\x90\xFD[Pb\x01Q\x80\x81\x81\x14\x90\x82\x11\x17c\x05\xA4\x90\x0B\x82\x81\x14\x90\x83\x10\x17\x16\x15a\x1E\x83Wa\x1E^a\x1E~\x91a3lV[\x84Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x84UV[a\x1DWV[`@Qc\xB5\x97\x03\x0F`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x1E\x13\x89\xA1`\xE2\x1B\x81R`\x04\x90\xFD[`\x80\x81\x01Qa\x1E\xCCWa\x1E\xBEc\xFF\xFF\xFF\xFF\x91a\x1E\xD6V[\x16\x80\x82\x10\x90\x82\x03\x02\x81\x03\x03\x90V[PPc\x01\xE1\x85Y\x90V[`\x80\x81\x01Qa\x1E\xF8Wc\xFF\xFF\xFF\xFF\x90\x81`@\x81``\x84\x01Q\x16\x92\x01Q\x16\x01\x16\x90V[`@Qc\xB0\x19\x84\x97`\xE0\x1B\x81R`\x04\x90\xFD[a\x1F\x12a\x14xV[P`\xA0\x81Q\x03a\x1F\x8EW`\xA0\x81\x80Q\x81\x01\x03\x12a\x01,W`\xA0`@Q\x91a\x1F8\x83a\x06\x98V[` \x81\x01Qa\x1FF\x81a\x07\xBEV[\x83Ra\x1FT`@\x82\x01a\x11\x18V[` \x84\x01Ra\x1Fe``\x82\x01a\x11\x18V[`@\x84\x01Ra\x1Fv`\x80\x82\x01a\x11\x18V[``\x84\x01R\x01Qa\x1F\x86\x81a\x01]V[`\x80\x82\x01R\x90V[`@Qc\x01YW\xD3`\xE2\x1B\x81R`\x04\x90\xFD[a\x0B\x9A\x91a\x1F\xB8a\n\xEA\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91`\x01`\x01`\x80\x1B\x03a \x02a\x1F\xE5a\x1F\xDEa\n\xEA`@\x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x96a1FV[\x94a\x1F\xFDa\n\xEA` \x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a1FV[\x92a ga \x17\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a ac\xFF\xFF\xFF\xFF``a Ta Ba 9` \x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[g\r\xE0\xB6\xB3\xA7d\0\0a'\x10\x91\x02\x04\x90V[\x97\x01Q\x16c\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x1E\xA7V[\x93a pa\x07\x01V[\x95\x86R` \x86\x01R\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\0`\xA0\x82\x01R[a\x0B\x9A\x90a \xF4a \xA0\x82a\x19\xA4V[\x91a \xAF\x81`@\x01Q\x90`\0\x90V[\x82Q\x90\x92\x91\x90g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a!*WP`\x01\x92[` \x83\x01Q\x91\x82\x10a \xF9WPPPa\x1CGa \xEEg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92[a*\x03V[\x91a*\x03V[a\x1B\xDCV[\x81\x11a!\x0FWPPa\x1CGa \xEE`\x01\x92a*\x03V[a!$a \xEE\x91`@a\x1CG\x94\x01Q\x90a1\xFEV[\x92a*\x03V[\x80a!?WPg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92a \xC9V[a!H\x90a\x14\x19V[\x92a \xC9V[\x90`\x01`\x01`\x80\x1B\x03a\x0B\x9A\x94\x92a!pa\n\xEA\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a!\x87a\n\xEA` \x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95\x15a!\xEBWa!\xB1a!\xAAa\n\xEA`@a!\xB7\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a1FV[\x95a1\xFEV[\x92[a ga!\xCD\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a!\xE5a Ba 9` \x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x94a\x1E\xA7V[a\"\x0Fa\"\x08a\n\xEA`@a\"\x15\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a1\xFEV[\x95a1FV[\x92a!\xB9V[a\x0B\x05a\n\xEAa#3\x93a#$`@a\"\xA9a\"D\x9A\x99\x98\x99a\"K\x8A`\x80\x8D\x01\x9D\x8EQ\x15\x15\x90V[\x87\x8Ba!NV[\x9Ca\"]\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[` \x8A\x01Q`\xC0\x8B\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x03a#XW`\xA0\x8A\x01Qa\xFF\xFF\x16\x9C[`\x01`\x01`\x80\x1B\x03\x9Da\xFF\xFF\x16\x92\x8E\x16\x91\x8E\x16\x90a#gV[\x98\x92P\x94\x90Pa\"\xE1a\"\xC3\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9Aa\"\xDBa Ba 9` \x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92a\x1E\xA7V[\x90a\"\xEAa\x07\x01V[\x9B`\0\x8DR` \x8D\x01\x9B`\0\x8DR\x16\x84\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\0`\xA0\x8B\x01R\x01\x91a\x0B\x05a\n\xEA\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87RQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81Ra#Ha#A\x83a \x90V[\x93Q\x15\x15\x90V[\x15a#RWPQ\x92V[\x90PQ\x92V[`\x80\x8A\x01Qa\xFF\xFF\x16\x9Ca\"\x90V[\x91\x94\x92\x94`\0a#\x82`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16a1\xB0V[\x95\x80\x15\x80\x15a$\xB5W[PP\x80\x95\x80\x97`\x80\x86\x01\x92a#\xD8a#\xA4\x85Q\x15\x15\x90V[\x93\x84\x15a$\xAEW\x87\x94[\x15a$\xA4Wa#\xD3\x87\x95[a#\xCDa\n\xEA\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a$\xD1V[a\x14/V[\x90\x81\x81\x11a$\x92Wa#\xED` \x91\x85\x93a\x14/V[\x97\x01\x91a$\x01\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11a$\x80Wa$)\x91a\r\xB6a\n\xEAa$\"\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91Q\x15\x15\x90V[\x90\x81\x15a$wW\x84\x85\x92[\x15a$oWP\x92[\x14a$]W\x81\x14a$KW\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92a$<V[\x80\x94\x85\x92a$4V[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[a#\xD3\x88\x95a#\xB9V[\x86\x94a#\xAEV[\x90\x91Pa\x1A\xC9W\x85\x04\x94\x85\x81\x03\x90\x81\x11a\x13\nW\x948\x80a#\x8CV[\x91\x90\x82\x01\x80\x92\x11a\x13\nWV[`\0\x19\x81\x14a\x13\nW`\x01\x01\x90V[\x93\x91a%\x1Ea%\x06`\x80\x93\x97a%)\x95\x87\x85\x8B\x8Ba\"\x1BV[P\x95\x90\x97a%\x13\x81a\x1CsV[\x96`\xA0\x88\x01Ra\x1E\xA7V[\x82\x85\x01R\x01Q\x15\x15\x90V[\x92\x83\x15a&[W\x81Ra%;\x81a\x1B\xF8V[\x80` \x83\x01R[\x80\x15a&*W\x91a%\xF9a%\xE2a%\xDDa%\xC1`\x01`\x01`\x80\x1B\x03\x95\x87a%\xCFa%xa%ra\x0B\x9A\x9C\x9Ba1hV[\x92a1\xD6V[\x92`@Q\x94\x85\x91` \x83\x01\x91\x90\x91`\xA0\x80`\xC0\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x85R\x84a\x06\xD3V[\x88\x15a&\"W`\x02\x92a&qV[a$\xDEV[a\r\xDEa\n\xEA`@\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x92\x15a&\x13W` \x01Q`\x01`\x01`\x80\x1B\x03\x16[\x16a\x14/V[Q`\x01`\x01`\x80\x1B\x03\x16a&\rV[`\x01\x92a&qV[PP`\x01`\x01`\x80\x1B\x03\x91`\0\x14a&MW` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[` \x82\x01Ra&i\x81a'\xA0V[\x80\x82Ra%BV[\x91\x90\x93\x92\x93`\0\x94`\0\x92\x80\x83\x11a'\x7FWa&\x8E\x83\x86\x84a3\x98V[\x90a&\x9A\x81\x87\x85a3\x98V[\x85a&\xA5\x82\x85a\x18\xB3V[\x13a'^WP\x95\x94\x93a&\xB8\x84\x88a\x14/V[\x94\x81`\x01\x96\x87\x80[a&\xD2W[PPPPPPPPP\x90PV[\x15a'9W[P\x86\x97\x98\x99P\x80\x92a&\xF3a&\xED\x8C\x89a$\xD1V[`\x01\x1C\x90V[\x9A\x8B\x90a'\x01\x8D\x86\x8Aa3\x98V[\x90\x84a'\r\x89\x84a\x18\xB3V[\x13a'+WPP\x93[\x88a'!\x89\x87a\x14/V[\x92\x01\x94\x9A\x99a&\xC0V[\x95\x96P\x97PP\x8A\x96\x94a'\x16V[\x87\x10\x80a'SW[\x15a'LW\x89a&\xD8V[\x80\x80a&\xC5V[Pa\x01\0\x83\x10a'AV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@\x81\x01\x80Q\x91` \x81\x01\x92\x83Q\x90\x81\x10\x15a(\x0FW\x15a(\0W`\xA0a'\xE1a'\xEA\x94a \xF4a \xE9a\x1CU\x96a'\xD7\x87a\x19\xA4V[\x93Q\x90Q\x90a1\xFEV[\x91\x01Q\x90a\x132V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x13\nW\x90V[PPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[PPPP`\0\x90V[\x90\x81`\xC0\x91\x03\x12a\x01,W`\xA0`@Q\x91a(2\x83a\x06\xB8V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R\x01Q`\xA0\x82\x01R\x90V[`\x80\x82\x01Qa(\x8DW``\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91a(\x87\x90a\x1E\xD6V[\x16\x11\x15\x90V[PP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\x13\nW`\0\x03\x90V[a(\xC9a\x1A\x9Ba\x1A\x96g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1A\x90g\x1B\xC1mgN\xC8\0\0\x95a\x18\\V[\x05\x90V[\x80\x15a)\xF6WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a)\xF0WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a)\xE3Wa)\xD1a)\0\x82a+\xAEV[a)\x94a)\xCCa)\x1Fa)\x1Aa)\x15\x85a1\x85V[a\x1A\xCEV[a-EV[\x92a \xF4a)\xC7a)\xC2a)\xBBa)\xB5a)\xB0a)\xAAa)\xA5a)\x9Fa)\x9A\x8Da)\x94a)\x8Fa)\x89a)\x84a)~a)ya)sa)na)ha)c\x8Aa+\xDBV[a\x1A\xE6V[\x89a,}V[a\x1B\0V[\x87a,}V[a\x1B\x18V[\x85a,}V[a\x1B2V[\x83a,}V[a\x1BJV[\x90a,}V[a\x1BdV[\x8Ca,}V[a\x1B|V[\x8Aa,}V[a\x1B\x94V[\x88a,}V[\x93\x80a,}V[a\x18yV[a\x12\xF1V[a-\x86V[\x90`\0\x13\x15a\x0B\x9AWa\x0B\x9A\x90a\x13\x0FV[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a)\xF0Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a+XW\x81\x15a+yW`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x13\nW`\0\x83\x12\x80\x15a+\x9DW[a+\x8BW\x82\x15a+XWg\x1B\xC1mgN\xC8\0\0\x83\x14a+yW\x82\x12\x91\x82\x15a+jW\x92[a*r\x84a,\xC5V[\x80\x15a+XWa*\xCFa*\x9Ea*\x99a\x19\xB4a*\x94a*\xD4\x95\x99\x97\x96\x99a/BV[a,\x06V[a\x18\x9AV[a\x1CGa*\xB2a*\xAD\x83a,\xF0V[a\x1B\xACV[a*\xC9a)\x15a)~a*\xC4\x86a-\x1BV[a\x1B\xC4V[\x90a-dV[a,.V[\x93`\0\x92[\x81\x84\x10a+\x0BWPPPPa\x0B\x9A\x91a*\xF8\x91`\0\x14a*\xFDWa,\x9EV[a(\x94V[a+\x06\x90a(\x94V[a,\x9EV[\x90\x91a+N\x86a\x1AUa+#\x85a\x1CG\x86\x99\x9Ba(\xCDV[a*\xC9a+>a+9a)\xCCa*\xF8\x87\x80a,}V[a,VV[a+H\x83\x86a,}V[\x90a\x132V[\x95\x01\x92\x91\x90a*\xD9V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a+s\x90a\x13\x0FV[\x92a*iV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a*EV[`\x01`\xFF\x1B\x81\x14a+\xC9W`\0\x81\x12\x15a\x0B\x9AW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x01,Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01,W\x05\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a)\xF0Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a.\xD6We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x15a/\x11WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a/n`\0\x82\x13a/\nV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a/\x8A\x82a2\xECV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wc\x01\xE1\x85Y\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x02\x04\x90\x81\x14`\x01\x16\x15a\x01,W\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01,W\x04\x90V[`2\x81\x02\x90`2\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`d\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x96\x81\x02\x90`\x96\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01,W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a2\xD5W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a2\xC8W[e\x01\0\0\0\0\0\x81\x10\x15a2\xBBW[c\x01\0\0\0\x81\x10\x15a2\xAEW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a2rV[` \x1C\x91`\x10\x1B\x91a2eV[`@\x1C\x91` \x1B\x91a2VV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca2>V[a2\xF7\x81\x15\x15a/\nV[\x80`\x01`\x01`\x80\x1B\x03\x10`\x07\x1B\x81\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\x80\x1B\x81\x10\x15a\x01,W`\x01`\x01`\x80\x1B\x03\x16\x90V[d\x01\0\0\0\0\x81\x10\x15a\x01,Wc\xFF\xFF\xFF\xFF\x16\x90V[cNH{q`\xE0\x1B`\0R`Q`\x04R`$`\0\xFD[\x80`\x02\x14a3\xEEW`\x01\x03a3\x82W\x80` \x80a3\xBA\x93Q\x83\x01\x01\x91\x01a(\x18V[\x90\x81R`\xA0a3\xC8\x82a \x90V[\x91\x01Q`\x01\x81\x01\x90`\0`\x01\x83\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x13\nWa\x0B\x9A\x91a\x132V[P\x80` \x80a4\x02\x93Q\x83\x01\x01\x91\x01a(\x18V[\x90` \x82\x01R`\xA0a3\xC8\x82a \x90V\xFE\xA2dipfsX\"\x12 \x15\xAE\xFC\xB37\x02\xB8X\xED\x0B\x82[\x1F\x81\xE0\x0F#}\x05\xEE\x7FD\xDBK\xF1\x10\xB9\xF9n\x8BT\x19dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static PORTFOLIO_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0#W[6\x15a\0\x19W`\0\x80\xFD[a\0!a0\xF9V[\0[`\x005`\xE0\x1C\x80b\xFD\xD5\x8E\x14a\x02\xA2W\x80c\x01\xFF\xC9\xA7\x14a\x02\x9DW\x80c\x06C;\x1B\x14a\x02\x98W\x80c\x07\x88\x88\xD6\x14a\x02\x93W\x80c\x0E\x894\x1C\x14a\x02\x8EW\x80c\x19\x05x\x07\x14a\x02\x89W\x80c&z\x0C\xFE\x14a\x02\x84W\x80c*\xFB\x9D\xF8\x14a\x02\x7FW\x80c.\xB2\xC2\xD6\x14a\x02zW\x80c/\x9E8\xE2\x14a\x02uW\x80c0$K\xE7\x14a\x02pW\x80c9CMZ\x14a\x02kW\x80c?\x92\xA39\x14a\x02fW\x80cM\xC6\x8A\x90\x14a\x02aW\x80cN\x12s\xF4\x14a\x02\\W\x80cS\x1E\x17\xB3\x14a\x02WW\x80c[\xC5Td\x14a\x02RW\x80c^Gf<\x14a\x02MW\x80cx}\xCE=\x14a\x02HW\x80c\x80\xAF\x9Dv\x14a\x02CW\x80c\x89\x92\xF2\n\x14a\x02>W\x80c\x89\xA5\xF0\x84\x14a\x029W\x80c\x8Ag\x89g\x14a\x024W\x80c\xA2,\xB4e\x14a\x02/W\x80c\xA5\xCD\x8AI\x14a\x02*W\x80c\xAC\x96P\xD8\x14a\x02%W\x80c\xAD\\FH\x14a\x02 W\x80c\xB0\xC3\xA9P\x14a\x02\x1BW\x80c\xB0\xE2\x1E\x8A\x14a\x02\x16W\x80c\xC9\xA3\x96\xE9\x14a\x02\x11W\x80c\xC9\xC6S\x96\x14a\x02\x0CW\x80c\xD6\xB7\xDE\xC5\x14a\x02\x07W\x80c\xDC\xF8D\xA7\x14a\x02\x02W\x80c\xDD\xA4\x07\x97\x14a\x01\xFDW\x80c\xE31\xBA4\x14a\x01\xF8W\x80c\xE9\x85\xE9\xC5\x14a\x01\xF3W\x80c\xF0{\x87\x9E\x14a\x01\xEEW\x80c\xF2BC*\x14a\x01\xE9W\x80c\xF3:\xE1\xBC\x14a\x01\xE4Wc\xFF\xA1\xADt\x03a\0\x0EWa0\xCDV[a*\x05V[a(\x84V[a'bV[a&\xC3V[a&RV[a$\x8FV[a$RV[a#vV[a \xA7V[a jV[a LV[a \x07V[a\x1F\xC2V[a\x1E\xB0V[a\x1E\x12V[a\x1D}V[a\x1B\xDCV[a\x1B\x02V[a\x19\xF1V[a\x18\xFDV[a\x16\xA2V[a\x16?V[a\x12\xDAV[a\x12\x95V[a\x11\x9DV[a\x111V[a\x10\xE7V[a\x10QV[a\x0F\xF6V[a\x0B\x17V[a\x08\x95V[a\x07lV[a\x06\xD1V[a\x05oV[a\x04LV[a\x03\xCDV[a\x03\x88V[a\x03\x1AV[a\x02\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\xB8WV[`\0\x80\xFD[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x02\xDA\x81a\x02\xA7V[`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 `$5`\0R` R` `@`\0 T`@Q\x90\x81R\xF3[`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x03a\x02\xB8WV[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W` `\x045a\x039\x81a\x03\x08V[c\xFF\xFF\xFF\xFF`\xE0\x1B\x16c\x01\xFF\xC9\xA7`\xE0\x1B\x81\x14\x90\x81\x15a\x03wW[\x81\x15a\x03fW[P`@Q\x90\x15\x15\x81R\xF3[c\x03\xA2M\x07`\xE2\x1B\x14\x90P8a\x03[V[cl\xDB=\x13`\xE1\x1B\x81\x14\x91Pa\x03TV[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W` b\xFF\xFF\xFF`\x06T\x16`@Q\x90\x81R\xF3[`\0[\x83\x81\x10a\x04\x03WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xF3V[\x90` \x91a\x04,\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xF0V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x04I\x92\x81\x81R\x01\x90a\x04\x13V[\x90V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`@Qc\x03\xA2M\x07`\xE2\x1B\x81R`\x04\x805\x90\x82\x01R`\0\x90\x81\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05OW\x82\x91a\x04\xC6W[`@Q\x80a\x04\xC2\x84\x82a\x048V[\x03\x90\xF3[\x90P=\x80\x83\x83>a\x04\xD7\x81\x83a\x18RV[\x81\x01\x90` \x81\x83\x03\x12a\x05GW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x05KW\x01\x81`\x1F\x82\x01\x12\x15a\x05GW\x80Q\x92a\x05\r\x84a1+V[\x92a\x05\x1B`@Q\x94\x85a\x18RV[\x84\x84R` \x85\x84\x01\x01\x11a\x05DWPa\x04\xC2\x92a\x05>\x91` \x80\x85\x01\x91\x01a\x03\xF0V[8a\x04\xB4V[\x80\xFD[\x82\x80\xFD[\x83\x80\xFD[a1FV[`\x01`\x01`@\x1B\x03\x81\x16\x03a\x02\xB8WV[\x80\x15\x15\x03a\x02\xB8WV[4a\x02\xB8W`\x806`\x03\x19\x01\x12a\x02\xB8Wa\x06\x1B` `\x045a\x05\x91\x81a\x05TV[`$5a\x05\x9D\x81a\x05eV[`d5\x91a\x05\xAA\x83a\x02\xA7V[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n\x85R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16`@Qc\x19\x05x\x07`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x92\x16`\x04\x83\x01R\x91\x15\x15`$\x82\x01R`D\x805\x90\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`d\x83\x01R\x90\x92\x83\x91\x90\x82\x90\x81\x90`\x84\x82\x01\x90V[\x03\x91Z\xFA\x80\x15a\x05OWa\x04\xC2\x91`\0\x91a\x06BW[P`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x06c\x91P` =\x81\x11a\x06iW[a\x06[\x81\x83a\x18RV[\x81\x01\x90aC\xACV[8a\x061V[P=a\x06QV[`\x045\x90b\xFF\xFF\xFF\x82\x16\x82\x03a\x02\xB8WV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x02\xB8WV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x02\xB8WV[\x91\x81`\x1F\x84\x01\x12\x15a\x02\xB8W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB8W` \x83\x81\x86\x01\x95\x01\x01\x11a\x02\xB8WV[a\x01\x006`\x03\x19\x01\x12a\x02\xB8Wa\x06\xE6a\x06pV[a\xFF\xFF\x90`d5\x82\x81\x16\x81\x03a\x02\xB8W`\x845\x92\x83\x16\x83\x03a\x02\xB8W`\xA45\x91a\x07\x0F\x83a\x02\xA7V[`\xC45a\x07\x1B\x81a\x02\xA7V[`\xE45\x93`\x01`\x01`@\x1B\x03\x85\x11a\x02\xB8Wa\x04\xC2\x95a\x07Ba\x07R\x966\x90`\x04\x01a\x06\xA4V[\x95\x90\x94`D5\x90`$5\x90a9\x14V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x07\x89\x81a\x05TV[`\x01`\x01`@\x1B\x03\x81\x16`\0R`\n` Ra\x07\xA8`@`\0 a3\x9DV[`@\x81\x01\x90`\x01`\x01`\x80\x1B\x03`\x01`\x01`\x7F\x1B\x03\x81\x84Q\x16\x11a\x08SWa\x081a\x08+``a\x088a\x08\x18a\x08\x13a\x07\xF7b\xFF\xFF\xFF\x98a\x07\xF1\x89a\x08A\x9CQ\x16`\x0F\x0Ba4YV[\x90aM\x8BV[\x98\x90\x9A` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[a3YV[\x97\x85a\x081a\x08+` \x8C\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x91\x16aH#V[\x96\x01Q`\xFF\x16\x90V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`@Qc\xAC\xC9@{`\xE0\x1B\x81R`\x04\x90\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x02\xB8W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB8W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x02\xB8WV[4a\x02\xB8W`\xA06`\x03\x19\x01\x12a\x02\xB8W`\x045a\x08\xB2\x81a\x02\xA7V[`$5\x90a\x08\xBF\x82a\x02\xA7V[`\x01`\x01`@\x1B\x03\x90`D5\x82\x81\x11a\x02\xB8Wa\x08\xE0\x906\x90`\x04\x01a\x08eV[\x92\x90\x93`d5\x82\x81\x11a\x02\xB8Wa\x08\xFB\x906\x90`\x04\x01a\x08eV[\x94\x90\x92`\x845\x90\x81\x11a\x02\xB8Wa\t\x16\x906\x90`\x04\x01a\x06\xA4V[\x90a\t\"\x87\x84\x14aRkV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x94\x903\x86\x14\x80\x15a\n\xBAW[a\tB\x90aQgV[\x84`\0[\x89\x89\x8D\x8D\x85\x85\x10a\n.WPPPPPP\x81\x16\x80\x95\x8A\x7FJ9\xDC\x06\xD4\xC0\xDB\xC6Kp\xAF\x90\xFDi\x8A#:Q\x8A\xA5\xD0~Y]\x98;\x8C\x05&\xC8\xF7\xFBa\t\x91\x8C`@Q\x91\x82\x91\x8D\x8C3\x97\x85aR\xDDV[\x03\x90\xA4;\x15a\n\x1BW\x96a\t\xC6`\0\x92` \x97\x98\x99`@Q\x9A\x8B\x98\x89\x97\x88\x96c\xBC\x19|\x81`\xE0\x1B\x9D\x8E\x89R3`\x04\x8A\x01aS\x04V[\x03\x92Z\xF1\x80\x15a\x05OWa\0!\x92`\0\x91a\t\xEDW[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x14aR,V[a\n\x0E\x91P` =\x81\x11a\n\x14W[a\n\x06\x81\x83a\x18RV[\x81\x01\x90aQ\xE4V[8a\t\xDCV[P=a\t\xFCV[PPP\x92PPPa\0!\x91P\x15\x15aQ\xA4V[a\n~a\n\xB0\x93a\noa\nU\x88a\nL\x81`\x01\x9Ca\n\xA8\x99aR\xA9V[5\x95\x86\x94aR\xA9V[5\x96`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\0R` R`@`\0 \x90V[a\n\x89\x85\x82Ta8\x03V[\x90U`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R` \x81\x90R`@\x90 a\noV[\x91\x82Ta8\x10V[\x90U\x01\x85\x90a\tFV[Pa\tBa\n\xFFa\n\xF83a\n\xE1\x8C`\x01\x80`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[T`\xFF\x16\x90V[\x90Pa\t9V[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x02\xB8WV[`\xC06`\x03\x19\x01\x12a\x02\xB8W`\x04\x805\x90a\x0B1\x82a\x05eV[`$5a\x0B=\x81a\x02\xA7V[`D5\x90a\x0BJ\x82a\x05TV[`d5\x90a\x0BW\x82a\x0B\x06V[`\x845\x91a\x0Bd\x83a\x0B\x06V[`\xA45\x93a\x0Bq\x85a\x0B\x06V[\x90\x81a\x0B{a2\xB9V[`\x0FT`\xFF\x16\x15a\x0F\xE9W[`\x01`\x01`@\x1B\x03\x80\x91\x16\x15a\x0F\xD1W[`\x01\x80`\xA0\x1B\x03\x97\x88a\x0B\xD1`\x03a\x0B\xC3\x87`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x16\x15a\x0F\xABW\x83a\x0C\x0Ca\x0C\0a\x0C\0`\x03a\x0B\xC3\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x96`@\x97\x8A\x89Q\x80\x92c\xE6\x04{\x19`\xE0\x1B\x82R\x81\x80a\x0C@` \x98\x89\x96\x83\x01\x91\x90\x91`\x01`\x01`@\x1B\x03` \x82\x01\x93\x16\x90RV[\x03\x91Z\xFA\x90\x81\x15a\x05OW`\0\x91a\x0F~W[P\x15a\x0FUWa\x0Cq\x90`\x01`\x01`\x80\x1B\x03\x80\x80\x9B\x16\x91\x16\x87a=yV[a\x0C\x96a\x08\x13b\xFF\xFF\xFF\x89\x86\x95\x96\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93a\x0E\xC4W[\x89\x86\x16\x15a\x0E\xB4W\x89\x90\x81\x80a\x0C\xD9a\x0C\xD0a\x0C\xCB\x8C`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[a3\x9DV[a\x07\xF1\x8BaG\xD7V[\x9D\x90\x9D\x16\x9C\x16\x94\x16\x84\x11a\x0E\xA4W\x16\x89\x11a\x0E\x94W\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x83\x89\x01\x80Q\x90\x98\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90a\r\x15\x88aG\xD7V[\x92a\r\x1Ea\x18sV[\x93B\x85R\x86\x86\x86\x01R\x8D\x8D\x86\x01R``\x85\x01\x90a\r=\x91\x90`\x0F\x0B\x90RV[`\x01`\x01`@\x1B\x03\x8A\x16`\x80\x85\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\r\x7F\x90a4\xDDV[\x82\x01Q`\xFF\x16`\xFF\x16a\r\x91\x91aH#V[\x96``\x82\x01Qa\r\xA1\x90`\xFF\x16\x90V[`\xFF\x16a\r\xAD\x91aH#V[\x97\x87\x15\x80a\x0E\x8CW[a\x0E~WP\x97\x7F\xFD\xFF\xEC\xA7Q\xF0\xDC\xAA\xB7U1\xCB\x81<\x12\xBB\xFDV\xEE>\x96L\xC4q\xD7\xEFC\x93$\x02\xEE\x18\x91a\x0E6a\x0E\x07a\r\xF9a\x04\xC2\x9A\x9B\x9CQ`\x01\x80`\xA0\x1B\x03\x16\x90V[\x97Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x88Q\x8B\x81R` \x81\x01\x8D\x90R`\x01`\x01`\x80\x1B\x03\x90\x96\x16`@\x87\x01R\x83\x16\x96\x90\x92\x16\x94\x16\x92\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA4a\x0EMa\x0EH`\x0FT`\xFF\x16\x90V[\x15\x15\x90V[\x15a\x0EqW[a\x0E[a3\x02V[Q\x91\x82R` \x82\x01\x92\x90\x92R\x90\x81\x90`@\x82\x01\x90V[a\x0EyaACV[a\x0ESV[\x86Qce\x8B\x16\xED`\xE0\x1B\x81R\xFD[P\x88\x15a\r\xB6V[\x87Qc!0\x16\x97`\xE2\x1B\x81R\x8A\x90\xFD[\x89QcVr\x0E\x1D`\xE1\x1B\x81R\x8C\x90\xFD[\x88Qc\x90`\x9A}`\xE0\x1B\x81R\x8B\x90\xFD[\x83Q\x90\x95Pa\x0F>\x90a\x0F\x12\x90a\x0E\xE3\x90`\x01`\x01`\xA0\x1B\x03\x16a1RV[\x85\x8B\x01Qa\x0E\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16a1RV[\x90`\0\x81\x12a\x0FMW[`\0\x82\x12a\x0FDW[\x89a=yV[\x8B\x80a\x0F4a\x0C\xCB\x8C`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[\x92\x16\x92\x16\x90aLdV[\x94a\x0C\x9CV[`\0\x91Pa\x0F\x0CV[P`\0a\x0F\x03V[\x87Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x87\x16\x81\x8C\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[\x03\x90\xFD[a\x0F\x9E\x91P\x83=\x85\x11a\x0F\xA4W[a\x0F\x96\x81\x83a\x18RV[\x81\x01\x90a3DV[8a\x0CSV[P=a\x0F\x8CV[`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16\x81\x8A\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[`\x0FT\x90\x92P`\x08\x1C`\x01`\x01`@\x1B\x03\x16\x91a\x0B\x98V[a\x0F\xF1a;\xBFV[a\x0B\x87V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W` a\x10?`\x045a\x10\x18\x81a\x05TV[`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wa\x06\x1B` `\x045a\x10s\x81a\x05TV[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n\x83R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16`@Q\x80\x80\x95\x81\x94c\x1C\xA1\xA6\xAD`\xE1\x1B\x83R`\x04\x83\x01\x91\x90\x91`\x01`\x01`@\x1B\x03` \x82\x01\x93\x16\x90RV[`@\x90`\x03\x19\x01\x12a\x02\xB8W`\x045a\x10\xDA\x81a\x02\xA7V[\x90`$5a\x04I\x81a\x02\xA7V[4a\x02\xB8W` b\xFF\xFF\xFFa\x11'a\x10\xFE6a\x10\xC2V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x0B\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x81R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W` a\x11Z`\x045a\x11S\x81a\x02\xA7V[0\x90aGYV[`@Q\x90\x81R\xF3[` \x90\x81`@\x81\x83\x01\x92\x82\x81R\x85Q\x80\x94R\x01\x93\x01\x91`\0[\x82\x81\x10a\x11\x89WPPPP\x90V[\x83Q\x85R\x93\x81\x01\x93\x92\x81\x01\x92`\x01\x01a\x11{V[4a\x02\xB8W`@\x80`\x03\x196\x01\x12a\x02\xB8W`\x01`\x01`@\x1B\x03\x90`\x045\x82\x81\x11a\x02\xB8Wa\x11\xD0\x906\x90`\x04\x01a\x08eV[\x91\x90\x92`$5\x90\x81\x11a\x02\xB8Wa\x11\xEB\x906\x90`\x04\x01a\x08eV[\x93\x90a\x11\xF8\x85\x85\x14aRkV[a\x12\x01\x84a1]V[\x93a\x12\x0E\x84Q\x95\x86a\x18RV[\x80\x85R`\x1F\x19a\x12\x1D\x82a1]V[\x01` \x906\x82\x88\x017`\0\x91\x82[\x81\x81\x10a\x12?W\x86Q\x80a\x04\xC2\x8A\x82a\x11bV[\x80a\x12M`\x01\x92\x84\x89aR\xA9V[5a\x12W\x81a\x02\xA7V[\x82\x80`\xA0\x1B\x03\x16\x85R\x84\x84Ra\x12\x83\x88\x86 a\x12t\x83\x8D\x8AaR\xA9V[5`\0R` R`@`\0 \x90V[Ta\x12\x8E\x82\x8Ba2\xA5V[R\x01a\x12+V[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\xA06`\x03\x19\x01\x12a\x02\xB8W`\x04\x805a\x12\xF3\x81a\x05eV[`$5\x91a\x13\0\x83a\x05TV[`D5\x92a\x13\r\x84a\x0B\x06V[`d5\x90a\x13\x1A\x82a\x0B\x06V[`\x845\x94a\x13'\x86a\x0B\x06V[\x90a\x130a2\xB9V[`\x0FT`\xFF\x16\x15a\x162W[a\x13da\x0C\0a\x0C\0`\x03a\x0B\xC3\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`@\x80Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16\x87\x82\x01\x90\x81R\x91\x95` \x95\x93\x90\x92\x86\x91\x83\x91\x82\x90\x81\x90\x85\x01\x03\x91Z\xFA\x90\x81\x15a\x05OW`\0\x91a\x16\x15W[P\x15a\x15\xF0Wa\x13\xC5\x90`\x01`\x01`\x80\x1B\x03\x80\x80\x9A\x16\x91\x16\x84a=yV[\x93a\x13\xE9a\x08\x13b\xFF\xFF\xFF\x86\x84\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x80Q\x90\x97\x90`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x01Q\x90\x96\x90`\x01`\x01`\xA0\x1B\x03\x16\x99a\x15\xA7W[\x8A\x85\x16\x15a\x15\x98W\x8A\x90\x81\x80a\x14Ma\x14<a\x0C\xCB\x8B`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[a\x07\xF1a\x14H\x8BaG\xD7V[a4YV[\x9E\x90\x9E\x16\x9D\x16\x95\x16\x85\x10a\x15\x88W\x16\x8A\x10a\x15zWPa\x15C\x89a\x154a\x08+``a\x15:\x87\x8E\x9F\x8E\x9Fa\x04\xC2\x9F\x99\x8F\x9A\x8F\x8F\x8F\x7F0\x84\xCA\xF4\x89f\\\xAB\x07E,\xFEO=\x0E\xB5\xE0\xDC\x15\xEA\xC6\xFCe\x80\x98\x85\x8Ec\x9Ep\xE5:\x9F\x95`\x01`\x01`@\x1B\x03\x9Fa\x15,\x94a\x15\x01\x88\x95a\x14\xF1a\x08+\x9Ba\x154\x9Da\x15\x17\x96a\x14\xD4a\x14Ha\x15'\x9BaG\xD7V[\x92a\x14\xDDa\x18sV[B\x81R\x9B\x8C\x01R\x8A\x01R`\x0F\x0B``\x89\x01RV[`\x01`\x01`@\x1B\x03\x16`\x80\x87\x01RV[3`\xA0\x86\x01R`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x85\x01RV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x83\x01RV[a4\xDDV[\x01Q`\xFF\x16\x90V[\x90aH#V[\x9C\x01Q`\xFF\x16\x90V[\x86Q\x89\x81R` \x81\x01\x82\x90R`\x01`\x01`\x80\x1B\x03\x90\x94\x16`@\x85\x01R\x98`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x95\x16\x93\x16\x91\x80``\x81\x01a\x0E6V[\x86QcVZ\xDE\xF5`\xE1\x1B\x81R\xFD[\x88Qc\x064HC`\xE4\x1B\x81R\x83\x90\xFD[P\x86Qc\nw\xB0/`\xE1\x1B\x81R\xFD[\x93Pa\x15\xEAa\x15\xE4\x86a\x15\xCC3`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x90`\x01`\x01`@\x1B\x03\x16`\0R` R`@`\0 \x90V[TaU+V[\x93a\x14\rV[\x84Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16\x81\x88\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a\x16,\x91P\x85=\x87\x11a\x0F\xA4Wa\x0F\x96\x81\x83a\x18RV[8a\x13\xA7V[a\x16:a;\xBFV[a\x13<V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wb\xFF\xFF\xFFa\x16\\a\x06pV[\x16`\0R`\t` R`\x80`@`\0 `\xFF`\x01\x82T\x92\x01T`@Q\x92\x82`\x01\x80`\xA0\x1B\x03\x91\x82\x81\x16\x86R`\xA0\x1C\x16` \x85\x01R\x81\x16`@\x84\x01R`\xA0\x1C\x16``\x82\x01R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x16\xBEa2\xB9V[`@Qc\xF7|G\x91`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x16Z\xFA\x90\x81\x15a\x05OW`\0\x91a\x17\xA4W[P\x163\x03a\x17\x8FW`\x14\x81\x11\x80\x15a\x17\x85W[a\x17kW\x7F\x81\xC99\x14H\0(v\x03G\x9B\x97\xBB\xA9\xC1\x12\x88\xCEz\xBCZ\xCBH\x90y\xE1Y\xF3\\\xF9\x8B\xD1\x90`\rTa\x17S\x82`\rUV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xA1a\0!a3\x02V[`@QcdYtw`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x90\xFD[P`\x04\x81\x10a\x17!V[`@Q`\x01b\x10X\xF5`\xE2\x1B\x03\x19\x81R`\x04\x90\xFD[a\x17\xC5\x91P` =\x81\x11a\x17\xCBW[a\x17\xBD\x81\x83a\x18RV[\x81\x01\x90aC\x97V[8a\x17\x0EV[P=a\x17\xB3V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[a\x17\xD2V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[a\x01\0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\x03W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@RV[`@Q\x90a\x18\x80\x82a\x18#V[V[`@Q\x90a\x18\x80\x82a\x18\x08V[`\xA0\x90`\x03\x19\x01\x12a\x02\xB8W`@Q\x90a\x18\xA8\x82a\x17\xE8V[\x81`\x045a\x18\xB5\x81a\x0B\x06V[\x81R`$5a\x18\xC3\x81a\x0B\x06V[` \x82\x01R`D5a\x18\xD4\x81a\x05eV[`@\x82\x01R`d5a\x18\xE5\x81a\x05TV[``\x82\x01R`\x80`\x845\x91a\x18\xF9\x83a\x05eV[\x01RV[4a\x02\xB8W`\xE06`\x03\x19\x01\x12a\x02\xB8Wa\x19\x176a\x18\x8FV[``a\x19u`\xE4`\xC45\x93a\x19+\x85a\x02\xA7V[\x80\x84\x01Q`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\n` R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x90\x81\x16\x16\x90`@Q\x96\x87\x95\x86\x94c@W\xCE\xBB`\xE1\x1B\x86R`\x04\x86\x01\x90a'\rV[`\xA45`\xA4\x85\x01R\x16`\xC4\x83\x01RZ\xFA\x80\x15a\x05OW`\0\x90\x81\x92\x82\x91a\x19\xBBW[Pa\x04\xC2\x90`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x90Pa\x04\xC2\x92Pa\x19\xE3\x91P``=\x81\x11a\x19\xEAW[a\x19\xDB\x81\x83a\x18RV[\x81\x01\x90aD.V[\x90\x92a\x19\x97V[P=a\x19\xD1V[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x1A\x0E\x81a\x05TV[`$5\x90\x81`\x0F\x0B\x91\x82\x81\x03a\x02\xB8Wb\xFF\xFF\xFF\x90a\x1A}a\x08\x13a\x1AT`\0\x93a\x1AOa\x0C\xCB\x88`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[aM\x8BV[\x94\x90`\x01`\x01`\x80\x1B\x03\x80\x91\x16\x95\x16\x95` \x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x93\x12\x15a\x1A\xCFWa\x1A\xB0\x91a\x154a\x08+``a\x088a\x1A\xABa\x1A\xAB\x96a\x154a\x08+` \x8C\x01Q`\xFF\x16\x90V[aU+V[\x90[`@\x80Q`\x01`\x01`\x80\x1B\x03\x92\x83\x16\x81R\x92\x90\x91\x16` \x83\x01R\x90\xF3[a\x1A\xFC\x91a\x1A\xF6a\x08+``a\x088a\x1A\xABa\x1A\xAB\x96a\x1A\xF6a\x08+` \x8C\x01Q`\xFF\x16\x90V[\x90aH]V[\x90a\x1A\xB2V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x01`\x01`@\x1B\x03`\x045a\x1B'\x81a\x05TV[\x16`\0R`\n` R`@`\0 \x80Ta\x04\xC2`\x01\x83\x01T\x92a\x1Bg`\x03a\x1BX`\x02\x84\x01T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x92\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@\x80Q`\x01`\x01`\x80\x1B\x03\x86\x81\x16\x82R`\x80\x96\x87\x1C` \x83\x01R\x87\x16\x91\x81\x01\x91\x90\x91Rc\xFF\xFF\xFF\xFF\x86\x86\x1C\x16``\x82\x01Ra\xFF\xFF`\xA0\x87\x81\x1C\x82\x16\x96\x83\x01\x96\x90\x96R`\xB0\x96\x90\x96\x1C\x90\x95\x16\x93\x85\x01\x93\x90\x93R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\xC0\x85\x01R\x90\x91\x16`\xE0\x83\x01R\x81\x90a\x01\0\x82\x01\x90V[4a\x02\xB8W``6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x1B\xF9\x81a\x05TV[a\x1C\x01a\x06\x82V[a\x1C\ta\x06\x93V[\x91a\x1C\x12a2\xB9V[a\x1C/\x81`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`\x02\x81\x01T\x90\x92\x903`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x03a\x17\x8FWa\xFF\xFF\x80\x85\x16\x94\x85a\x1D\x1FW[P\x81\x16\x92\x83a\x1C\x97W[PP`\x01`\x01`@\x1B\x03\x16\x7F\x80N\x0E\xF8\xEB\xD1\x19o\x98\xB3\xC6\xA20\xDE\xFF\xD8\xCF\xE0<\xB1\x92?\xE0\xE4\x02%-\x06\xD8\xD4v\xDA`\0\x80\xA4a\0!a3\x02V[`\x01\x01a\x1C\xD4a\x1C\xD0a\x1C\xBAa\x1C\xB3\x84Ta\xFF\xFF\x90`\xA0\x1C\x16\x90V[a\xFF\xFF\x16\x90V[\x86\x90\x80\x82\x10\x90\x82\x14\x17\x90`\x01\x80\x82\x11\x91\x14\x17\x16\x90V[\x15\x90V[a\x1D\x03W\x80Ta\xFF\xFF`\xB0\x1B\x19\x16`\xB0\x92\x90\x92\x1Ba\xFF\xFF`\xB0\x1B\x16\x91\x90\x91\x17\x90U`\x01`\x01`@\x1B\x038a\x1C_V[`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81Ra\xFF\xFF\x83\x16`\x04\x82\x01R`$\x90\xFD[a\x03\xE8\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15a\x1D_W`\x01\x85\x01\x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x92\x90\x92\x1Ba\xFF\xFF`\xA0\x1B\x16\x91\x90\x91\x17\x90U8a\x1CUV[`@Qc\x97\x1B1\t`\xE0\x1B\x81Ra\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a\x1D\x9A\x81a\x02\xA7V[`$5\x90a\x1D\xA7\x82a\x05eV[3`\0\x90\x81R`\x01` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x85\x16\x84R\x90\x91R\x90 \x91\x15\x15\x91`\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90U`@Q\x91\x82R`\x01\x80`\xA0\x1B\x03\x16\x90\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\0[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wb\xFF\xFF\xFFa\x1E/a\x06pV[\x16`\0R`\x08` R` c\xFF\xFF\xFF\xFF`@`\0 T\x16`@Q\x90\x81R\xF3[` \x80\x82\x01\x90\x80\x83R\x83Q\x80\x92R`@\x83\x01\x92\x81`@\x84`\x05\x1B\x83\x01\x01\x95\x01\x93`\0\x91[\x84\x83\x10a\x1E\x82WPPPPPP\x90V[\x90\x91\x92\x93\x94\x95\x84\x80a\x1E\xA0`\x01\x93`?\x19\x86\x82\x03\x01\x87R\x8AQa\x04\x13V[\x98\x01\x93\x01\x93\x01\x91\x94\x93\x92\x90a\x1ErV[` \x80`\x03\x196\x01\x12a\x02\xB8W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB8Wa\x1E\xDC\x906\x90`\x04\x01a\x08eV[\x91a\x1E\xE9`\x0FT`\xFF\x16\x90V[a\x1F\xB0Wa\x1E\xF5a2\xB9V[a\x1F\x07`\x01`\xFF\x19`\x0FT\x16\x17`\x0FUV[a\x1F\x0Fa;\xBFV[a\x1F\x18\x83a1tV[\x92`\0\x90\x81[\x81\x81\x10a\x1FTWa\x04\xC2\x86a\x1F8`\xFF\x19`\x0FT\x16`\x0FUV[a\x1F@aACV[a\x1FHa3\x02V[`@Q\x91\x82\x91\x82a\x1ENV[\x82\x80a\x1Fa\x83\x85\x89a1\xFEV[\x90a\x1Fq`@Q\x80\x93\x81\x93a2DV[\x03\x900Z\xF4a\x1F~a2uV[\x90\x15a\x1F\xA9W\x90a\x1F\xA4\x91a\x1F\x93\x82\x89a2\xA5V[Ra\x1F\x9E\x81\x88a2\xA5V[Pa1\xD4V[a\x1F\x1EV[\x80Q\x90\x85\x01\xFD[`@QcU\xE1\xF7\xC5`\xE0\x1B\x81R`\x04\x90\xFD[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W` `\rT`@Q\x90\x81R\xF3[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a \x87\x81a\x02\xA7V[`\x01\x80`\xA0\x1B\x03\x16`\0R`\x02` R` `@`\0 T`@Q\x90\x81R\xF3[a \xB06a\x10\xC2V[\x90a \xB9a2\xB9V[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16\x91\x90\x81\x16\x90\x81\x83\x14a#dWa \xFCa \xF3\x85a\n\xE1\x84`\x01\x80`\xA0\x1B\x03\x16`\0R`\x0B` R`@`\0 \x90V[Tb\xFF\xFF\xFF\x16\x90V[\x93b\xFF\xFF\xFF\x94\x85\x81\x16a#EWP`@\x94\x85Q\x92c1<\xE5g`\xE0\x1B\x92\x83\x85R` \x80\x86`\x04\x81\x8AZ\xFA\x95\x86\x15a\x05OW`\0\x96a#&W[P\x88Q\x94\x85R\x80\x85`\x04\x81\x8BZ\xFA\x94\x85\x15a\x05OW`\0\x95a\"\xF7W[P`\xFF\x86\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x15a\"\xDDWa!\x8Fa\x1C\xD0`\xFF\x87\x16`\x12\x81\x10`\x12\x82\x14\x17\x90`\x06\x80\x82\x11\x91\x14\x17\x16\x90V[a\"\xC3W\x92a\"\x8D\x7F\xC0\xC5\xDF\x98\xA4\xCA\x87\xA3!\xA3;\xF1'|\xF3-1\xA9{l\xE1K\x97G8!I\xB9\xE2c\x1E\xA3\x93a\"fa\x04\xC2\x9B\x99\x97\x94a\"W\x8Aa\"3\x9D\x9B\x99a!\xE3a!\xDE`\x06Tb\xFF\xFF\xFF\x16\x90V[a86V[\x9E\x8Fa!\xFD\x81b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x19`\x06T\x16\x17`\x06UV[`\x01`\x01`\xA0\x1B\x03\x88\x16`\0\x90\x81R`\x0B` R`@\x90 a\" \x90\x87\x90a\n\xE1V[\x90b\xFF\xFF\xFF\x16b\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[a\"Ma\">a\x18\x82V[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[\x85\x01\x90`\xFF\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16\x82\x8B\x01RV[`\xFF\x84\x16``\x82\x01Ra\"\x88\x8Ab\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[a8JV[\x86Q`\xFF\x94\x85\x16\x81R\x91\x90\x93\x16` \x82\x01R\x91\x86\x16\x91`@\x90\xA4a\"\xAFa3\x02V[Qb\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x88Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x86\x16`\x04\x82\x01R`$\x90\xFD[\x88Qc\xC3\xDAwG`\xE0\x1B\x81R`\xFF\x87\x16`\x04\x82\x01R`$\x90\xFD[\x81a#\x18\x92\x96P=\x87\x11a#\x1FW[a#\x10\x81\x83a\x18RV[\x81\x01\x90a8\x1DV[\x938a!RV[P=a#\x06V[\x81a#>\x92\x97P=\x88\x11a#\x1FWa#\x10\x81\x83a\x18RV[\x948a!5V[`@Qc\xB0\x98\x8CC`\xE0\x1B\x81Rb\xFF\xFF\xFF\x91\x90\x91\x16`\x04\x82\x01R`$\x90\xFD[`@Qc\x01D\xD33`\xE2\x1B\x81R`\x04\x90\xFD[4a\x02\xB8W``6`\x03\x19\x01\x12a\x02\xB8Wa\x04\xC2a$8`\x045a#\x99\x81a\x05TV[b\xFF\xFF\xFF\x81` \x1C\x16`\0R`\t` R`@`\0 a$3a\x0C\xCBa$\x19a$\x11a\x08+a$\t`@Q\x96a#\xCE\x88a\x18\x08V[`\xFF``\x82T\x99`\x01\x83\x81\x80`\xA0\x1B\x03\x9C\x8D\x81\x16\x84R`\xA0\x1C\x16\x94\x85` \x84\x01R\x01T\x9A\x8B\x16`@\x82\x01R\x01\x98`\xA0\x1C\x16\x88R`$5aG\xE8V[\x95Q`\xFF\x16\x90V[`D5aG\xE8V[\x93`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[aLdV[`@Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8W`\x045a$o\x81a\x02\xA7V[`\x01\x80`\xA0\x1B\x03\x16`\0R`\x07` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x02\xB8W`@6`\x03\x19\x01\x12a\x02\xB8W`\x045a$\xAC\x81a\x02\xA7V[`$5\x90a$\xB8a2\xB9V[`@Qc\xF7|G\x91`\xE0\x1B\x81R` \x91`\x01`\x01`\xA0\x1B\x03\x91\x83\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x16Z\xFA\x80\x15a\x05OW\x83\x91`\0\x91a&5W[P\x163\x03a\x17\x8FW`@Qc1<\xE5g`\xE0\x1B\x81R\x91\x81\x16\x93`\0\x92\x90\x84\x82`\x04\x81\x89Z\xFA\x91\x82\x15a\x05OW\x7F\x1F\xDD\0 5\x88\x93U\x97\x13\xDE\xF8\xB4,\xADf\x1F\xFB\xC7U\xD1\xA2dY@'\x92\x14B\xBBV\xA0\x95a%\xDA\x95\x93a&\x16W[PP`\0\x19\x81\x03a%\xFEWPa%\xCA\x90a%\x9C`\xFFa%\x94\x85`\x01\x80`\xA0\x1B\x03\x16`\0R`\x07` R`@`\0 \x90V[T\x92\x16aH\x0FV[\x81\x04\x92[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x07` R`@\x90 a%\xC3\x83\x82Ta8\x03V[\x90Ua<\xCCV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2`\x0FT`\xFF\x16\x15a%\xF1W[a\0!a3\x02V[a%\xF9aACV[a%\xE9V[\x91a&\x0E`\xFFa%\xCA\x93\x16aH\x0FV[\x83\x02\x90a%\xA0V[a&-\x92\x93P\x80=\x10a#\x1FWa#\x10\x81\x83a\x18RV[\x908\x80a%cV[a&L\x91P\x85=\x87\x11a\x17\xCBWa\x17\xBD\x81\x83a\x18RV[8a%\x0BV[4a\x02\xB8W` 6`\x03\x19\x01\x12a\x02\xB8Wa\x06\x1B` `\x045a&t\x81a\x05TV[`\x01`\x01`@\x1B\x03\x81\x16`\0\x90\x81R`\n\x83R`@\x90 `\x03\x01T`\x01`\x01`\xA0\x1B\x03\x16`@Q\x80\x80\x95\x81\x94c8\xCCn\x8D`\xE2\x1B\x83R`\x04\x83\x01\x91\x90\x91`\x01`\x01`@\x1B\x03` \x82\x01\x93\x16\x90RV[4a\x02\xB8W` `\xFFa'\x01a&\xD86a\x10\xC2V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x01\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x15\x15\x81R\xF3[`\x80\x80\x91`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[`\xA0\x81\x01\x92\x91a\x18\x80\x91\x90a'\rV[4a\x02\xB8W``6`\x03\x19\x01\x12a\x02\xB8W`\x80a(4`\xA0`\x045a'\x86\x81a\x05TV[`$5\x90a'\x93\x82a\x05eV[`D5\x91a'\xA0\x83a\x02\xA7V[`@Qa'\xAC\x81a\x17\xE8V[`\0\x96\x81\x88\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01Ra'\xF0a\x0C\0a\x0C\0`\x03a\x0B\xC3\x86`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`@Qcx=\xC3\xCF`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x93\x16`\x04\x84\x01R\x90\x15\x15`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`D\x82\x01R\x92\x83\x91\x90\x82\x90\x81\x90`d\x82\x01\x90V[\x03\x91Z\xFA\x90\x81\x15a\x05OWa\x04\xC2\x92\x91a(VW[P`@Q\x91\x82\x91\x82a'RV[a(w\x91P`\xA0=\x81\x11a(}W[a(o\x81\x83a\x18RV[\x81\x01\x90aC\xBBV[8a(IV[P=a(eV[4a\x02\xB8W`\xA06`\x03\x19\x01\x12a\x02\xB8W`\x045a(\xA1\x81a\x02\xA7V[`$5a(\xAD\x81a\x02\xA7V[`D5\x91`d5`\x845`\x01`\x01`@\x1B\x03\x81\x11a\x02\xB8Wa(\xD3\x906\x90`\x04\x01a\x06\xA4V[\x93\x90\x91`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x92\x903\x84\x14\x80\x15a)\xD7W[a(\xF6\x90aQgV[a)\x16\x88a\no\x88`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a)!\x84\x82Ta8\x03V[\x90Ua)C\x88a\no\x84`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a)N\x84\x82Ta8\x10V[\x90U\x81\x16\x80\x93\x88\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb`@Q\x80a)\x93\x883\x95\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[\x03\x90\xA4;\x15a)\xC6W`\0` \x94\x95\x96a\t\xC6`@Q\x98\x89\x96\x87\x95\x86\x94c\xF2:na`\xE0\x1B\x9B\x8C\x87R3`\x04\x88\x01aQ\xF9V[P\x92PPPa\0!\x91P\x15\x15aQ\xA4V[Pa(\xF6a)\xFEa\n\xF83a\n\xE1\x8A`\x01\x80`\xA0\x1B\x03\x16`\0R`\x01` R`@`\0 \x90V[\x90Pa(\xEDV[`\xA06`\x03\x19\x01\x12a\x02\xB8Wa*\x19a2\xB9V[`\x0FT`\xFF\x16\x15a0\xC0W[a*-a7<V[a*5a7HV[\x90a*Ma*Aa7TV[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90`\x01`\x01`\x80\x1B\x03\x80\x93\x16\x91a*~a*ea7<V[`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[a*\x9Ca\x1C\xD0a*\x8D\x83a3\x9DV[``\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x15\x90V[a0\x9FWa*\xB1a*\xAC\x82a3\x9DV[aL:V[a0\x8DWa*\xDDa\x0C\0a\x0C\0`\x03a\x0B\xC3\x87`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`@\x80Qc\xE6\x04{\x19`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x86\x16`\x04\x80\x83\x01\x91\x90\x91R` \x98\x92\x94\x92\x93\x90\x91\x89\x81`$\x81\x85Z\xFA\x90\x81\x15a\x05OW`\0\x91a0pW[P\x15a0KWa+.B\x84aJ\xE2V[a+q\x85a+:a7`V[\x81Qc;\x1C\xDA\x15`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x8B\x16\x86\x82\x01\x90\x81R\x91\x15\x15` \x83\x01R3`@\x83\x01R\x92\x83\x91\x82\x91``\x90\x91\x01\x90V[\x03\x81`\0\x86Z\xF1\x90\x81\x15a\x05OW`\0\x90\x81\x92a0+W[P\x15a0\x1BWa+\xB2a\x08\x13b\xFF\xFF\xFF\x8A\x8D\x1C\x16b\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[\x90\x8A\x87a+\xBDa7\x97V[\x93a+\xC6a7`V[\x15a/\xA8W\x90\x81a+\xEEa+\xE3a,8\x95a,(\x95\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x88\x01RV[a,\na+\xFF``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x88\x01RV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x87\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x84\x01RV[a,@a7lV[a/gW[\x90a,]\x91\x81R\x89`\x80\x82\x01R\x87`\xA0\x82\x01RaP\x8CV[\x92`\xA0\x84\x01\x90\x81Q\x15a/WW`\x80\x85\x01\x93\x84Q\x15a/IW\x87a,\xFA\x8Da,\xBCa,\x876a\x18\x8FV[\x91a,\xAFa,\x95\x8BQaU+V[\x91a,\xA0\x8AQaU+V[`\x01`\x01`\x80\x1B\x03\x16\x90\x85\x01RV[`\x01`\x01`\x80\x1B\x03\x16\x82RV[`\x02\x85\x01T`\x01`\x01`\xA0\x1B\x03\x9A\x903\x90\x8C\x16\x03a/7W`\x01\x86\x01T`\xB0\x1Ca\xFF\xFF\x16\x91[\x86T\x90a\xFF\xFF`\rT\x94\x16\x92\x82`\x80\x1C\x92\x16\x90aO\x1FV[\x93\x92``\x8B\x99\x92\x93\x99\x01\x98\x89R\x83\x8B\x01R\x8D\x8AQ\x90a-M\x85Q\x96\x87\x95\x86\x94\x85\x94c\xA4G\x89\x19`\xE0\x1B\x86R\x8B\x86\x01\x90\x94\x93\x92``\x92`\x01`\x01`@\x1B\x03`\x80\x84\x01\x97\x16\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x91Z\xFA\x90\x81\x15a\x05OW\x8D`\0\x91\x82\x93a/\x06W[P\x82\x90\x89\x01R\x15a.\xE1WPP\x93\x88\x99\x9A\x93\x7F\xCD\x80T&o\xAE;\xBD#\0\xC8\x9A@t\xC4\x11=\x12\xB9\xA5\x83\xF7\xCD#\x90\xD40\xAFp\xB1*X\x93a-\xF7a.\x06\x94a-\xC4a\x04\xC2\x9D\x99a-\xAEa7`V[a-\xBB\x87Q\x87Q\x90a8\x03V[\x90\x84Q\x92aKqV[`\xC0\x85\x01\x80Q\x90\x94a-\xE1\x91`\x01`\x01`\xA0\x1B\x03\x16\x90Q\x90a<cV[`\xE0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90Q\x90a<\xCCV[Q\x90\x81a.\xB7W[PPaP\xFCV[\x93a.\x0Fa7`V[`\xC0\x86\x01Q`\xE0\x87\x01Q\x88\x88\x01Q\x95\x90\x97\x01Q\x88Q\x92\x15\x15\x83R` \x83\x01\x8D\x90R`@\x83\x01\x8A\x90R``\x83\x01\x95\x90\x95R`\x80\x82\x01\x94\x90\x94R`\x01`\x01`\xA0\x1B\x03\x95\x82\x16\x86\x16\x95\x91\x90\x93\x16\x16\x92`\x01`\x01`@\x1B\x03\x16\x91`\xA0\x90\xA4a.xa\x0EH`\x0FT`\xFF\x16\x90V[\x15a.\xAAW[a.\x86a3\x02V[Q\x93\x84\x93\x84`@\x91\x94\x93\x92`\x01`\x01`@\x1B\x03``\x83\x01\x96\x16\x82R` \x82\x01R\x01RV[a.\xB2aACV[a.~V[Q`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x07` R`@\x90 a.\xD8\x90a\n\xA8V[\x90U8\x80a-\xFFV[\x86Q\x89Qc\\\x9E\xF7\x05`\xE1\x1B\x81R\x92\x83\x01\x90\x81R` \x81\x01\x91\x90\x91R\x81\x90`@\x01\x03\x90\xFD[\x90\x92Pa/)\x91P\x8A=\x8C\x11a/0W[a/!\x81\x83a\x18RV[\x81\x01\x90a7xV[\x918a-cV[P=a/\x17V[`\x01\x86\x01T`\xA0\x1Ca\xFF\xFF\x16\x91a,\xE2V[\x87Qc\x13\xFD\x9Bm`\xE0\x1B\x81R\xFD[\x86Qc7\xC2\xD9\xA7`\xE1\x1B\x81R\x84\x90\xFD[`\xC0\x82\x01Qa/~\x90`\x01`\x01`\xA0\x1B\x03\x16a1RV[`\0\x81\x13a/\x8DW[Pa,EV[a,]\x92\x91\x9APa*Aa/\xA0\x91aU+V[\x99\x90\x91a/\x87V[a,(\x91Pa/\xE7a/\xDCa0\x16\x94a/\xD3a/\xC8``\x86\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01\0\x8A\x01RV[\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x87\x01RV[\x80\x8A\x01Qa0\t\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x87\x01RV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a,8V[\x85Qc.\xD0\xEA\x01`\xE0\x1B\x81R\x83\x90\xFD[\x90Pa0D\x91P\x86=\x88\x11a/0Wa/!\x81\x83a\x18RV[\x908a+\x89V[\x84Qc\xBC'\xA5\x17`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x88\x16\x81\x84\x01\x90\x81R\x81\x90` \x01\x03\x90\xFD[a0\x87\x91P\x8A=\x8C\x11a\x0F\xA4Wa\x0F\x96\x81\x83a\x18RV[8a+\x1EV[`@Qcz\x95\xCB!`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\t\x0Bp}`\xE1\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01R`$\x90\xFD[a0\xC8a;\xBFV[a*%V[4a\x02\xB8W`\x006`\x03\x19\x01\x12a\x02\xB8W` `\0Rk\x0Bv1.4.0-beta`+R```\0\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x02\xB8WV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\x03W`\x1F\x01`\x1F\x19\x16` \x01\x90V[`@Q=`\0\x82>=\x90\xFD[a\x04I\x900\x90aGYV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\x03W`\x05\x1B` \x01\x90V[\x90a1~\x82a1]V[a1\x8B`@Q\x91\x82a\x18RV[\x82\x81R\x80\x92a1\x9C`\x1F\x19\x91a1]V[\x01\x90`\0[\x82\x81\x10a1\xADWPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a1\xA1V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x19\x81\x14a1\xE3W`\x01\x01\x90V[a1\xBEV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x91\x90\x81\x10\x15a2?W`\x05\x1B\x81\x015\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x02\xB8W\x01\x90\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xB8W` \x01\x826\x03\x81\x13a\x02\xB8W\x91\x90V[a1\xE8V[\x90\x80\x92\x91\x827\x01`\0\x81R\x90V[`@Q\x90` \x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@R`\0\x82RV[=\x15a2\xA0W=\x90a2\x86\x82a1+V[\x91a2\x94`@Q\x93\x84a\x18RV[\x82R=`\0` \x84\x01>V[``\x90V[\x80Q\x82\x10\x15a2?W` \x91`\x05\x1B\x01\x01\x90V[`\x0CT`\x01\x81\x14\x15\x80a2\xEBW[a2\xD9Wa2\xD4\x90a1\xD4V[`\x0CUV[`@Qc\x02\xB8\0-`\xE0\x1B\x81R`\x04\x90\xFD[P`\xFF`\x0FT\x16\x15\x80a2\xC7WP`\x02\x81\x11a2\xC7V[`\x0CT\x80\x15a1\xE3W`\0\x19\x01`\x0CU`\x05T`\xFF\x16\x15\x80a37W[a3%WV[`@Qc2n\xFAC`\xE0\x1B\x81R`\x04\x90\xFD[P`\xFF`\x0FT\x16\x15a3\x1FV[\x90\x81` \x91\x03\x12a\x02\xB8WQa\x04I\x81a\x05eV[\x90`@Qa3f\x81a\x18\x08V[\x82T`\x01`\x01`\xA0\x1B\x03\x80\x82\x16\x83R`\xFF`\xA0\x92\x83\x1C\x81\x16` \x85\x01R`\x01\x90\x95\x01T\x90\x81\x16`@\x84\x01R\x90\x1C\x90\x92\x16``\x83\x01RV[\x90a\x18\x80`@Qa3\xAD\x81a\x18#V[`\xE0a4K`\x03\x83\x96a44\x81Ta3\xE6`\x01`\x01`\x80\x1B\x03\x91a3\xDC\x83\x82\x16\x8A\x90`\x01`\x01`\x80\x1B\x03\x16\x90RV[`\x80\x1C` \x89\x01RV[a4\x03`\x01\x84\x01T\x91\x82\x16`@\x89\x01\x90`\x01`\x01`\x80\x1B\x03\x16\x90RV[`\x80\x81\x90\x1Cc\xFF\xFF\xFF\xFF\x16``\x88\x01Ra\xFF\xFF`\xA0\x82\x90\x1C\x81\x16`\x80\x89\x01R\x90`\xB0\x1C\x16`\xA0\x87\x01\x90a\xFF\xFF\x16\x90RV[`\x02\x81\x01Ta\x0B\xC3\x90`\x01`\x01`\xA0\x1B\x03\x16a/\xF9V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14a1\xE3W`\0\x03\x90V[`\x0F\x0Bc;\x9A\xC9\xFF\x19\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a1\xE3WV[`\x01`\xFF\x1B\x81\x14a1\xE3W`\0\x03\x90V[\x91\x90\x91`\x01`\x01`\x80\x1B\x03\x80\x80\x94\x16\x91\x16\x01\x91\x82\x11a1\xE3WV[`\x01`\x01`\x80\x1B\x03\x91\x82\x16\x90\x82\x16\x03\x91\x90\x82\x11a1\xE3WV[`\x80\x81\x01\x90a4\xF6a*e\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90a5\x04` \x82\x01QaU+V[\x90a5\x12`@\x82\x01QaU+V[\x93``\x82\x01\x91a5#\x83Q`\x0F\x0B\x90V[`\x01`\x01`\x80\x1B\x03\x92\x83a5A`\x01\x89\x01T`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x15a6\xFAW[a*ea5\xAA\x91a5\xB5\x93`\x0F\x0B`\0\x81\x13`\0\x14a6\xB8W`\xA0\x86\x01Qa5\x9D\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x90`\x01`\x01`@\x1B\x03a5\x8E\x85Q`\x01`\x01`@\x1B\x03\x16\x90V[\x16a5\x97a2RV[\x92aS\x85V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x84Q`\x0F\x0B\x90aK\x15V[`\0a5\xF2a5\xECa5\xE4`\xE0a5\xD5`\xC0\x87\x01Q`\x01\x80`\xA0\x1B\x03\x16\x90V[\x95\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x95Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x12\x15a6vW\x83a6 a\x18\x80\x97\x94\x84a6\x18a60\x95a6L\x97a6V\x9A\x16\x90a<\xCCV[\x86\x16\x90a<\xCCV[\x85T`\x01`\x01`\x80\x1B\x03\x16a4\xC4V[\x84T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x84UV[\x82T`\x80\x1Ca4\xC4V[\x81T`\x01`\x01`\x80\x1B\x03\x16`\x80\x91\x90\x91\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x90UV[\x83a6\x9Ea\x18\x80\x97\x94\x84a6\x96a60\x95a6\xAE\x97a6V\x9A\x16\x90a<cV[\x86\x16\x90a<cV[\x85T`\x01`\x01`\x80\x1B\x03\x16a4\xA9V[\x82T`\x80\x1Ca4\xA9V[`\xA0\x86\x01Qa6\xF5\x91\x90`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`@\x1B\x03a6\xEDa6\xE7\x86Q`\x01`\x01`@\x1B\x03\x16\x90V[\x93a4\x98V[\x92\x16\x90aT]V[a5\x9DV[`\0\x87Uc;\x9A\xCA\0`\x0F\x83\x90\x0B\x12a7*Wa*ea5\xAA\x91a7 a5\xB5\x94a4qV[\x93P\x91PPa5HV[`@Qc\xCBm\xABu`\xE0\x1B\x81R`\x04\x90\xFD[`d5a\x04I\x81a\x05TV[`\x045a\x04I\x81a\x0B\x06V[`$5a\x04I\x81a\x0B\x06V[`\x845a\x04I\x81a\x05eV[`D5a\x04I\x81a\x05eV[\x91\x90\x82`@\x91\x03\x12a\x02\xB8W` \x82Qa7\x91\x81a\x05eV[\x92\x01Q\x90V[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\x03W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[`\0\x19\x81\x01\x91\x90\x82\x11a1\xE3WV[\x91\x90\x82\x03\x91\x82\x11a1\xE3WV[\x91\x90\x82\x01\x80\x92\x11a1\xE3WV[\x90\x81` \x91\x03\x12a\x02\xB8WQ`\xFF\x81\x16\x81\x03a\x02\xB8W\x90V[b\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a1\xE3W`\x01\x01\x90V[\x81Q\x81T` \x84\x01Q`\xFF`\xA0\x1B`\xA0\x91\x90\x91\x1B\x16`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x17\x17\x82U`@\x83\x01Q`\x01\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x92\x90\x91\x16\x91\x90\x91\x17\x81Ua\x18\x80\x91`\xFF\x90``\x01Q\x82T`\xFF`\xA0\x1B\x19\x16\x91\x16`\xA0\x1B`\xFF`\xA0\x1B\x16\x17\x90UV[c\xFF\xFF\xFF\xFF\x80\x91\x16\x90\x81\x14a1\xE3W`\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90`\x01`\x01`@\x1B\x03a\x04I\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a8\xD3V[\x96\x98\x97\x90\x94\x95\x92\x95a9$a2\xB9V[b\xFF\xFF\xFF\x97\x80\x89\x16a;\xB9WP`\x06Tb\xFF\xFF\xFF\x16\x97[\x88\x16\x15a;\xA7Wa9[\x88b\xFF\xFF\xFF\x16`\0R`\x08` R`@`\0 \x90V[\x90a9\x8Ba9ua9p\x84Tc\xFF\xFF\xFF\xFF\x16\x90V[a8\xBEV[\x83Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x82\x16\x17\x90\x93UV[`\x01`\x01`\xA0\x1B\x03\x94\x80\x86\x16a;\xA1WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91[a9\xF6\x90\x8A\x83\x88\x16\x15\x15\x85\x89\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x8A\x16\x14\x15aH\xA2V[\x9A\x8B\x93a:\x16\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[\x84\x84\x8C\x8C\x8A\x8Da\xFF\xFF\x80\x91\x16\x93\x16\x91a:.\x96aH\xF6V[`\x0F\x80Th\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x19\x16`\x08\x87\x90\x1Bh\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x17\x90Ua:n\x85`\x01`\x01`@\x1B\x03\x16`\0R`\n` R`@`\0 \x90V[`\x03\x01T`@Qc\xE0hx\x7F`\xE0\x1B\x81R\x95`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92\x86\x92\x83\x92a:\x9E\x92\x90`\x04\x85\x01a8\xF4V[\x03\x81Z` \x94`\0\x91\xF1\x95\x86\x15a\x05OW\x8B\x98a;$`\x01a\x0B\xC3a;\x0Ea;\x01\x8Fa;x\x9A\x7Fa<|I?\x9A\xA3\x97M\xCD\xBB\x87\xE6\xFE\xA5L\xCD\xB9\x10>\xA7\x83.m\xF0\x8D\xB8\x1Co\x90\xD2\x9D\x9Ea;\x83W[Pb\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x9Db\xFF\xFF\xFF\x16`\0R`\t` R`@`\0 \x90V[`@\x80Q\x9A\x8BR` \x8B\x01\x97\x90\x97Ra\xFF\xFF\x91\x82\x16\x96\x8A\x01\x96\x90\x96R\x16``\x88\x01R`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80\x88\x01R\x16`\xA0\x86\x01R\x90\x82\x16\x95\x90\x91\x16\x93`\x01`\x01`@\x1B\x03\x16\x92\x90\x81\x90`\xC0\x82\x01\x90V[\x03\x90\xA4a\x18\x80a3\x02V[a;\x9A\x90` =\x81\x11a\x0F\xA4Wa\x0F\x96\x81\x83a\x18RV[P8a:\xEBV[\x91a9\xBFV[`@Qc\xCCzs\x9B`\xE0\x1B\x81R`\x04\x90\xFD[\x97a9;V[4a;\xC6WV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a;\xF0\x81aFlV[`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x02\xB8W`\0`\x04\x91`@Q\x92\x83\x80\x92c\r\x0E0\xDB`\xE4\x1B\x82R4\x90Z\xF1\x80\x15a\x05OWa<TW[P`@Q4\x81R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C` 3\x92\xA2V[a<]\x90a\x18?V[8a<%V[a<l\x81aFlV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 \x80T\x91\x83\x83\x01\x80\x93\x11a1\xE3W\x91\x90U`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x80\xB2\x17H\xC7\x87\xC5.\x87\xA6\xB2\"\x01\x1E\n\x0E\xD0\xF9\xCC \x15\xF0\xCE\xD4gHd-\xC6.\xE9\xF8\x90` \x90\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 T\x80\x83\x11a=VWPa<\xF6\x81aFlV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x02` R`@\x90 \x80T\x91\x83\x83\x03\x92\x83\x11a1\xE3W\x91\x90U`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x90\x7F\x1Cq\x1E\xCA\x8D\x0BiK\xBC\xB0\xA1Db\xA7\0b\"\xE7!\x95K,_\xF7\x98\xF6\x06\x81~\xB1\x102\x90` \x90\xA2V[`@Qc1Rv\xC9`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[b\xFF\xFF\xFF\x90\x93\x91\x92\x93` \x1C\x16`\0R`\t` R`@`\0 \x92`@Qa=\xA0\x81a\x18\x08V[`\xFF``\x86T\x92`\x01\x83\x81\x80`\xA0\x1B\x03\x95\x86\x81\x16\x84R`\xA0\x1C\x16\x98\x89` \x84\x01R\x01T\x93\x84\x16`@\x82\x01R\x01\x91`\xA0\x1C\x16\x81Ra=\xDC\x84aU+V[\x94`\x01`\x01`\x80\x1B\x03\x94\x85\x81\x03a>\"W[PPa=\xF9\x82aU+V[\x93\x82\x03a>\x04WPPV[a\x04I\x92\x93P\x90a>\x1Ca\x08+a\x1A\xAB\x93Q`\xFF\x16\x90V[\x90aG\xE8V[a>2\x92\x96P\x90a\x1A\xAB\x91aG\xE8V[\x938\x80a=\xEEV[`@Q\x90`\x04T\x80\x83R\x82` \x91\x82\x82\x01\x90`\x04`\0R\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x93`\0\x90[\x82\x82\x10a>\x8DWPPPa\x18\x80\x92P\x03\x83a\x18RV[\x85T`\x01`\x01`\xA0\x1B\x03\x16\x84R`\x01\x95\x86\x01\x95\x88\x95P\x93\x81\x01\x93\x90\x91\x01\x90a>wV[`\x0ET\x90`\x01`@\x1B\x82\x10\x15a\x18\x03W`\x01\x82\x01\x80`\x0EU\x82\x10\x15a2?W`\x0E`\0R\x80Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD`\x02\x93\x90\x93\x1B\x92\x83\x01U` \x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFE\x83\x01U`@\x81\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFF\x83\x01U``\x01Q\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC4\0\x91\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x04T\x80\x15a?\xF5W`\0\x19\x81\x01\x90\x80\x82\x10\x15a2?W\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9A\x90`\x04`\0R\x01k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x81T\x16\x90U`\x04UV[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\x0ET\x90a@\x18\x82a1]V[\x91`@a@'\x81Q\x94\x85a\x18RV[\x81\x84R\x83` \x80\x91\x01\x91`\x0E`\0R\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x90`\0\x93[\x85\x85\x10a@kWPPPPPPV[`\x04\x84`\x01\x92\x84Qa@|\x81a\x18\x08V[\x86T\x81R\x84\x87\x01T\x83\x82\x01R`\x02\x87\x01T\x86\x82\x01R\x84\x80`\xA0\x1B\x03`\x03\x88\x01T\x16``\x82\x01R\x81R\x01\x93\x01\x94\x01\x93\x91a@\\V[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a1\xE3WV[`\x0ET`\0\x80`\x0EU\x81a@\xDBWPPV[`\x01`\x01`\xFE\x1B\x03\x82\x16\x82\x03a1\xE3W`\x0E\x81R`\x02\x7F\xBB{JEM\xC3I9#H/\x07\x82#)\xED\x19\xE8$N\xFFX,\xC2\x04\xF8UL6 \xC3\xFD\x92\x81\x1B\x83\x01\x92[\x83\x81\x10aA&WPPPPV[\x80\x83`\x04\x92U\x83`\x01\x82\x01U\x83\x83\x82\x01U\x83`\x03\x82\x01U\x01aA\x19V[aAKa>:V[\x80Q\x80\x15aC\x8DW\x90`\x01\x91\x82\x80[aB\xE7W[PPPPaAka@\x0BV[\x80Q\x80[aA\x85WPPaA}aG(V[a\x18\x80a@\xC9V[aA\x8E\x81a7\xF4V[aA\xAC``aA\x9D\x83\x86a2\xA5V[Q\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[aA\xB6\x82\x85a2\xA5V[QQ` aA\xC4\x84\x87a2\xA5V[Q\x01Q\x81\x15aB\x97WPaB4aA\xE0\x93`@\x94\x85\x91\x88a2\xA5V[Q\x01Q\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x90\x83\x16\x03aB\x86WaB$\x913\x90aD\xE5V[aB.0\x85aDRV[\x92a8\x03V[\x90\x81\x81\x10aBKWPPPP[`\0\x19\x01\x80aAoV[a\x0Fz\x91aBX\x91a@\xB0V[\x92QcU\x13N\xF1`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x92\x90\x92R\x90\x81\x90`D\x82\x01\x90V[aB\x92\x91P3\x86aN\xCDV[aB$V[\x80\x91PaB\xA7W[PPPaBAV[aB\xD9aB\xB9\x93`@\x94\x85\x91\x88a2\xA5V[Q\x01Q\x91aB\xC9\x8103\x87aNvV[aB\xD30\x85aDRV[\x92a8\x10V[\x90\x81\x81\x10aBKWPaB\x9FV[\x15aC\x82W[`\0\x90aC\x05a0\taB\xFF\x83a7\xF4V[\x85a2\xA5V[aC\x0F0\x82aEUV[\x81\x15\x80\x15\x90aCyW[aC4W[PPPaC)a?\x9CV[`\0\x19\x01\x90\x83aAZV[aCq\x92aCl\x91aCF0\x83aDRV[\x90aCOa\x18\x82V[\x94\x85R` \x85\x01R`@\x84\x01R`\x01`\x01`\xA0\x1B\x03\x16``\x83\x01RV[a>\xB0V[8\x80\x80aC\x1EV[P\x80\x15\x15aC\x19V[\x80aB\xEDW\x80aA_V[PPa\x18\x80aG(V[\x90\x81` \x91\x03\x12a\x02\xB8WQa\x04I\x81a\x02\xA7V[\x90\x81` \x91\x03\x12a\x02\xB8WQ\x90V[\x90\x81`\xA0\x91\x03\x12a\x02\xB8W`\x80`@Q\x91aC\xD5\x83a\x17\xE8V[\x80QaC\xE0\x81a\x0B\x06V[\x83R` \x81\x01QaC\xF0\x81a\x0B\x06V[` \x84\x01R`@\x81\x01QaD\x03\x81a\x05eV[`@\x84\x01R``\x81\x01QaD\x16\x81a\x05TV[``\x84\x01R\x01QaD&\x81a\x05eV[`\x80\x82\x01R\x90V[\x90\x81``\x91\x03\x12a\x02\xB8W\x80QaDD\x81a\x05eV[\x91`@` \x83\x01Q\x92\x01Q\x90V[`@Qcp\xA0\x821`\xE0\x1B` \x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`$\x80\x83\x01\x91\x90\x91R\x81R\x91``\x83\x01\x91\x90`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x18\x03W`\0\x93\x84\x93`@RQ\x91Z\xFAaD\xA7a2uV[\x90\x15\x80\x15aD\xD9W[aD\xC7W\x80` \x80a\x04I\x93Q\x83\x01\x01\x91\x01aC\xACV[`@Qc\xC5.>\xFF`\xE0\x1B\x81R`\x04\x90\xFD[P` \x81Q\x14\x15aD\xB0V[`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x02\xB8W`@Q\x80\x92c.\x1A}M`\xE0\x1B\x82R\x81`$`\0\x95\x86\x80\x94\x89`\x04\x84\x01RZ\xF1\x80\x15a\x05OWaEBW[P\x81\x80\x93\x81\x92Z\xF1\x15aE0WV[`@Qc5e\x94\xAB`\xE0\x1B\x81R`\x04\x90\xFD[aEN\x90\x92\x91\x92a\x18?V[\x908aE!V[\x91`\0\x80\x81\x93aEw\x86`\x01\x80`\xA0\x1B\x03\x16`\0R`\x02` R`@`\0 \x90V[T`@Qc1<\xE5g`\xE0\x1B\x81R\x90` \x82`\x04\x81`\x01`\x01`\xA0\x1B\x03\x8C\x16Z\xFA\x90\x81\x15a\x05OW`\xFFaE\xB8\x92aE\xBF\x94\x88\x91aFNW[P\x16\x90aH]V[\x91\x87aDRV[`\x01`\x01`\xFF\x1B\x03\x80\x83\x11aFJW\x81\x11a\x05KW\x90aE\xDE\x91a@\xB0V[\x91\x80\x83\x13\x15aF\rWPP`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`\0\x90\x81R`\x03` R`@\x90 [\x80T`\xFF\x19\x16\x90UV[\x82\x91\x95\x92\x12aF4W[P`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x03` R`@\x90 aF\x03V[aF\x03\x91\x93PaFC\x90a4\x98V[\x92\x90aF\x17V[\x84\x80\xFD[aFf\x91P` =\x81\x11a#\x1FWa#\x10\x81\x83a\x18RV[8aE\xB0V[`\x05T`\xFF\x81\x16aG\x1BW[P`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x03` R`@\x90 T\x90\x91\x90`\xFF\x16\x15aF\xA1WPPV[`\x04T\x90`\x01`@\x1B\x82\x10\x15a\x18\x03W`\x01\x82\x01\x80`\x04U\x82\x10\x15a2?W\x7F\x8A5\xAC\xFB\xC1_\xF8\x1A9\xAE}4O\xD7\t\xF2\x8E\x86\0\xB4\xAA\x8Ce\xC6\xB6K\xFE\x7F\xE3k\xD1\x9B\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U`\0R`\x03` R`@`\0 `\x01`\xFF\x19\x82T\x16\x17\x90UV[`\xFF\x19\x16`\x05U8aFxV[`\x04TaGCW`\x01`\xFF\x19`\x05T\x16\x17`\x05U`\0`\x04UV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x81\x81R`\x02` \x90\x81R`@\x80\x83 T\x90Qc1<\xE5g`\xE0\x1B\x81R\x92\x96\x95\x92\x93\x90\x91\x90\x84\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x05OW`\xFFaG\xB1\x92aG\xB7\x95\x89\x91aFNWP\x16\x90aH]V[\x92aDRV[`\x01`\x01`\xFF\x1B\x03\x80\x83\x11aFJW\x81\x11a\x05KWa\x04I\x92\x93Pa@\xB0V[`\x01`\x01`\x7F\x1B\x03\x81\x11a\x02\xB8W\x90V[\x90`\x12\x03`\x12\x81\x11a1\xE3WaG\xFD\x90aH\x01V[\x02\x90V[`M\x81\x11a1\xE3W`\n\n\x90V[`\x12\x03`\x12\x81\x11a1\xE3Wa\x04I\x90aH\x01V[\x90`\x12\x03`\x12\x81\x11a1\xE3WaH8\x90aH\x01V[\x90\x04\x90V[\x81\x15aHGW\x04\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x90\x81\x15aH\x9BW`\x12\x03`\x12\x81\x11a1\xE3WaHx\x90aH\x01V[`\0\x19\x82\x01\x91\x82\x11a1\xE3WaH\x8D\x91aH=V[`\x01\x81\x01\x80\x91\x11a1\xE3W\x90V[PP`\0\x90V[g\xFF\0\0\0\0\0\0\0\x91\x90`\0\x90\x15aH\xF0WP`\x01\x90[`\0\x90\x15aH\xEAWP`\x01\x92[` \x1B\x92`\x0F`\xF8\x1B\x90`\xF8\x1B\x16\x90`\x0F`\xFC\x1B\x90`\xFC\x1B\x16\x17`\xC0\x1C\x16\x17\x17\x90V[\x92aH\xC7V[\x90aH\xBAV[\x93\x96\x95\x94\x91\x90aI\x08a*\x8D\x86a3\x9DV[aJ\xBEWaI\x16B\x86aJ\xE2V[\x80\x15aJ\xACWaI(aID\x91aU+V[\x85T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x85UV[\x80\x15aJ\x9AWaIVaIv\x91aU+V[\x84T`\x01`\x01`\x80\x1B\x03\x16`\x80\x91\x90\x91\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x84UV[a\x03\xE8\x80\x83\x10\x90\x83\x14\x17`\x01\x83\x11`\x01\x84\x14\x17\x16\x15aJ\x81WaI\x98\x82aJ\xD0V[`\x01\x84\x01\x80Ta\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x92\x90\x92\x1Ba\xFF\xFF`\xA0\x1B\x16\x91\x90\x91\x17\x81U\x91`\x01`\x01`\xA0\x1B\x03\x82\x16aI\xF0W[PPP`\x03\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x90PV[\x80\x87\x10\x90\x87\x14\x17`\x01\x87\x11`\x01\x88\x14\x17\x16\x15aJhW\x82\x91aJCa\x18\x80\x96\x97aJ>aJ_\x94`\x02`\x03\x98\x01\x90`\x01\x80`\xA0\x1B\x03\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90UV[aJ\xD0V[\x81Ta\xFF\xFF`\xB0\x1B\x19\x16`\xB0\x91\x90\x91\x1Ba\xFF\xFF`\xB0\x1B\x16\x17\x90UV[\x90\x84\x938aI\xC9V[`@Qc\xED\xDF\xE1\x19`\xE0\x1B\x81R`\x04\x81\x01\x87\x90R`$\x90\xFD[`@Qc\x97\x1B1\t`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x90\xFD[`@Qc(i\xC5\xF3`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x17O\xD4\x1B`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\xE90\xCE\xDF`\xE0\x1B\x81R`\x04\x90\xFD[b\x01\0\0\x81\x10\x15a\x02\xB8Wa\xFF\xFF\x16\x90V[d\x01\0\0\0\0\x82\x10\x15a\x02\xB8W`\x01\x01\x80Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x80\x92\x90\x92\x1Bc\xFF\xFF\xFF\xFF`\x80\x1B\x16\x91\x90\x91\x17\x90UV[\x90`\x01a\x18\x80\x92\x01\x90`\x01`\x01`\x80\x1B\x03\x80\x83T\x16\x90`\0\x83`\x0F\x0B\x13`\0\x14aK^WaKD\x92\x16\x90a4\xA9V[`\x01`\x01`\x80\x1B\x03\x16`\x01`\x01`\x80\x1B\x03\x19\x82T\x16\x17\x90UV[aKjaKD\x93a4YV[\x16\x90a4\xC4V[\x92\x91\x90\x15aL\x02WaK\xBEaK\xE8\x92a\x1A\xABaK\xA2aK\x92aK\xC8\x95aU+V[\x87T`\x01`\x01`\x80\x1B\x03\x16a4\xA9V[\x86T`\x01`\x01`\x80\x1B\x03\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x86UV[\x83T`\x80\x1Ca4\xC4V[\x82T`\x01`\x01`\x80\x1B\x03\x16`\x80\x91\x90\x91\x1B`\x01`\x01`\x80\x1B\x03\x19\x16\x17\x82UV[T`\x01`\x01`\x80\x1B\x03\x81\x16\x15aJ\xACW`\x80\x1C\x15aJ\x9AWV[aL+aK\xC8\x91a\x1A\xABaK\xA2aL\x1BaL5\x96aU+V[\x87T`\x01`\x01`\x80\x1B\x03\x16a4\xC4V[\x83T`\x80\x1Ca4\xA9V[aK\xE8V[c\xFF\xFF\xFF\xFF``\x82\x01Q\x16\x15\x15\x90\x81aLQWP\x90V[`\x01`\x01`\x80\x1B\x03\x91P`@\x01Q\x16\x15\x90V[\x81\x15\x80aMiW[aM<W\x82\x15\x80aMDW[aM<Wa\x04I\x92`\0\x92\x83\x92aL\x8E\x81aL:V[\x15aM\x1FWg\r\xE0\xB6\xB3\xA7d\0\0\x91[`\x01`\x01`\x80\x1B\x03aL\xB7\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x90\x81aM\x01W[PP` \x01QaL\xD7\x90`\x01`\x01`\x80\x1B\x03\x16a*AV[\x91\x82aL\xEFW[PPP\x80\x82\x11\x90\x82\x03\x02\x90\x03aU+V[aL\xF9\x93PaT\xC5V[8\x80\x80aL\xDEV[aL\xD7\x92\x96PaM\x17a*A\x92\x85` \x93aT\xC5V[\x96\x92PaL\xBFV[aM6a*A`@\x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91aL\x9EV[PPP`\0\x90V[P`\x01`\x01`\x80\x1B\x03aMa` \x83\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x15\x15aLxV[P`\x01`\x01`\x80\x1B\x03aM\x83\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x15\x15aLlV[\x91\x90\x80`\x0F\x0B\x90\x81\x15aNjW`\0aM\xB1a*A`@\x87\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x92\x13\x15aN%Wa\x04I\x91a\x1A\xAB\x91aM\xC9\x86aL:V[aN\x15W[`\x01`\x01`\x80\x1B\x03\x16aN\x0Fa*A` aN\0a\x1A\xAB\x86aM\xFAa*A\x8DQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87aU\x03V[\x98\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90aU\x03V[g\r\xE0\xB6\xB3\xA7d\0\0\x91PaM\xCEV[a\x04I\x91aN;a*Aa*Aa\x1A\xAB\x94a4YV[aNda*A` aN\0a\x1A\xAB\x86aN^a*A\x8DQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87aT\xC5V[\x90aT\xC5V[PP\x90P`\0\x90`\0\x90V[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15aN\xBEWPV[cn\x89\xEC\xA5`\xE0\x1B\x81R`\x04\x90\xFD[\x90\x91` \x90`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15aO\x10WPV[c:\xCB=?`\xE2\x1B\x81R`\x04\x90\xFD[\x91\x94\x92\x94`\0aO:`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16aT\xDDV[\x95\x80aPoW[P\x80\x95\x80\x97`\x80\x86\x01\x92aO\x8CaOX\x85Q\x15\x15\x90V[\x93\x84\x15aPhW\x87\x94[\x15aP^WaO\x87\x87\x95[aO\x81a*A\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a8\x10V[a8\x03V[\x90\x81\x81\x11aPLWaO\xA1` \x91\x85\x93a8\x03V[\x97\x01\x91aO\xB5\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11aP:WaO\xE3\x91aO\xD6a*AaO\xDC\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a8\x03V[\x91Q\x15\x15\x90V[\x90\x81\x15aP1W\x84\x85\x92[\x15aP)WP\x92[\x14aP\x17W\x81\x14aP\x05W\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92aO\xF6V[\x80\x94\x85\x92aO\xEEV[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[aO\x87\x88\x95aOmV[\x86\x94aObV[aPz\x91P\x86aH=V[\x94\x85\x81\x03\x90\x81\x11a1\xE3W\x948aOAV[aP\x94a7\x97V[P`@\x81\x01\x80Q\x90aP\xB0a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aG\xE8V[\x90R``\x82\x01aP\xC6\x81Q`\xFF\x84Q\x16\x90aG\xE8V[\x90RaP\xDD`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aG\xE8V[\x90R`\xA0\x81\x01aP\xF7\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aG\xE8V[\x90R\x90V[aQ\x04a7\x97V[P`@\x81\x01\x80Q\x90aQ a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90aH#V[\x90R``\x82\x01aQ6\x81Q`\xFF\x84Q\x16\x90aH#V[\x90RaQM`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90aH#V[\x90R`\xA0\x81\x01aP\xF7\x81Q`\xFFa\x01 \x85\x01Q\x16\x90aH#V[\x15aQnWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x90\xFD[\x15aQ\xABWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x11`$\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\x02\xB8WQa\x04I\x81a\x03\x08V[\x91\x92a\x04I\x96\x94\x91`\xA0\x94`\x01\x80\x87\x1B\x03\x80\x92\x16\x85R\x16` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x91a8\xD3V[\x15aR3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`D\x82\x01R`d\x90\xFD[\x15aRrWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\t\x88\xA9\xC8\xEA\x89\x0B\xE9\xA9*i\xA8*\x88i`\x8B\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a2?W`\x05\x1B\x01\x90V[\x81\x83R\x90\x91`\x01`\x01`\xFB\x1B\x03\x83\x11a\x02\xB8W` \x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x92\x90aR\xF6\x90a\x04I\x95\x93`@\x86R`@\x86\x01\x91aR\xB9V[\x92` \x81\x85\x03\x91\x01RaR\xB9V[\x96\x94\x92aSF\x94aS8\x92a\x04I\x9A\x98\x94`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x8BR\x16` \x8A\x01R`\xA0`@\x8A\x01R`\xA0\x89\x01\x91aR\xB9V[\x91\x86\x83\x03``\x88\x01RaR\xB9V[\x92`\x80\x81\x85\x03\x91\x01Ra8\xD3V[\x90\x92`\xA0\x92a\x04I\x95\x94`\x01\x80\x86\x1B\x03\x16\x83R`\0` \x84\x01R`@\x83\x01R``\x82\x01R\x81`\x80\x82\x01R\x01\x90a\x04\x13V[\x92\x90\x91aS\xA8\x83a\no\x86`\x01\x80`\xA0\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[aS\xB3\x82\x82Ta8\x10V[\x90U`@\x80Q\x84\x81R` \x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x86\x16\x95\x91\x86\x91`\0\x913\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x90\xA4;\x15aTOW\x90aT)\x93` \x92`\0`@Q\x80\x97\x81\x95\x82\x94c\xF2:na`\xE0\x1B\x99\x8A\x85R3`\x04\x86\x01aSTV[\x03\x92Z\xF1\x80\x15a\x05OWa\x18\x80\x92`\0\x91a\t\xEDWP`\x01`\x01`\xE0\x1B\x03\x19\x16\x14aR,V[PPPa\x18\x80\x90\x15\x15aQ\xA4V[`\x01\x80`\xA0\x1B\x03\x16\x90`\0\x92\x82\x84R\x83` R`@\x84 \x82\x85R` R`@\x84 \x80T\x91\x80\x83\x03\x92\x83\x11a1\xE3W\x91\x90U`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R3\x91\x7F\xC3\xD5\x81h\xC5\xAEs\x97s\x1D\x06=[\xBF=exTBsC\xF4\xC0\x83$\x0Fz\xAC\xAA-\x0Fb\x91\x90\xA4V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\xB8W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\xB8W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x91\x90\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\xB8W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01`\x80\x1B\x81\x10\x15a\x02\xB8W`\x01`\x01`\x80\x1B\x03\x16\x90V\xFE\xA2dipfsX\"\x12 \x14K\xAD\x9DW[\xEAk\x12\xB6\xCF\x8B\xF0\x84\x8E\\\x9A\xEFH\xF7\x18\xADY\x16\xD8QB\0\xF2k\xBB\xBDdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static PORTFOLIO_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct Portfolio<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Portfolio<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Portfolio<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Portfolio<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Portfolio<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Portfolio))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Portfolio<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PORTFOLIO_ABI.clone(),
                client,
            ))
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
                PORTFOLIO_ABI.clone(),
                PORTFOLIO_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `DEFAULT_STRATEGY` (0x531e17b3) function
        pub fn default_strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([83, 30, 23, 179], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `POSITION_RENDERER` (0xb0c3a950) function
        pub fn position_renderer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([176, 195, 169, 80], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `REGISTRY` (0x06433b1b) function
        pub fn registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([6, 67, 59, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `VERSION` (0xffa1ad74) function
        pub fn version(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([255, 161, 173, 116], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `WETH` (0xad5c4648) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([173, 92, 70, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allocate` (0x2f9e38e2) function
        pub fn allocate(
            &self,
            use_max: bool,
            recipient: ::ethers::core::types::Address,
            pool_id: u64,
            delta_liquidity: u128,
            max_delta_asset: u128,
            max_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [47, 158, 56, 226],
                    (
                        use_max,
                        recipient,
                        pool_id,
                        delta_liquidity,
                        max_delta_asset,
                        max_delta_quote,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOf` (0x00fdd58e) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([0, 253, 213, 142], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `balanceOfBatch` (0x4e1273f4) function
        pub fn balance_of_batch(
            &self,
            owners: ::std::vec::Vec<::ethers::core::types::Address>,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([78, 18, 115, 244], (owners, ids))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeParameters` (0x8a678967) function
        pub fn change_parameters(
            &self,
            pool_id: u64,
            priority_fee: u16,
            fee: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 103, 137, 103], (pool_id, priority_fee, fee))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimFee` (0xdda40797) function
        pub fn claim_fee(
            &self,
            token: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 164, 7, 151], (token, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPair` (0xc9c65396) function
        pub fn create_pair(
            &self,
            asset: ::ethers::core::types::Address,
            quote: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([201, 198, 83, 150], (asset, quote))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createPool` (0x267a0cfe) function
        pub fn create_pool(
            &self,
            pair_id: u32,
            reserve_x_per_wad: ::ethers::core::types::U256,
            reserve_y_per_wad: ::ethers::core::types::U256,
            fee_basis_points: u16,
            priority_fee_basis_points: u16,
            controller: ::ethers::core::types::Address,
            strategy: ::ethers::core::types::Address,
            strategy_args: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash(
                    [38, 122, 12, 254],
                    (
                        pair_id,
                        reserve_x_per_wad,
                        reserve_y_per_wad,
                        fee_basis_points,
                        priority_fee_basis_points,
                        controller,
                        strategy,
                        strategy_args,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0x5bc55464) function
        pub fn deallocate(
            &self,
            use_max: bool,
            pool_id: u64,
            delta_liquidity: u128,
            min_delta_asset: u128,
            min_delta_quote: u128,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [91, 197, 84, 100],
                    (
                        use_max,
                        pool_id,
                        delta_liquidity,
                        min_delta_asset,
                        min_delta_quote,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAmountOut` (0x19057807) function
        pub fn get_amount_out(
            &self,
            pool_id: u64,
            sell_asset: bool,
            amount_in: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 5, 120, 7], (pool_id, sell_asset, amount_in, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInvariant` (0x39434d5a) function
        pub fn get_invariant(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([57, 67, 77, 90], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLiquidityDeltas` (0x8992f20a) function
        pub fn get_liquidity_deltas(
            &self,
            pool_id: u64,
            delta_liquidity: i128,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u128)> {
            self.0
                .method_hash([137, 146, 242, 10], (pool_id, delta_liquidity))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxLiquidity` (0xd6b7dec5) function
        pub fn get_max_liquidity(
            &self,
            pool_id: u64,
            amount_0: ::ethers::core::types::U256,
            amount_1: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([214, 183, 222, 197], (pool_id, amount_0, amount_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMaxOrder` (0xf07b879e) function
        pub fn get_max_order(
            &self,
            pool_id: u64,
            sell_asset: bool,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, Order> {
            self.0
                .method_hash([240, 123, 135, 158], (pool_id, sell_asset, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNetBalance` (0x4dc68a90) function
        pub fn get_net_balance(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([77, 198, 138, 144], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairId` (0x3f92a339) function
        pub fn get_pair_id(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([63, 146, 163, 57], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPairNonce` (0x078888d6) function
        pub fn get_pair_nonce(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([7, 136, 136, 214], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolNonce` (0xa5cd8a49) function
        pub fn get_pool_nonce(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([165, 205, 138, 73], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolReserves` (0x2afb9df8) function
        pub fn get_pool_reserves(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([42, 251, 157, 248], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReserve` (0xc9a396e9) function
        pub fn get_reserve(
            &self,
            token: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([201, 163, 150, 233], token)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSpotPrice` (0xe331ba34) function
        pub fn get_spot_price(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 49, 186, 52], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStrategy` (0x30244be7) function
        pub fn get_strategy(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([48, 36, 75, 231], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0xac9650d8) function
        pub fn multicall(
            &self,
            data: ::std::vec::Vec<::ethers::core::types::Bytes>,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Bytes>,
        > {
            self.0
                .method_hash([172, 150, 80, 216], data)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairs` (0x5e47663c) function
        pub fn pairs(
            &self,
            p0: u32,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                u8,
                ::ethers::core::types::Address,
                u8,
            ),
        > {
            self.0
                .method_hash([94, 71, 102, 60], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0x89a5f084) function
        pub fn pools(
            &self,
            p0: u64,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u128,
                u128,
                u128,
                u32,
                u16,
                u16,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([137, 165, 240, 132], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFee` (0xb0e21e8a) function
        pub fn protocol_fee(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([176, 226, 30, 138], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFees` (0xdcf844a7) function
        pub fn protocol_fees(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 248, 68, 167], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeBatchTransferFrom` (0x2eb2c2d6) function
        pub fn safe_batch_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 178, 194, 214], (from, to, ids, amounts, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `safeTransferFrom` (0xf242432a) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            amount: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 66, 67, 42], (from, to, id, amount, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setProtocolFee` (0x787dce3d) function
        pub fn set_protocol_fee(
            &self,
            fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 125, 206, 61], fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x80af9d76) function
        pub fn simulate_swap(
            &self,
            order: Order,
            timestamp: ::ethers::core::types::U256,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
            ),
        > {
            self.0
                .method_hash([128, 175, 157, 118], (order, timestamp, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0xf33ae1bc) function
        pub fn swap(
            &self,
            args: Order,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u64,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([243, 58, 225, 188], (args,))
                .expect("method not found (this should never happen)")
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
        ///Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AllocateFilter> {
            self.0.event()
        }
        ///Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalForAllFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ChangeParameters` event
        pub fn change_parameters_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ChangeParametersFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ClaimFees` event
        pub fn claim_fees_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ClaimFeesFilter> {
            self.0.event()
        }
        ///Gets the contract's `CreatePair` event
        pub fn create_pair_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CreatePairFilter> {
            self.0.event()
        }
        ///Gets the contract's `CreatePool` event
        pub fn create_pool_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, CreatePoolFilter> {
            self.0.event()
        }
        ///Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeallocateFilter> {
            self.0.event()
        }
        ///Gets the contract's `DecreaseReserveBalance` event
        pub fn decrease_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DecreaseReserveBalanceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        ///Gets the contract's `IncreaseReserveBalance` event
        pub fn increase_reserve_balance_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, IncreaseReserveBalanceFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        ///Gets the contract's `TransferBatch` event
        pub fn transfer_batch_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferBatchFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `TransferSingle` event
        pub fn transfer_single_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferSingleFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `URI` event
        pub fn uri_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UriFilter> {
            self.0.event()
        }
        ///Gets the contract's `UpdateProtocolFee` event
        pub fn update_protocol_fee_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UpdateProtocolFeeFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PortfolioEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for Portfolio<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `EtherTransfer` with signature `EtherTransfer()` and selector `0x356594ab`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EtherTransfer", abi = "EtherTransfer()")]
    pub struct EtherTransfer;
    ///Custom Error type `InsufficientReserve` with signature `InsufficientReserve(uint256,uint256)` and selector `0x315276c9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "InsufficientReserve",
        abi = "InsufficientReserve(uint256,uint256)"
    )]
    pub struct InsufficientReserve {
        pub amount: ::ethers::core::types::U256,
        pub delta: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidBalance` with signature `InvalidBalance()` and selector `0xc52e3eff`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidBalance", abi = "InvalidBalance()")]
    pub struct InvalidBalance;
    ///Custom Error type `PoolLib_AlreadyCreated` with signature `PoolLib_AlreadyCreated()` and selector `0xe930cedf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PoolLib_AlreadyCreated", abi = "PoolLib_AlreadyCreated()")]
    pub struct PoolLib_AlreadyCreated;
    ///Custom Error type `PoolLib_InvalidFee` with signature `PoolLib_InvalidFee(uint256)` and selector `0x971b3109`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PoolLib_InvalidFee", abi = "PoolLib_InvalidFee(uint256)")]
    pub struct PoolLib_InvalidFee(pub ::ethers::core::types::U256);
    ///Custom Error type `PoolLib_InvalidPriorityFee` with signature `PoolLib_InvalidPriorityFee(uint256)` and selector `0xeddfe119`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "PoolLib_InvalidPriorityFee",
        abi = "PoolLib_InvalidPriorityFee(uint256)"
    )]
    pub struct PoolLib_InvalidPriorityFee(pub ::ethers::core::types::U256);
    ///Custom Error type `PoolLib_InvalidReserveX` with signature `PoolLib_InvalidReserveX()` and selector `0x5d3f506c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PoolLib_InvalidReserveX", abi = "PoolLib_InvalidReserveX()")]
    pub struct PoolLib_InvalidReserveX;
    ///Custom Error type `PoolLib_InvalidReserveY` with signature `PoolLib_InvalidReserveY()` and selector `0x2869c5f3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PoolLib_InvalidReserveY", abi = "PoolLib_InvalidReserveY()")]
    pub struct PoolLib_InvalidReserveY;
    ///Custom Error type `PoolLib_UpperLiquidityLimit` with signature `PoolLib_UpperLiquidityLimit()` and selector `0xacc9407b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "PoolLib_UpperLiquidityLimit",
        abi = "PoolLib_UpperLiquidityLimit()"
    )]
    pub struct PoolLib_UpperLiquidityLimit;
    ///Custom Error type `Portfolio_BeforeSwapFail` with signature `Portfolio_BeforeSwapFail()` and selector `0x2ed0ea01`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Portfolio_BeforeSwapFail", abi = "Portfolio_BeforeSwapFail()")]
    pub struct Portfolio_BeforeSwapFail;
    ///Custom Error type `Portfolio_DuplicateToken` with signature `Portfolio_DuplicateToken()` and selector `0x05134ccc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Portfolio_DuplicateToken", abi = "Portfolio_DuplicateToken()")]
    pub struct Portfolio_DuplicateToken;
    ///Custom Error type `Portfolio_Insolvent` with signature `Portfolio_Insolvent(address,int256)` and selector `0xaa269de2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_Insolvent",
        abi = "Portfolio_Insolvent(address,int256)"
    )]
    pub struct Portfolio_Insolvent {
        pub token: ::ethers::core::types::Address,
        pub net: ::ethers::core::types::I256,
    }
    ///Custom Error type `Portfolio_InsufficientLiquidity` with signature `Portfolio_InsufficientLiquidity()` and selector `0xcb6dab75`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InsufficientLiquidity",
        abi = "Portfolio_InsufficientLiquidity()"
    )]
    pub struct Portfolio_InsufficientLiquidity;
    ///Custom Error type `Portfolio_InvalidDecimals` with signature `Portfolio_InvalidDecimals(uint8)` and selector `0xc3da7747`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InvalidDecimals",
        abi = "Portfolio_InvalidDecimals(uint8)"
    )]
    pub struct Portfolio_InvalidDecimals {
        pub decimals: u8,
    }
    ///Custom Error type `Portfolio_InvalidInvariant` with signature `Portfolio_InvalidInvariant(int256,int256)` and selector `0xb93dee0a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InvalidInvariant",
        abi = "Portfolio_InvalidInvariant(int256,int256)"
    )]
    pub struct Portfolio_InvalidInvariant {
        pub prev: ::ethers::core::types::I256,
        pub next: ::ethers::core::types::I256,
    }
    ///Custom Error type `Portfolio_InvalidMulticall` with signature `Portfolio_InvalidMulticall()` and selector `0x55e1f7c5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InvalidMulticall",
        abi = "Portfolio_InvalidMulticall()"
    )]
    pub struct Portfolio_InvalidMulticall;
    ///Custom Error type `Portfolio_InvalidPairNonce` with signature `Portfolio_InvalidPairNonce()` and selector `0xcc7a739b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InvalidPairNonce",
        abi = "Portfolio_InvalidPairNonce()"
    )]
    pub struct Portfolio_InvalidPairNonce;
    ///Custom Error type `Portfolio_InvalidPool` with signature `Portfolio_InvalidPool(uint64)` and selector `0xbc27a517`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Portfolio_InvalidPool", abi = "Portfolio_InvalidPool(uint64)")]
    pub struct Portfolio_InvalidPool {
        pub pool_id: u64,
    }
    ///Custom Error type `Portfolio_InvalidProtocolFee` with signature `Portfolio_InvalidProtocolFee(uint256)` and selector `0x64597477`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InvalidProtocolFee",
        abi = "Portfolio_InvalidProtocolFee(uint256)"
    )]
    pub struct Portfolio_InvalidProtocolFee {
        pub protocol_fee: ::ethers::core::types::U256,
    }
    ///Custom Error type `Portfolio_InvalidReentrancy` with signature `Portfolio_InvalidReentrancy()` and selector `0x02b8002d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InvalidReentrancy",
        abi = "Portfolio_InvalidReentrancy()"
    )]
    pub struct Portfolio_InvalidReentrancy;
    ///Custom Error type `Portfolio_InvalidSettlement` with signature `Portfolio_InvalidSettlement()` and selector `0x326efa43`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_InvalidSettlement",
        abi = "Portfolio_InvalidSettlement()"
    )]
    pub struct Portfolio_InvalidSettlement;
    ///Custom Error type `Portfolio_MaxAssetExceeded` with signature `Portfolio_MaxAssetExceeded()` and selector `0xace41c3a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_MaxAssetExceeded",
        abi = "Portfolio_MaxAssetExceeded()"
    )]
    pub struct Portfolio_MaxAssetExceeded;
    ///Custom Error type `Portfolio_MaxQuoteExceeded` with signature `Portfolio_MaxQuoteExceeded()` and selector `0x84c05a5c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_MaxQuoteExceeded",
        abi = "Portfolio_MaxQuoteExceeded()"
    )]
    pub struct Portfolio_MaxQuoteExceeded;
    ///Custom Error type `Portfolio_MinAssetExceeded` with signature `Portfolio_MinAssetExceeded()` and selector `0x63448430`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_MinAssetExceeded",
        abi = "Portfolio_MinAssetExceeded()"
    )]
    pub struct Portfolio_MinAssetExceeded;
    ///Custom Error type `Portfolio_MinQuoteExceeded` with signature `Portfolio_MinQuoteExceeded()` and selector `0xacb5bdea`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_MinQuoteExceeded",
        abi = "Portfolio_MinQuoteExceeded()"
    )]
    pub struct Portfolio_MinQuoteExceeded;
    ///Custom Error type `Portfolio_NonExistentPool` with signature `Portfolio_NonExistentPool(uint64)` and selector `0x1216e0fa`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_NonExistentPool",
        abi = "Portfolio_NonExistentPool(uint64)"
    )]
    pub struct Portfolio_NonExistentPool {
        pub pool_id: u64,
    }
    ///Custom Error type `Portfolio_NotController` with signature `Portfolio_NotController()` and selector `0xffbe9c2c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Portfolio_NotController", abi = "Portfolio_NotController()")]
    pub struct Portfolio_NotController;
    ///Custom Error type `Portfolio_PairExists` with signature `Portfolio_PairExists(uint24)` and selector `0xb0988c43`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Portfolio_PairExists", abi = "Portfolio_PairExists(uint24)")]
    pub struct Portfolio_PairExists {
        pub pair_id: u32,
    }
    ///Custom Error type `Portfolio_ZeroAmountsAllocate` with signature `Portfolio_ZeroAmountsAllocate()` and selector `0x658b16ed`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_ZeroAmountsAllocate",
        abi = "Portfolio_ZeroAmountsAllocate()"
    )]
    pub struct Portfolio_ZeroAmountsAllocate;
    ///Custom Error type `Portfolio_ZeroLiquidityAllocate` with signature `Portfolio_ZeroLiquidityAllocate()` and selector `0x90609a7d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_ZeroLiquidityAllocate",
        abi = "Portfolio_ZeroLiquidityAllocate()"
    )]
    pub struct Portfolio_ZeroLiquidityAllocate;
    ///Custom Error type `Portfolio_ZeroLiquidityDeallocate` with signature `Portfolio_ZeroLiquidityDeallocate()` and selector `0x14ef605e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_ZeroLiquidityDeallocate",
        abi = "Portfolio_ZeroLiquidityDeallocate()"
    )]
    pub struct Portfolio_ZeroLiquidityDeallocate;
    ///Custom Error type `Portfolio_ZeroSwapInput` with signature `Portfolio_ZeroSwapInput()` and selector `0x13fd9b6d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Portfolio_ZeroSwapInput", abi = "Portfolio_ZeroSwapInput()")]
    pub struct Portfolio_ZeroSwapInput;
    ///Custom Error type `Portfolio_ZeroSwapLiquidity` with signature `Portfolio_ZeroSwapLiquidity()` and selector `0x7a95cb21`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "Portfolio_ZeroSwapLiquidity",
        abi = "Portfolio_ZeroSwapLiquidity()"
    )]
    pub struct Portfolio_ZeroSwapLiquidity;
    ///Custom Error type `Portfolio_ZeroSwapOutput` with signature `Portfolio_ZeroSwapOutput()` and selector `0x6f85b34e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Portfolio_ZeroSwapOutput", abi = "Portfolio_ZeroSwapOutput()")]
    pub struct Portfolio_ZeroSwapOutput;
    ///Custom Error type `SwapLib_OutputExceedsReserves` with signature `SwapLib_OutputExceedsReserves()` and selector `0x866a032b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "SwapLib_OutputExceedsReserves",
        abi = "SwapLib_OutputExceedsReserves()"
    )]
    pub struct SwapLib_OutputExceedsReserves;
    ///Custom Error type `SwapLib_ProtocolFeeTooHigh` with signature `SwapLib_ProtocolFeeTooHigh()` and selector `0xec8e1fce`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "SwapLib_ProtocolFeeTooHigh",
        abi = "SwapLib_ProtocolFeeTooHigh()"
    )]
    pub struct SwapLib_ProtocolFeeTooHigh;
    ///Custom Error type `SwapLib_ZeroXAdjustment` with signature `SwapLib_ZeroXAdjustment()` and selector `0x7276f08a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SwapLib_ZeroXAdjustment", abi = "SwapLib_ZeroXAdjustment()")]
    pub struct SwapLib_ZeroXAdjustment;
    ///Custom Error type `SwapLib_ZeroYAdjustment` with signature `SwapLib_ZeroYAdjustment()` and selector `0x1fb0b7dd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SwapLib_ZeroYAdjustment", abi = "SwapLib_ZeroYAdjustment()")]
    pub struct SwapLib_ZeroYAdjustment;
    ///Custom Error type `TokenTransfer` with signature `TokenTransfer()` and selector `0xeb2cf4fc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TokenTransfer", abi = "TokenTransfer()")]
    pub struct TokenTransfer;
    ///Custom Error type `TokenTransferFrom` with signature `TokenTransferFrom()` and selector `0x6e89eca5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TokenTransferFrom", abi = "TokenTransferFrom()")]
    pub struct TokenTransferFrom;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PortfolioErrors {
        EtherTransfer(EtherTransfer),
        InsufficientReserve(InsufficientReserve),
        InvalidBalance(InvalidBalance),
        PoolLib_AlreadyCreated(PoolLib_AlreadyCreated),
        PoolLib_InvalidFee(PoolLib_InvalidFee),
        PoolLib_InvalidPriorityFee(PoolLib_InvalidPriorityFee),
        PoolLib_InvalidReserveX(PoolLib_InvalidReserveX),
        PoolLib_InvalidReserveY(PoolLib_InvalidReserveY),
        PoolLib_UpperLiquidityLimit(PoolLib_UpperLiquidityLimit),
        Portfolio_BeforeSwapFail(Portfolio_BeforeSwapFail),
        Portfolio_DuplicateToken(Portfolio_DuplicateToken),
        Portfolio_Insolvent(Portfolio_Insolvent),
        Portfolio_InsufficientLiquidity(Portfolio_InsufficientLiquidity),
        Portfolio_InvalidDecimals(Portfolio_InvalidDecimals),
        Portfolio_InvalidInvariant(Portfolio_InvalidInvariant),
        Portfolio_InvalidMulticall(Portfolio_InvalidMulticall),
        Portfolio_InvalidPairNonce(Portfolio_InvalidPairNonce),
        Portfolio_InvalidPool(Portfolio_InvalidPool),
        Portfolio_InvalidProtocolFee(Portfolio_InvalidProtocolFee),
        Portfolio_InvalidReentrancy(Portfolio_InvalidReentrancy),
        Portfolio_InvalidSettlement(Portfolio_InvalidSettlement),
        Portfolio_MaxAssetExceeded(Portfolio_MaxAssetExceeded),
        Portfolio_MaxQuoteExceeded(Portfolio_MaxQuoteExceeded),
        Portfolio_MinAssetExceeded(Portfolio_MinAssetExceeded),
        Portfolio_MinQuoteExceeded(Portfolio_MinQuoteExceeded),
        Portfolio_NonExistentPool(Portfolio_NonExistentPool),
        Portfolio_NotController(Portfolio_NotController),
        Portfolio_PairExists(Portfolio_PairExists),
        Portfolio_ZeroAmountsAllocate(Portfolio_ZeroAmountsAllocate),
        Portfolio_ZeroLiquidityAllocate(Portfolio_ZeroLiquidityAllocate),
        Portfolio_ZeroLiquidityDeallocate(Portfolio_ZeroLiquidityDeallocate),
        Portfolio_ZeroSwapInput(Portfolio_ZeroSwapInput),
        Portfolio_ZeroSwapLiquidity(Portfolio_ZeroSwapLiquidity),
        Portfolio_ZeroSwapOutput(Portfolio_ZeroSwapOutput),
        SwapLib_OutputExceedsReserves(SwapLib_OutputExceedsReserves),
        SwapLib_ProtocolFeeTooHigh(SwapLib_ProtocolFeeTooHigh),
        SwapLib_ZeroXAdjustment(SwapLib_ZeroXAdjustment),
        SwapLib_ZeroYAdjustment(SwapLib_ZeroYAdjustment),
        TokenTransfer(TokenTransfer),
        TokenTransferFrom(TokenTransferFrom),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for PortfolioErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <EtherTransfer as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::EtherTransfer(decoded));
            }
            if let Ok(decoded) =
                <InsufficientReserve as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientReserve(decoded));
            }
            if let Ok(decoded) = <InvalidBalance as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidBalance(decoded));
            }
            if let Ok(decoded) =
                <PoolLib_AlreadyCreated as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolLib_AlreadyCreated(decoded));
            }
            if let Ok(decoded) =
                <PoolLib_InvalidFee as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolLib_InvalidFee(decoded));
            }
            if let Ok(decoded) =
                <PoolLib_InvalidPriorityFee as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolLib_InvalidPriorityFee(decoded));
            }
            if let Ok(decoded) =
                <PoolLib_InvalidReserveX as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolLib_InvalidReserveX(decoded));
            }
            if let Ok(decoded) =
                <PoolLib_InvalidReserveY as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolLib_InvalidReserveY(decoded));
            }
            if let Ok(decoded) =
                <PoolLib_UpperLiquidityLimit as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PoolLib_UpperLiquidityLimit(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_BeforeSwapFail as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_BeforeSwapFail(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_DuplicateToken as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_DuplicateToken(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_Insolvent as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_Insolvent(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InsufficientLiquidity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InsufficientLiquidity(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidDecimals as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidDecimals(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidInvariant as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidInvariant(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidMulticall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidMulticall(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidPairNonce as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidPairNonce(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidPool as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidPool(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidProtocolFee as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidProtocolFee(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidReentrancy as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidReentrancy(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_InvalidSettlement as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_InvalidSettlement(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_MaxAssetExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_MaxAssetExceeded(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_MaxQuoteExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_MaxQuoteExceeded(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_MinAssetExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_MinAssetExceeded(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_MinQuoteExceeded as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_MinQuoteExceeded(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_NonExistentPool as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_NonExistentPool(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_NotController as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_NotController(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_PairExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_PairExists(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_ZeroAmountsAllocate as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_ZeroAmountsAllocate(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_ZeroLiquidityAllocate as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_ZeroLiquidityAllocate(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_ZeroLiquidityDeallocate as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_ZeroLiquidityDeallocate(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_ZeroSwapInput as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_ZeroSwapInput(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_ZeroSwapLiquidity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_ZeroSwapLiquidity(decoded));
            }
            if let Ok(decoded) =
                <Portfolio_ZeroSwapOutput as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Portfolio_ZeroSwapOutput(decoded));
            }
            if let Ok(decoded) =
                <SwapLib_OutputExceedsReserves as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapLib_OutputExceedsReserves(decoded));
            }
            if let Ok(decoded) =
                <SwapLib_ProtocolFeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapLib_ProtocolFeeTooHigh(decoded));
            }
            if let Ok(decoded) =
                <SwapLib_ZeroXAdjustment as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapLib_ZeroXAdjustment(decoded));
            }
            if let Ok(decoded) =
                <SwapLib_ZeroYAdjustment as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SwapLib_ZeroYAdjustment(decoded));
            }
            if let Ok(decoded) = <TokenTransfer as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenTransfer(decoded));
            }
            if let Ok(decoded) = <TokenTransferFrom as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TokenTransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PortfolioErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::EtherTransfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientReserve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PoolLib_AlreadyCreated(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidPriorityFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_InvalidReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolLib_UpperLiquidityLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_BeforeSwapFail(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_DuplicateToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_Insolvent(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InsufficientLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidMulticall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidPairNonce(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidProtocolFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidReentrancy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_InvalidSettlement(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MaxAssetExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MaxQuoteExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MinAssetExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_MinQuoteExceeded(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_NonExistentPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_NotController(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_PairExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroAmountsAllocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroLiquidityAllocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroLiquidityDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroSwapInput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroSwapLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio_ZeroSwapOutput(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ZeroXAdjustment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapLib_ZeroYAdjustment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for PortfolioErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <EtherTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientReserve as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidBalance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_AlreadyCreated as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidPriorityFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidReserveX as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_InvalidReserveY as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PoolLib_UpperLiquidityLimit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_BeforeSwapFail as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_DuplicateToken as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_Insolvent as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InsufficientLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidDecimals as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidInvariant as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidMulticall as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidPairNonce as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidProtocolFee as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidReentrancy as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_InvalidSettlement as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MaxAssetExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MaxQuoteExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MinAssetExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_MinQuoteExceeded as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_NonExistentPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_NotController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_PairExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroAmountsAllocate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroLiquidityAllocate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroLiquidityDeallocate as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroSwapInput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroSwapLiquidity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Portfolio_ZeroSwapOutput as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_OutputExceedsReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ProtocolFeeTooHigh as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ZeroXAdjustment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SwapLib_ZeroYAdjustment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransferFrom as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for PortfolioErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EtherTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLib_AlreadyCreated(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLib_InvalidFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLib_InvalidPriorityFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLib_InvalidReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLib_InvalidReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolLib_UpperLiquidityLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_BeforeSwapFail(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_DuplicateToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_Insolvent(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_InsufficientLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_InvalidInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_InvalidMulticall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_InvalidPairNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_InvalidPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_InvalidProtocolFee(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_InvalidReentrancy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_InvalidSettlement(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_MaxAssetExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_MaxQuoteExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_MinAssetExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_MinQuoteExceeded(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_NonExistentPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_NotController(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_PairExists(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_ZeroAmountsAllocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroLiquidityAllocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroLiquidityDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Portfolio_ZeroSwapInput(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_ZeroSwapLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio_ZeroSwapOutput(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_ZeroXAdjustment(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_ZeroYAdjustment(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for PortfolioErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<EtherTransfer> for PortfolioErrors {
        fn from(value: EtherTransfer) -> Self {
            Self::EtherTransfer(value)
        }
    }
    impl ::core::convert::From<InsufficientReserve> for PortfolioErrors {
        fn from(value: InsufficientReserve) -> Self {
            Self::InsufficientReserve(value)
        }
    }
    impl ::core::convert::From<InvalidBalance> for PortfolioErrors {
        fn from(value: InvalidBalance) -> Self {
            Self::InvalidBalance(value)
        }
    }
    impl ::core::convert::From<PoolLib_AlreadyCreated> for PortfolioErrors {
        fn from(value: PoolLib_AlreadyCreated) -> Self {
            Self::PoolLib_AlreadyCreated(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidFee> for PortfolioErrors {
        fn from(value: PoolLib_InvalidFee) -> Self {
            Self::PoolLib_InvalidFee(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidPriorityFee> for PortfolioErrors {
        fn from(value: PoolLib_InvalidPriorityFee) -> Self {
            Self::PoolLib_InvalidPriorityFee(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidReserveX> for PortfolioErrors {
        fn from(value: PoolLib_InvalidReserveX) -> Self {
            Self::PoolLib_InvalidReserveX(value)
        }
    }
    impl ::core::convert::From<PoolLib_InvalidReserveY> for PortfolioErrors {
        fn from(value: PoolLib_InvalidReserveY) -> Self {
            Self::PoolLib_InvalidReserveY(value)
        }
    }
    impl ::core::convert::From<PoolLib_UpperLiquidityLimit> for PortfolioErrors {
        fn from(value: PoolLib_UpperLiquidityLimit) -> Self {
            Self::PoolLib_UpperLiquidityLimit(value)
        }
    }
    impl ::core::convert::From<Portfolio_BeforeSwapFail> for PortfolioErrors {
        fn from(value: Portfolio_BeforeSwapFail) -> Self {
            Self::Portfolio_BeforeSwapFail(value)
        }
    }
    impl ::core::convert::From<Portfolio_DuplicateToken> for PortfolioErrors {
        fn from(value: Portfolio_DuplicateToken) -> Self {
            Self::Portfolio_DuplicateToken(value)
        }
    }
    impl ::core::convert::From<Portfolio_Insolvent> for PortfolioErrors {
        fn from(value: Portfolio_Insolvent) -> Self {
            Self::Portfolio_Insolvent(value)
        }
    }
    impl ::core::convert::From<Portfolio_InsufficientLiquidity> for PortfolioErrors {
        fn from(value: Portfolio_InsufficientLiquidity) -> Self {
            Self::Portfolio_InsufficientLiquidity(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidDecimals> for PortfolioErrors {
        fn from(value: Portfolio_InvalidDecimals) -> Self {
            Self::Portfolio_InvalidDecimals(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidInvariant> for PortfolioErrors {
        fn from(value: Portfolio_InvalidInvariant) -> Self {
            Self::Portfolio_InvalidInvariant(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidMulticall> for PortfolioErrors {
        fn from(value: Portfolio_InvalidMulticall) -> Self {
            Self::Portfolio_InvalidMulticall(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidPairNonce> for PortfolioErrors {
        fn from(value: Portfolio_InvalidPairNonce) -> Self {
            Self::Portfolio_InvalidPairNonce(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidPool> for PortfolioErrors {
        fn from(value: Portfolio_InvalidPool) -> Self {
            Self::Portfolio_InvalidPool(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidProtocolFee> for PortfolioErrors {
        fn from(value: Portfolio_InvalidProtocolFee) -> Self {
            Self::Portfolio_InvalidProtocolFee(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidReentrancy> for PortfolioErrors {
        fn from(value: Portfolio_InvalidReentrancy) -> Self {
            Self::Portfolio_InvalidReentrancy(value)
        }
    }
    impl ::core::convert::From<Portfolio_InvalidSettlement> for PortfolioErrors {
        fn from(value: Portfolio_InvalidSettlement) -> Self {
            Self::Portfolio_InvalidSettlement(value)
        }
    }
    impl ::core::convert::From<Portfolio_MaxAssetExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MaxAssetExceeded) -> Self {
            Self::Portfolio_MaxAssetExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_MaxQuoteExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MaxQuoteExceeded) -> Self {
            Self::Portfolio_MaxQuoteExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_MinAssetExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MinAssetExceeded) -> Self {
            Self::Portfolio_MinAssetExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_MinQuoteExceeded> for PortfolioErrors {
        fn from(value: Portfolio_MinQuoteExceeded) -> Self {
            Self::Portfolio_MinQuoteExceeded(value)
        }
    }
    impl ::core::convert::From<Portfolio_NonExistentPool> for PortfolioErrors {
        fn from(value: Portfolio_NonExistentPool) -> Self {
            Self::Portfolio_NonExistentPool(value)
        }
    }
    impl ::core::convert::From<Portfolio_NotController> for PortfolioErrors {
        fn from(value: Portfolio_NotController) -> Self {
            Self::Portfolio_NotController(value)
        }
    }
    impl ::core::convert::From<Portfolio_PairExists> for PortfolioErrors {
        fn from(value: Portfolio_PairExists) -> Self {
            Self::Portfolio_PairExists(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroAmountsAllocate> for PortfolioErrors {
        fn from(value: Portfolio_ZeroAmountsAllocate) -> Self {
            Self::Portfolio_ZeroAmountsAllocate(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroLiquidityAllocate> for PortfolioErrors {
        fn from(value: Portfolio_ZeroLiquidityAllocate) -> Self {
            Self::Portfolio_ZeroLiquidityAllocate(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroLiquidityDeallocate> for PortfolioErrors {
        fn from(value: Portfolio_ZeroLiquidityDeallocate) -> Self {
            Self::Portfolio_ZeroLiquidityDeallocate(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroSwapInput> for PortfolioErrors {
        fn from(value: Portfolio_ZeroSwapInput) -> Self {
            Self::Portfolio_ZeroSwapInput(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroSwapLiquidity> for PortfolioErrors {
        fn from(value: Portfolio_ZeroSwapLiquidity) -> Self {
            Self::Portfolio_ZeroSwapLiquidity(value)
        }
    }
    impl ::core::convert::From<Portfolio_ZeroSwapOutput> for PortfolioErrors {
        fn from(value: Portfolio_ZeroSwapOutput) -> Self {
            Self::Portfolio_ZeroSwapOutput(value)
        }
    }
    impl ::core::convert::From<SwapLib_OutputExceedsReserves> for PortfolioErrors {
        fn from(value: SwapLib_OutputExceedsReserves) -> Self {
            Self::SwapLib_OutputExceedsReserves(value)
        }
    }
    impl ::core::convert::From<SwapLib_ProtocolFeeTooHigh> for PortfolioErrors {
        fn from(value: SwapLib_ProtocolFeeTooHigh) -> Self {
            Self::SwapLib_ProtocolFeeTooHigh(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroXAdjustment> for PortfolioErrors {
        fn from(value: SwapLib_ZeroXAdjustment) -> Self {
            Self::SwapLib_ZeroXAdjustment(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroYAdjustment> for PortfolioErrors {
        fn from(value: SwapLib_ZeroYAdjustment) -> Self {
            Self::SwapLib_ZeroYAdjustment(value)
        }
    }
    impl ::core::convert::From<TokenTransfer> for PortfolioErrors {
        fn from(value: TokenTransfer) -> Self {
            Self::TokenTransfer(value)
        }
    }
    impl ::core::convert::From<TokenTransferFrom> for PortfolioErrors {
        fn from(value: TokenTransferFrom) -> Self {
            Self::TokenTransferFrom(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Allocate",
        abi = "Allocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ChangeParameters",
        abi = "ChangeParameters(uint64,uint16,uint16)"
    )]
    pub struct ChangeParametersFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub priority_fee: u16,
        #[ethevent(indexed)]
        pub fee: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ClaimFees", abi = "ClaimFees(address,uint256)")]
    pub struct ClaimFeesFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "CreatePair",
        abi = "CreatePair(uint24,address,address,uint8,uint8)"
    )]
    pub struct CreatePairFilter {
        #[ethevent(indexed)]
        pub pair_id: u32,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub decimals_quote: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "CreatePool",
        abi = "CreatePool(uint64,address,address,uint256,uint256,uint16,uint16,address,address)"
    )]
    pub struct CreatePoolFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub reserve_x_per_wad: ::ethers::core::types::U256,
        pub reserve_y_per_wad: ::ethers::core::types::U256,
        pub fee_basis_points: u16,
        pub priority_fee_basis_points: u16,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Deallocate",
        abi = "Deallocate(uint64,address,address,uint256,uint256,uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        #[ethevent(indexed)]
        pub asset: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub quote: ::ethers::core::types::Address,
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "DecreaseReserveBalance",
        abi = "DecreaseReserveBalance(address,uint256)"
    )]
    pub struct DecreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "IncreaseReserveBalance",
        abi = "IncreaseReserveBalance(address,uint256)"
    )]
    pub struct IncreaseReserveBalanceFilter {
        #[ethevent(indexed)]
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(uint64,bool,address,uint256,address,uint256,uint256,int256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub pool_id: u64,
        pub sell_asset: bool,
        #[ethevent(indexed)]
        pub token_in: ::ethers::core::types::Address,
        pub input: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub token_out: ::ethers::core::types::Address,
        pub output: ::ethers::core::types::U256,
        pub fee_amount_dec: ::ethers::core::types::U256,
        pub invariant_wad: ::ethers::core::types::I256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "TransferBatch",
        abi = "TransferBatch(address,address,address,uint256[],uint256[])"
    )]
    pub struct TransferBatchFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "TransferSingle",
        abi = "TransferSingle(address,address,address,uint256,uint256)"
    )]
    pub struct TransferSingleFilter {
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "URI", abi = "URI(string,uint256)")]
    pub struct UriFilter {
        pub value: ::std::string::String,
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "UpdateProtocolFee", abi = "UpdateProtocolFee(uint256,uint256)")]
    pub struct UpdateProtocolFeeFilter {
        pub prev_fee: ::ethers::core::types::U256,
        pub next_fee: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PortfolioEvents {
        AllocateFilter(AllocateFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        ChangeParametersFilter(ChangeParametersFilter),
        ClaimFeesFilter(ClaimFeesFilter),
        CreatePairFilter(CreatePairFilter),
        CreatePoolFilter(CreatePoolFilter),
        DeallocateFilter(DeallocateFilter),
        DecreaseReserveBalanceFilter(DecreaseReserveBalanceFilter),
        DepositFilter(DepositFilter),
        IncreaseReserveBalanceFilter(IncreaseReserveBalanceFilter),
        SwapFilter(SwapFilter),
        TransferBatchFilter(TransferBatchFilter),
        TransferSingleFilter(TransferSingleFilter),
        UriFilter(UriFilter),
        UpdateProtocolFeeFilter(UpdateProtocolFeeFilter),
    }
    impl ::ethers::contract::EthLogDecode for PortfolioEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(PortfolioEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(PortfolioEvents::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = ChangeParametersFilter::decode_log(log) {
                return Ok(PortfolioEvents::ChangeParametersFilter(decoded));
            }
            if let Ok(decoded) = ClaimFeesFilter::decode_log(log) {
                return Ok(PortfolioEvents::ClaimFeesFilter(decoded));
            }
            if let Ok(decoded) = CreatePairFilter::decode_log(log) {
                return Ok(PortfolioEvents::CreatePairFilter(decoded));
            }
            if let Ok(decoded) = CreatePoolFilter::decode_log(log) {
                return Ok(PortfolioEvents::CreatePoolFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(PortfolioEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = DecreaseReserveBalanceFilter::decode_log(log) {
                return Ok(PortfolioEvents::DecreaseReserveBalanceFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(PortfolioEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = IncreaseReserveBalanceFilter::decode_log(log) {
                return Ok(PortfolioEvents::IncreaseReserveBalanceFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(PortfolioEvents::SwapFilter(decoded));
            }
            if let Ok(decoded) = TransferBatchFilter::decode_log(log) {
                return Ok(PortfolioEvents::TransferBatchFilter(decoded));
            }
            if let Ok(decoded) = TransferSingleFilter::decode_log(log) {
                return Ok(PortfolioEvents::TransferSingleFilter(decoded));
            }
            if let Ok(decoded) = UriFilter::decode_log(log) {
                return Ok(PortfolioEvents::UriFilter(decoded));
            }
            if let Ok(decoded) = UpdateProtocolFeeFilter::decode_log(log) {
                return Ok(PortfolioEvents::UpdateProtocolFeeFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for PortfolioEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeParametersFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFeesFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePairFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePoolFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DecreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncreaseReserveBalanceFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferBatchFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferSingleFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UriFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpdateProtocolFeeFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for PortfolioEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for PortfolioEvents {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<ChangeParametersFilter> for PortfolioEvents {
        fn from(value: ChangeParametersFilter) -> Self {
            Self::ChangeParametersFilter(value)
        }
    }
    impl ::core::convert::From<ClaimFeesFilter> for PortfolioEvents {
        fn from(value: ClaimFeesFilter) -> Self {
            Self::ClaimFeesFilter(value)
        }
    }
    impl ::core::convert::From<CreatePairFilter> for PortfolioEvents {
        fn from(value: CreatePairFilter) -> Self {
            Self::CreatePairFilter(value)
        }
    }
    impl ::core::convert::From<CreatePoolFilter> for PortfolioEvents {
        fn from(value: CreatePoolFilter) -> Self {
            Self::CreatePoolFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for PortfolioEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<DecreaseReserveBalanceFilter> for PortfolioEvents {
        fn from(value: DecreaseReserveBalanceFilter) -> Self {
            Self::DecreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for PortfolioEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<IncreaseReserveBalanceFilter> for PortfolioEvents {
        fn from(value: IncreaseReserveBalanceFilter) -> Self {
            Self::IncreaseReserveBalanceFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for PortfolioEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    impl ::core::convert::From<TransferBatchFilter> for PortfolioEvents {
        fn from(value: TransferBatchFilter) -> Self {
            Self::TransferBatchFilter(value)
        }
    }
    impl ::core::convert::From<TransferSingleFilter> for PortfolioEvents {
        fn from(value: TransferSingleFilter) -> Self {
            Self::TransferSingleFilter(value)
        }
    }
    impl ::core::convert::From<UriFilter> for PortfolioEvents {
        fn from(value: UriFilter) -> Self {
            Self::UriFilter(value)
        }
    }
    impl ::core::convert::From<UpdateProtocolFeeFilter> for PortfolioEvents {
        fn from(value: UpdateProtocolFeeFilter) -> Self {
            Self::UpdateProtocolFeeFilter(value)
        }
    }
    ///Container type for all input parameters for the `DEFAULT_STRATEGY` function with signature `DEFAULT_STRATEGY()` and selector `0x531e17b3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "DEFAULT_STRATEGY", abi = "DEFAULT_STRATEGY()")]
    pub struct DefaultStrategyCall;
    ///Container type for all input parameters for the `POSITION_RENDERER` function with signature `POSITION_RENDERER()` and selector `0xb0c3a950`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "POSITION_RENDERER", abi = "POSITION_RENDERER()")]
    pub struct PositionRendererCall;
    ///Container type for all input parameters for the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "REGISTRY", abi = "REGISTRY()")]
    pub struct RegistryCall;
    ///Container type for all input parameters for the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "VERSION", abi = "VERSION()")]
    pub struct VersionCall;
    ///Container type for all input parameters for the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "WETH", abi = "WETH()")]
    pub struct WethCall;
    ///Container type for all input parameters for the `allocate` function with signature `allocate(bool,address,uint64,uint128,uint128,uint128)` and selector `0x2f9e38e2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "allocate",
        abi = "allocate(bool,address,uint64,uint128,uint128,uint128)"
    )]
    pub struct AllocateCall {
        pub use_max: bool,
        pub recipient: ::ethers::core::types::Address,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub max_delta_asset: u128,
        pub max_delta_quote: u128,
    }
    ///Container type for all input parameters for the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOf", abi = "balanceOf(address,uint256)")]
    pub struct BalanceOfCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all input parameters for the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `0x4e1273f4`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "balanceOfBatch", abi = "balanceOfBatch(address[],uint256[])")]
    pub struct BalanceOfBatchCall {
        pub owners: ::std::vec::Vec<::ethers::core::types::Address>,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `changeParameters` function with signature `changeParameters(uint64,uint16,uint16)` and selector `0x8a678967`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "changeParameters",
        abi = "changeParameters(uint64,uint16,uint16)"
    )]
    pub struct ChangeParametersCall {
        pub pool_id: u64,
        pub priority_fee: u16,
        pub fee: u16,
    }
    ///Container type for all input parameters for the `claimFee` function with signature `claimFee(address,uint256)` and selector `0xdda40797`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "claimFee", abi = "claimFee(address,uint256)")]
    pub struct ClaimFeeCall {
        pub token: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "createPair", abi = "createPair(address,address)")]
    pub struct CreatePairCall {
        pub asset: ::ethers::core::types::Address,
        pub quote: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `createPool` function with signature `createPool(uint24,uint256,uint256,uint16,uint16,address,address,bytes)` and selector `0x267a0cfe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "createPool",
        abi = "createPool(uint24,uint256,uint256,uint16,uint16,address,address,bytes)"
    )]
    pub struct CreatePoolCall {
        pub pair_id: u32,
        pub reserve_x_per_wad: ::ethers::core::types::U256,
        pub reserve_y_per_wad: ::ethers::core::types::U256,
        pub fee_basis_points: u16,
        pub priority_fee_basis_points: u16,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub strategy_args: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "deallocate",
        abi = "deallocate(bool,uint64,uint128,uint128,uint128)"
    )]
    pub struct DeallocateCall {
        pub use_max: bool,
        pub pool_id: u64,
        pub delta_liquidity: u128,
        pub min_delta_asset: u128,
        pub min_delta_quote: u128,
    }
    ///Container type for all input parameters for the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getAmountOut",
        abi = "getAmountOut(uint64,bool,uint256,address)"
    )]
    pub struct GetAmountOutCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub amount_in: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getInvariant", abi = "getInvariant(uint64)")]
    pub struct GetInvariantCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getLiquidityDeltas", abi = "getLiquidityDeltas(uint64,int128)")]
    pub struct GetLiquidityDeltasCall {
        pub pool_id: u64,
        pub delta_liquidity: i128,
    }
    ///Container type for all input parameters for the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getMaxLiquidity",
        abi = "getMaxLiquidity(uint64,uint256,uint256)"
    )]
    pub struct GetMaxLiquidityCall {
        pub pool_id: u64,
        pub amount_0: ::ethers::core::types::U256,
        pub amount_1: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getMaxOrder` function with signature `getMaxOrder(uint64,bool,address)` and selector `0xf07b879e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getMaxOrder", abi = "getMaxOrder(uint64,bool,address)")]
    pub struct GetMaxOrderCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getNetBalance", abi = "getNetBalance(address)")]
    pub struct GetNetBalanceCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPairId", abi = "getPairId(address,address)")]
    pub struct GetPairIdCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPairNonce", abi = "getPairNonce()")]
    pub struct GetPairNonceCall;
    ///Container type for all input parameters for the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPoolNonce", abi = "getPoolNonce(uint24)")]
    pub struct GetPoolNonceCall(pub u32);
    ///Container type for all input parameters for the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getPoolReserves", abi = "getPoolReserves(uint64)")]
    pub struct GetPoolReservesCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getReserve", abi = "getReserve(address)")]
    pub struct GetReserveCall {
        pub token: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getSpotPrice", abi = "getSpotPrice(uint64)")]
    pub struct GetSpotPriceCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `getStrategy` function with signature `getStrategy(uint64)` and selector `0x30244be7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "getStrategy", abi = "getStrategy(uint64)")]
    pub struct GetStrategyCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "multicall", abi = "multicall(bytes[])")]
    pub struct MulticallCall {
        pub data: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all input parameters for the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pairs", abi = "pairs(uint24)")]
    pub struct PairsCall(pub u32);
    ///Container type for all input parameters for the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pools", abi = "pools(uint64)")]
    pub struct PoolsCall(pub u64);
    ///Container type for all input parameters for the `protocolFee` function with signature `protocolFee()` and selector `0xb0e21e8a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "protocolFee", abi = "protocolFee()")]
    pub struct ProtocolFeeCall;
    ///Container type for all input parameters for the `protocolFees` function with signature `protocolFees(address)` and selector `0xdcf844a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "protocolFees", abi = "protocolFees(address)")]
    pub struct ProtocolFeesCall(pub ::ethers::core::types::Address);
    ///Container type for all input parameters for the `safeBatchTransferFrom` function with signature `safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)` and selector `0x2eb2c2d6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "safeBatchTransferFrom",
        abi = "safeBatchTransferFrom(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct SafeBatchTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `safeTransferFrom` function with signature `safeTransferFrom(address,address,uint256,uint256,bytes)` and selector `0xf242432a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,uint256,bytes)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `setApprovalForAll` function with signature `setApprovalForAll(address,bool)` and selector `0xa22cb465`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    ///Container type for all input parameters for the `setProtocolFee` function with signature `setProtocolFee(uint256)` and selector `0x787dce3d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setProtocolFee", abi = "setProtocolFee(uint256)")]
    pub struct SetProtocolFeeCall {
        pub fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)` and selector `0x80af9d76`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "simulateSwap",
        abi = "simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)"
    )]
    pub struct SimulateSwapCall {
        pub order: Order,
        pub timestamp: ::ethers::core::types::U256,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `swap` function with signature `swap((uint128,uint128,bool,uint64,bool))` and selector `0xf33ae1bc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "swap", abi = "swap((uint128,uint128,bool,uint64,bool))")]
    pub struct SwapCall {
        pub args: Order,
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
        Hash,
    )]
    #[ethcall(name = "uri", abi = "uri(uint256)")]
    pub struct UriCall {
        pub id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum PortfolioCalls {
        DefaultStrategy(DefaultStrategyCall),
        PositionRenderer(PositionRendererCall),
        Registry(RegistryCall),
        Version(VersionCall),
        Weth(WethCall),
        Allocate(AllocateCall),
        BalanceOf(BalanceOfCall),
        BalanceOfBatch(BalanceOfBatchCall),
        ChangeParameters(ChangeParametersCall),
        ClaimFee(ClaimFeeCall),
        CreatePair(CreatePairCall),
        CreatePool(CreatePoolCall),
        Deallocate(DeallocateCall),
        GetAmountOut(GetAmountOutCall),
        GetInvariant(GetInvariantCall),
        GetLiquidityDeltas(GetLiquidityDeltasCall),
        GetMaxLiquidity(GetMaxLiquidityCall),
        GetMaxOrder(GetMaxOrderCall),
        GetNetBalance(GetNetBalanceCall),
        GetPairId(GetPairIdCall),
        GetPairNonce(GetPairNonceCall),
        GetPoolNonce(GetPoolNonceCall),
        GetPoolReserves(GetPoolReservesCall),
        GetReserve(GetReserveCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategy(GetStrategyCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Multicall(MulticallCall),
        Pairs(PairsCall),
        Pools(PoolsCall),
        ProtocolFee(ProtocolFeeCall),
        ProtocolFees(ProtocolFeesCall),
        SafeBatchTransferFrom(SafeBatchTransferFromCall),
        SafeTransferFrom(SafeTransferFromCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SetProtocolFee(SetProtocolFeeCall),
        SimulateSwap(SimulateSwapCall),
        SupportsInterface(SupportsInterfaceCall),
        Swap(SwapCall),
        Uri(UriCall),
    }
    impl ::ethers::core::abi::AbiDecode for PortfolioCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DefaultStrategyCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DefaultStrategy(decoded));
            }
            if let Ok(decoded) =
                <PositionRendererCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PositionRenderer(decoded));
            }
            if let Ok(decoded) = <RegistryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Registry(decoded));
            }
            if let Ok(decoded) = <VersionCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Version(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            if let Ok(decoded) = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) =
                <BalanceOfBatchCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BalanceOfBatch(decoded));
            }
            if let Ok(decoded) =
                <ChangeParametersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChangeParameters(decoded));
            }
            if let Ok(decoded) = <ClaimFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ClaimFee(decoded));
            }
            if let Ok(decoded) = <CreatePairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePair(decoded));
            }
            if let Ok(decoded) = <CreatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CreatePool(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded) =
                <GetLiquidityDeltasCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetLiquidityDeltas(decoded));
            }
            if let Ok(decoded) =
                <GetMaxLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetMaxLiquidity(decoded));
            }
            if let Ok(decoded) = <GetMaxOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxOrder(decoded));
            }
            if let Ok(decoded) = <GetNetBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNetBalance(decoded));
            }
            if let Ok(decoded) = <GetPairIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPairId(decoded));
            }
            if let Ok(decoded) = <GetPairNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPairNonce(decoded));
            }
            if let Ok(decoded) = <GetPoolNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolNonce(decoded));
            }
            if let Ok(decoded) =
                <GetPoolReservesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolReserves(decoded));
            }
            if let Ok(decoded) = <GetReserveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetReserve(decoded));
            }
            if let Ok(decoded) = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded) = <GetStrategyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetStrategy(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded) = <PairsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pairs(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <ProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProtocolFee(decoded));
            }
            if let Ok(decoded) = <ProtocolFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProtocolFees(decoded));
            }
            if let Ok(decoded) =
                <SafeBatchTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeBatchTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SetProtocolFeeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetProtocolFee(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <UriCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Uri(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for PortfolioCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DefaultStrategy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PositionRenderer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Registry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Version(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOfBatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChangeParameters(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreatePair(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CreatePool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deallocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInvariant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetLiquidityDeltas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMaxOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNetBalance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPairId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPairNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolReserves(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReserve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStrategy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Multicall(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pairs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProtocolFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProtocolFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeBatchTransferFrom(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SafeTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetApprovalForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetProtocolFee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Uri(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for PortfolioCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DefaultStrategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::PositionRenderer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Registry(element) => ::core::fmt::Display::fmt(element, f),
                Self::Version(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOfBatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeParameters(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePair(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLiquidityDeltas(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNetBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairId(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPairNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategy(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pairs(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeBatchTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetProtocolFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Uri(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DefaultStrategyCall> for PortfolioCalls {
        fn from(value: DefaultStrategyCall) -> Self {
            Self::DefaultStrategy(value)
        }
    }
    impl ::core::convert::From<PositionRendererCall> for PortfolioCalls {
        fn from(value: PositionRendererCall) -> Self {
            Self::PositionRenderer(value)
        }
    }
    impl ::core::convert::From<RegistryCall> for PortfolioCalls {
        fn from(value: RegistryCall) -> Self {
            Self::Registry(value)
        }
    }
    impl ::core::convert::From<VersionCall> for PortfolioCalls {
        fn from(value: VersionCall) -> Self {
            Self::Version(value)
        }
    }
    impl ::core::convert::From<WethCall> for PortfolioCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    impl ::core::convert::From<AllocateCall> for PortfolioCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for PortfolioCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<BalanceOfBatchCall> for PortfolioCalls {
        fn from(value: BalanceOfBatchCall) -> Self {
            Self::BalanceOfBatch(value)
        }
    }
    impl ::core::convert::From<ChangeParametersCall> for PortfolioCalls {
        fn from(value: ChangeParametersCall) -> Self {
            Self::ChangeParameters(value)
        }
    }
    impl ::core::convert::From<ClaimFeeCall> for PortfolioCalls {
        fn from(value: ClaimFeeCall) -> Self {
            Self::ClaimFee(value)
        }
    }
    impl ::core::convert::From<CreatePairCall> for PortfolioCalls {
        fn from(value: CreatePairCall) -> Self {
            Self::CreatePair(value)
        }
    }
    impl ::core::convert::From<CreatePoolCall> for PortfolioCalls {
        fn from(value: CreatePoolCall) -> Self {
            Self::CreatePool(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for PortfolioCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for PortfolioCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for PortfolioCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetLiquidityDeltasCall> for PortfolioCalls {
        fn from(value: GetLiquidityDeltasCall) -> Self {
            Self::GetLiquidityDeltas(value)
        }
    }
    impl ::core::convert::From<GetMaxLiquidityCall> for PortfolioCalls {
        fn from(value: GetMaxLiquidityCall) -> Self {
            Self::GetMaxLiquidity(value)
        }
    }
    impl ::core::convert::From<GetMaxOrderCall> for PortfolioCalls {
        fn from(value: GetMaxOrderCall) -> Self {
            Self::GetMaxOrder(value)
        }
    }
    impl ::core::convert::From<GetNetBalanceCall> for PortfolioCalls {
        fn from(value: GetNetBalanceCall) -> Self {
            Self::GetNetBalance(value)
        }
    }
    impl ::core::convert::From<GetPairIdCall> for PortfolioCalls {
        fn from(value: GetPairIdCall) -> Self {
            Self::GetPairId(value)
        }
    }
    impl ::core::convert::From<GetPairNonceCall> for PortfolioCalls {
        fn from(value: GetPairNonceCall) -> Self {
            Self::GetPairNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolNonceCall> for PortfolioCalls {
        fn from(value: GetPoolNonceCall) -> Self {
            Self::GetPoolNonce(value)
        }
    }
    impl ::core::convert::From<GetPoolReservesCall> for PortfolioCalls {
        fn from(value: GetPoolReservesCall) -> Self {
            Self::GetPoolReserves(value)
        }
    }
    impl ::core::convert::From<GetReserveCall> for PortfolioCalls {
        fn from(value: GetReserveCall) -> Self {
            Self::GetReserve(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for PortfolioCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetStrategyCall> for PortfolioCalls {
        fn from(value: GetStrategyCall) -> Self {
            Self::GetStrategy(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for PortfolioCalls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for PortfolioCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<PairsCall> for PortfolioCalls {
        fn from(value: PairsCall) -> Self {
            Self::Pairs(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for PortfolioCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeCall> for PortfolioCalls {
        fn from(value: ProtocolFeeCall) -> Self {
            Self::ProtocolFee(value)
        }
    }
    impl ::core::convert::From<ProtocolFeesCall> for PortfolioCalls {
        fn from(value: ProtocolFeesCall) -> Self {
            Self::ProtocolFees(value)
        }
    }
    impl ::core::convert::From<SafeBatchTransferFromCall> for PortfolioCalls {
        fn from(value: SafeBatchTransferFromCall) -> Self {
            Self::SafeBatchTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for PortfolioCalls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for PortfolioCalls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SetProtocolFeeCall> for PortfolioCalls {
        fn from(value: SetProtocolFeeCall) -> Self {
            Self::SetProtocolFee(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for PortfolioCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for PortfolioCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SwapCall> for PortfolioCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<UriCall> for PortfolioCalls {
        fn from(value: UriCall) -> Self {
            Self::Uri(value)
        }
    }
    ///Container type for all return fields from the `DEFAULT_STRATEGY` function with signature `DEFAULT_STRATEGY()` and selector `0x531e17b3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DefaultStrategyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `POSITION_RENDERER` function with signature `POSITION_RENDERER()` and selector `0xb0c3a950`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PositionRendererReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `REGISTRY` function with signature `REGISTRY()` and selector `0x06433b1b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `VERSION` function with signature `VERSION()` and selector `0xffa1ad74`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct VersionReturn(pub ::std::string::String);
    ///Container type for all return fields from the `WETH` function with signature `WETH()` and selector `0xad5c4648`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `allocate` function with signature `allocate(bool,address,uint64,uint128,uint128,uint128)` and selector `0x2f9e38e2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AllocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `balanceOf` function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `balanceOfBatch` function with signature `balanceOfBatch(address[],uint256[])` and selector `0x4e1273f4`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BalanceOfBatchReturn {
        pub balances: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `createPair` function with signature `createPair(address,address)` and selector `0xc9c65396`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreatePairReturn {
        pub pair_id: u32,
    }
    ///Container type for all return fields from the `createPool` function with signature `createPool(uint24,uint256,uint256,uint16,uint16,address,address,bytes)` and selector `0x267a0cfe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CreatePoolReturn {
        pub pool_id: u64,
    }
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(bool,uint64,uint128,uint128,uint128)` and selector `0x5bc55464`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DeallocateReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getAmountOut` function with signature `getAmountOut(uint64,bool,uint256,address)` and selector `0x19057807`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetAmountOutReturn {
        pub output: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getInvariant` function with signature `getInvariant(uint64)` and selector `0x39434d5a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetInvariantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getLiquidityDeltas` function with signature `getLiquidityDeltas(uint64,int128)` and selector `0x8992f20a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetLiquidityDeltasReturn {
        pub delta_asset: u128,
        pub delta_quote: u128,
    }
    ///Container type for all return fields from the `getMaxLiquidity` function with signature `getMaxLiquidity(uint64,uint256,uint256)` and selector `0xd6b7dec5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetMaxLiquidityReturn {
        pub delta_liquidity: u128,
    }
    ///Container type for all return fields from the `getMaxOrder` function with signature `getMaxOrder(uint64,bool,address)` and selector `0xf07b879e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetMaxOrderReturn(pub Order);
    ///Container type for all return fields from the `getNetBalance` function with signature `getNetBalance(address)` and selector `0x4dc68a90`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetNetBalanceReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getPairId` function with signature `getPairId(address,address)` and selector `0x3f92a339`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPairIdReturn(pub u32);
    ///Container type for all return fields from the `getPairNonce` function with signature `getPairNonce()` and selector `0x078888d6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPairNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolNonce` function with signature `getPoolNonce(uint24)` and selector `0xa5cd8a49`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPoolNonceReturn(pub u32);
    ///Container type for all return fields from the `getPoolReserves` function with signature `getPoolReserves(uint64)` and selector `0x2afb9df8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetPoolReservesReturn {
        pub delta_asset: ::ethers::core::types::U256,
        pub delta_quote: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getReserve` function with signature `getReserve(address)` and selector `0xc9a396e9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetReserveReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getSpotPrice` function with signature `getSpotPrice(uint64)` and selector `0xe331ba34`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetSpotPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getStrategy` function with signature `getStrategy(uint64)` and selector `0x30244be7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetStrategyReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isApprovedForAll` function with signature `isApprovedForAll(address,address)` and selector `0xe985e9c5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct IsApprovedForAllReturn(pub bool);
    ///Container type for all return fields from the `multicall` function with signature `multicall(bytes[])` and selector `0xac9650d8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MulticallReturn {
        pub results: ::std::vec::Vec<::ethers::core::types::Bytes>,
    }
    ///Container type for all return fields from the `pairs` function with signature `pairs(uint24)` and selector `0x5e47663c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PairsReturn {
        pub token_asset: ::ethers::core::types::Address,
        pub decimals_asset: u8,
        pub token_quote: ::ethers::core::types::Address,
        pub decimals_quote: u8,
    }
    ///Container type for all return fields from the `pools` function with signature `pools(uint64)` and selector `0x89a5f084`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct PoolsReturn {
        pub virtual_x: u128,
        pub virtual_y: u128,
        pub liquidity: u128,
        pub last_timestamp: u32,
        pub fee_basis_points: u16,
        pub priority_fee_basis_points: u16,
        pub controller: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `protocolFee` function with signature `protocolFee()` and selector `0xb0e21e8a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProtocolFeeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `protocolFees` function with signature `protocolFees(address)` and selector `0xdcf844a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProtocolFeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap((uint128,uint128,bool,uint64,bool),uint256,address)` and selector `0x80af9d76`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SimulateSwapReturn {
        pub success: bool,
        pub prev_invariant: ::ethers::core::types::I256,
        pub post_invariant: ::ethers::core::types::I256,
    }
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `swap` function with signature `swap((uint128,uint128,bool,uint64,bool))` and selector `0xf33ae1bc`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SwapReturn {
        pub pool_id: u64,
        pub input: ::ethers::core::types::U256,
        pub output: ::ethers::core::types::U256,
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
        Hash,
    )]
    pub struct UriReturn(pub ::std::string::String);
}
