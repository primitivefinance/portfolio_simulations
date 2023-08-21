pub use normal_strategy::*;
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
pub mod normal_strategy {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("portfolio_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("afterCreate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("afterCreate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
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
                            name: ::std::borrow::ToOwned::to_owned("success"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approximateReservesGivenPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approximateReservesGivenPrice",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("beforeSwap"),
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
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("configs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("configs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(128usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint128"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("volatilityBasisPoints",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("creationTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
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
                            name: ::std::borrow::ToOwned::to_owned("invariant"),
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
                    ::std::borrow::ToOwned::to_owned("getStrategyData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getStrategyData"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("volatilityBasisPoints",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strategyData"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("initialY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("portfolio"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("portfolio"),
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
                    ::std::borrow::ToOwned::to_owned("validatePool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validatePool"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint64"),
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
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invariant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AfterCreate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("AfterCreate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("portfolio"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("volatilityBasisPoints",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Genesis"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Genesis"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("portfolio"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("upper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lowerResult"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("upperResult"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Infinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Min"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_ConfigExists"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NormalStrategyLib_ConfigExists",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_InvalidDuration"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NormalStrategyLib_InvalidDuration",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_InvalidStrategyArgs"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "NormalStrategyLib_InvalidStrategyArgs",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_InvalidStrikePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "NormalStrategyLib_InvalidStrikePrice",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_InvalidVolatility"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned(
                            "NormalStrategyLib_InvalidVolatility",
                        ),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_NonExpiringPool"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NormalStrategyLib_NonExpiringPool",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategy_NotPortfolio"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NormalStrategy_NotPortfolio",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static NORMALSTRATEGY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0\x80`@R4b\0\x01IW`@Q`\x1Fb\09\xC28\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\x013W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12b\0\0\xE3WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03b\0\0\xDEW`\x80R`@Q\x90\x7F;5\xCD\xE5\x85\"y\xBF\xD1\xCE\x80S\x8F\x94\xC2\xE4*S\x0F\x11\x1FVB\x9B\x91,\x85\xCF\xA6P\xD4\xD3`\0\x80\xA2a8+\x90\x81b\0\x01\x97\x829`\x80Q\x81\x81\x81a\x01\x97\x01R\x81\x81a\x02\xAF\x01R\x81\x81a\x05\xA0\x01R\x81\x81a\n\xC9\x01R\x81\x81a\r\x17\x01R\x81\x81a\x0E\xA8\x01R\x81\x81a\x0F\x1F\x01R\x81\x81a\x12\xA8\x01R\x81\x81a\x15\xC1\x01Ra\x18\xD2\x01R\xF3[`\0\x80\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x12BW`\x005`\xE0\x1C\x80c\x16\xED\xE0\x16\x14a\0\xDCW\x80c\x19\x05x\x07\x14a\0\xD7W\x80c4\xDB\xC7;\x14a\0\xD2W\x80c9CMZ\x14a\0\xCDW\x80cE-/\x18\x14a\0\xC8W\x80cK\xF3F\xBF\x14a\0\xC3W\x80c\x80\xAF\x9Dv\x14a\0\xBEW\x80c\xA4G\x89\x19\x14a\0\xB9W\x80c\xE0hx\x7F\x14a\0\xB4W\x80c\xE31\xBA4\x14a\0\xAFW\x80c\xE6\x04{\x19\x14a\0\xAAW\x80c\xECshT\x14a\0\xA5Wc\xF0{\x87\x9E\x03a\x12BWa\x0F\x05V[a\x0E\x97V[a\x0E6V[a\x0C\xFCV[a\x0B\xD9V[a\n\xAEV[a\t\xFFV[a\t)V[a\x06\xCAV[a\x05\x85V[a\x05\x10V[a\x02{V[a\x01\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xCBW`\x006`\x03\x19\x01\x12a\x01\xC6W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[a\x011V[a\0\xE1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x026WV[`\0\x80\xFD[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x026WV[\x80\x15\x15\x03a\x026WV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x026WV[`\xC45\x90a\x02y\x82a\x02[V[V[4a\x01\xCBW`\x806`\x03\x19\x01\x12a\x01\xC6Wa\x02\x94a\x02 V[`$5a\x02\xA0\x81a\x02QV[`d5\x91a\x02\xAD\x83a\x02[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01Ra\x01\0\x91\x90\x82\x81`$\x81\x85Z\xFA\x92\x83\x15a\x04\x94W`\0\x93a\x04\xE1W[PP\x80;\x15a\x04\x99W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x84\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x91`\x80\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x94W`\0\x93a\x04\xB1W[P\x85\x15a\x04\x9EWa\x03t`\xFFa\x03n\x84\x86\x01Q`\xFF\x16\x90V[\x16a\x18\x14V[\x96a\x03\xDAa\x03\xA2a\x03\x98\x88`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x99`D5\x02a76V[\x96a\x03\xBDa\x03\xAEa\x08tV[`\x01`\x01`\x80\x1B\x03\x90\x99\x16\x89RV[`\x01\x88\x86\x01R`\0`@\x89\x01R`\x01`\x01`@\x1B\x03\x16``\x88\x01RV[\x86\x15\x15`\x80\x87\x01R\x81;\x15a\x04\x99W\x82`\x04\x92`@Q\x93\x84\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x80\x15a\x04\x94Wa\x04R\x98a\x04A\x97`\xFF\x97a\x04.\x95`\0\x94a\x04eW[Pa\x04(B\x93a\x14,V[\x90a(\xCFV[\x94\x15a\x04VWP``\x01Q`\xFF\x16a\x03nV[`@Q\x91\x04\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x01Q`\xFF\x16a\x03nV[a\x03nV[a\x04\x86\x91\x94P\x87=\x89\x11a\x04\x8DW[a\x04~\x81\x83a\x08SV[\x81\x01\x90a\x17\xACV[\x928a\x04\x1DV[P=a\x04tV[a\x15\xB3V[a\x14{V[a\x03t`\xFFa\x04```\x86\x01Q`\xFF\x16\x90V[a\x04\xD3\x91\x93P`\x80=\x81\x11a\x04\xDAW[a\x04\xCB\x81\x83a\x08SV[\x81\x01\x90a\x17>V[\x918a\x03UV[P=a\x04\xC1V[a\x05\x01\x92\x93P\x80=\x10a\x05\tW[a\x04\xF9\x81\x83a\x08SV[\x81\x01\x90a\x15\x04V[\x908\x80a\x03\x16V[P=a\x04\xEFV[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6W`\x01`\x01`@\x1B\x03a\x051a\x02 V[\x16`\0R`\0` R`\xA0`@`\0 T`\xFF`@Q\x91`\x01`\x01`\x80\x1B\x03\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x80\x82`\x80\x1C\x16` \x85\x01R\x80\x82\x86\x1C\x16`@\x85\x01R\x81`\xC0\x1C\x16``\x84\x01R`\xE0\x1C\x16\x15\x15`\x80\x82\x01R\xF3[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6Wa\x05\x9Ea\x02 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x94Wa\x04R\x93a\x06=\x93`\0\x93a\x06MW[PPa\x062a\x067\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x14,V[\x90a#\x82V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x067\x92\x93Pa\x062\x91\x81a\x06m\x92\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x92\x91a\x06\x10V[\x90\x93\x92\x91\x93``\x82R\x80Q\x90\x81``\x84\x01R`\0[\x82\x81\x10a\x06\xB4WPP\x90`\x80\x82`\0\x82`@\x95\x85\x01\x01R`\x1F\x80\x19\x91\x01\x16\x82\x01\x01\x94` \x82\x01R\x01RV[\x80` \x80\x92\x84\x01\x01Q`\x80\x82\x87\x01\x01R\x01a\x06\x89V[4a\x01\xCBW`\xA06`\x03\x19\x01\x12a\x01\xC6Wa\x04R`d5a\x06\xEA\x81a\x02QV[a\x07\x9Aa\x06\xF8`\x045a76V[a\x07\x03`$5a7NV[\x92a\x07\x0F`D5a7NV[`\x01`\x01`\x80\x1B\x03`@Q\x93a\x07$\x85a\x08\x18V[\x16\x94\x85\x84R` \x84\x01\x90c\xFF\xFF\xFF\xFF\x92\x83\x80\x92\x16\x83R\x81`@\x87\x01\x91\x16\x81R\x81``\x87\x01\x93`\0\x85R`\x80\x88\x01\x96\x15\x15\x87R`@Q\x99` \x8B\x01RQ\x16`@\x89\x01RQ\x16``\x87\x01RQ\x16`\x80\x85\x01RQ\x15\x15`\xA0\x84\x01R`\xA0\x83Ra\x07\x89\x83a\x088V[a\x07\x95`\x845\x91a UV[a\x1D\xBEV[`@\x93\x91\x93Q\x93\x84\x93\x84a\x06tV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@RV[a\x08\x02V[`\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@RV[`@Q\x90a\x02y\x82a\x08\x18V[`@Q\x90a\x02y\x82a\x088V[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x083W`@Q\x91a\x08\xB7`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x08SV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x08\xD4W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xCBW`@6`\x03\x19\x01\x12a\x01\xC6W`$5`\x01`\x01`@\x1B\x03\x81\x11a\t\x98W6`#\x82\x01\x12\x15a\t\x93Wa\t\x81\x90a\x07\x95a\t|a\tw`\x045\x936\x90`$\x81`\x04\x015\x91\x01a\x08\x8EV[a\"\xECV[a UV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[a\x07\xA9V[a\x01\xD0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x026WV[4a\x01\xCBW6`\x03\x19\x01`\xE0\x81\x12a\x01\xC6W`\xA0\x13a\n\xA9Wa\x04Ra\n\x8A`@Qa\n*\x81a\x08\x18V[`\x045a\n6\x81a\t\xEEV[\x81R`$5a\nD\x81a\t\xEEV[` \x82\x01R`D5a\nU\x81a\x02QV[`@\x82\x01Ra\nba\x02;V[``\x82\x01R`\x845a\ns\x81a\x02QV[`\x80\x82\x01Ra\n\x80a\x02lV[\x90`\xA45\x90a\x18\xCAV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[a\t\x9DV[4a\x01\xCBW`\x806`\x03\x19\x01\x12a\x01\xC6Wa\n\xC7a\x02 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01Ra\x01\0\x92\x90\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x04\x94Wa\x0B\x93\x93`\0\x93a\x0B\xB2W[PPa\x062a\x067\x91a\x0B\\a\x0BO`D5a76V[`\x01`\x01`\x80\x1B\x03\x16\x85RV[a\x0Bza\x0Bj`d5a76V[`\x01`\x01`\x80\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x0B\x9F\x81`$5a\x17\x15V[`@\x80Q\x91\x15\x15\x82R` \x82\x01\x92\x90\x92R\xF3[a\x067\x92\x93Pa\x062\x91\x81a\x0B\xD2\x92\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x92\x91a\x0B8V[4a\x01\xCBW`@6`\x03\x19\x01\x12a\x01\xC6Wa\x0B\xF2a\x02 V[`$5`\x01`\x01`@\x1B\x03\x80\x82\x11a\t\x98W6`#\x83\x01\x12\x15a\t\x93W\x81`\x04\x015\x90\x81\x11a\x0C\xA3W6`$\x82\x84\x01\x01\x11a\x0CJWa\x04R\x92`$a\x0C8\x93\x01\x90a\x12\xA5V[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6Wa\r\x15a\x02 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91a\x01\0\x91\x82\x90\x84\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04\x94Wa\x04R\x93a\x06=\x93`\0\x93a\x0E\x0BW[PPa\r\xAEa\x062a\x0E\x05\x92`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91a\r\xC3a\r\xBB\x84a UV[\x93B\x90a\"\x89V[`\x80\x84\x01Ra\r\xFFa\r\xE4`@a\r\xF0a\r\xE4\x85Q`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x93\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a5(V[\x90a\x1C\xE3V[a\x0E\x05\x92\x93Pa\x0E.a\r\xAE\x92\x82a\x062\x93\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x93\x92Pa\r\x89V[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6Wa\x0EOa\x02 V[P` `@Q`\x01\x81R\xF3[``\x90`\x03\x19\x01\x12a\x01\xC6W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x026W\x90`$5a\x0E\x87\x81a\x02QV[\x90`D5a\x0E\x94\x81a\x02[V[\x90V[4a\x01\xCBWa\x0E\xA56a\x0E[V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0E\xF3Wa\x0E\xDF\x91a\x15\xBFV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`@Qc:#%k`\xE2\x1B\x81R`\x04\x90\xFD[4a\x01\xCBWa\x0F\x136a\x0E[V[Pa\x0F\x1Ca\x18BV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@\x80Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01Ra\x01\0\x94\x91\x92\x91\x90\x85\x81`$\x81\x86Z\xFA\x95\x86\x15a\x04\x94W`\0\x96a\x12#W[PP\x81;\x15a\x04\x99W\x82Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x82\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x92`\x80\x90\x84\x90\x81\x80`$\x81\x01\x03\x91Z\xFA\x80\x15a\x04\x94Wa\x04R\x96\x85\x94`\0\x92a\x12\x03W[Pa\x10\x04a\x0F\xFAa\t|a\x062\x87`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[`@\x01Q\x90`\0\x90V[\x95\x90a\x10\x0Ea\x08tV[\x98`\0\x8ARa\x106\x86\x8B\x01\x97`\0\x89R`\0\x85\x8D\x01R``\x8C\x01\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x15\x80\x15`\x80\x8B\x01Ra\x11\x84WPa\x11\0``a\x11\x0C\x94a\x11\x1Ea\x11\x11a\x11\x0C\x87a\x11\x06a\x11\0a\x10\xF6a\x113\x9Fa\r\xE4a\x11&\x9F\x9Da\x10\xEF\x8F\x93a\x11\x06\x9Fa\r\xE4a\x10\xDBa\x10\xC7\x97a\x10\xCCa\x10\xC7\x88a\x10\xC1a\r\xE4a\x10\xB3a\x10\xAEa\x10\xE9\x9A`\x01`\x01`\x80\x1B\x03\x9E\x01\x9E\x8FQ`\x01`\x01`\x80\x1B\x03\x16\x90V[a4\xECV[\x92Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x17\xF9V[a\x17\xD4V[\x9C\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a5\x07V[\x91\x16a\x17\xF9V[\x9A\x87\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x90a\x18(V[a76V[`\x01`\x01`\x80\x1B\x03\x16\x8CRV[\x01Q`\xFF\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90RV[Q\x91\x82\x91\x82\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[a\x11\xFE\x96P\x84a\x11\x0C\x94a\x11\x1Ea\x11\x11a\x11\x0Ca\x11\xEC`\x01`\x01`\x80\x1B\x03a\x11\xE6a\x11\xD8a\x10\xC7\x8Ca\x11&\x9Fa\r\xE4a\x10\xC1\x91a\x11\x06\x9Fa\x11\0\x9Fa\x10\xE9a\r\xE4a\r\xF0\x93\x88\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16a\x17\xD4V[\x97a\x11\x06a\x11\0``\x87\x01Q`\xFF\x16\x90V[a\x113V[a\x12\x1C\x91\x92P`\x80=\x81\x11a\x04\xDAWa\x04\xCB\x81\x83a\x08SV[\x908a\x0F\xD3V[a\x12:\x92\x96P\x80=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x938\x80a\x0F\x8AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x913\x83\x90\x03a\x0E\xF3Wa\twa\x12\xEB\x91a\x13X\x936\x91a\x08\x8EV[a\x13\x08\x84`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x81Q`\x01`\x01`\x80\x1B\x03\x16` \x83\x01Qc\xFF\xFF\xFF\xFF\x16\x91`\x01`\x01`\x80\x1B\x03a\x13E`\x80a\x13=`@\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x96\x01Q\x15\x15\x90V[\x94c\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x92\x16\x90a \xDFV[\x7F\x8Di\xFAt\x9A\xF4\x15\xF9<\x96\x86>\xBF6\x92\x93&#\xC0\x10\xEB\xA8\x11\x8B\x02\xF0DXZ\xABe\x13`\x01`\x01`@\x1B\x03a\x13\xA1a\x062\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x93a\x14$a\x13\xB6\x86Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95a\x13\xC8` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x13\xE7`\x80a\x13\xDF`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92\x01Q\x15\x15\x90V[\x91`@Q\x95\x86\x95\x16\x98\x85\x92\x91``\x92\x95\x94\x91\x95`\x01`\x01`\x80\x1B\x03`\x80\x86\x01\x97\x16\x85Rc\xFF\xFF\xFF\xFF\x80\x92\x16` \x86\x01R\x16`@\x84\x01R\x15\x15\x91\x01RV[\x03\x90\xA3`\0\x90V[\x90`@Qa\x149\x81a\x08\x18V[`\x80`\xFF\x82\x94T`\x01`\x01`\x80\x1B\x03\x81\x16\x84Rc\xFF\xFF\xFF\xFF\x80\x82\x85\x1C\x16` \x86\x01R\x80\x82`\xA0\x1C\x16`@\x86\x01R\x81`\xC0\x1C\x16``\x85\x01R`\xE0\x1C\x16\x15\x15\x91\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90a\x02y\x82a\t\xEEV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x026WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\x026WV[Q\x90a\x02y\x82a\x02[V[\x80\x91a\x01\0\x92\x83\x91\x03\x12a\x01\xC6W`@Q\x91\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083Wa\x15\xAB\x91`\xE0\x91`@Ra\x15=\x81a\x14\xCEV[\x84Ra\x15K` \x82\x01a\x14\xCEV[` \x85\x01Ra\x15\\`@\x82\x01a\x14\xCEV[`@\x85\x01Ra\x15m``\x82\x01a\x14\xD9V[``\x85\x01Ra\x15~`\x80\x82\x01a\x14\xEAV[`\x80\x85\x01Ra\x15\x8F`\xA0\x82\x01a\x14\xEAV[`\xA0\x85\x01Ra\x15\xA0`\xC0\x82\x01a\x14\xF9V[`\xC0\x85\x01R\x01a\x14\xF9V[`\xE0\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90\x83;\x15a\x04\x99W`@Q\x80\x94c\"i|!`\xE2\x1B\x82R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x83\x01R\x81`$a\x01\0\x97\x88\x93Z\xFA\x94\x85\x15a\x04\x94W`\0\x95a\x16tW[PP\x90a\x16^a\x16Ta\x062a\x16d\x94`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91B\x90\x83\x87a%0V[\x93a,JV[a\x16nW`\x01\x91\x90V[`\0\x91\x90V[a\x16d\x93\x92\x95Pa\x062a\x16\x9Ba\x16^\x93\x83a\x16T\x94\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x96\x93\x94PPa\x16+V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x16\xD4WV[a\x16\xA5V[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x16\xD4WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x16\xD4WV[a\x17!\x90`\x01\x92a\x16\xFCV[\x12a\x17+W`\x01\x90V[`\0\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x026WV[\x90\x81`\x80\x91\x03\x12a\x01\xC6W`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083Wa\x17\xA4\x91``\x91`@R\x80Qa\x17x\x81a\x02[V[\x84Ra\x17\x86` \x82\x01a\x170V[` \x85\x01R`@\x81\x01Qa\x17\x99\x81a\x02[V[`@\x85\x01R\x01a\x170V[``\x82\x01R\x90V[\x90\x81` \x91\x03\x12a\x01\xC6WQ\x90V[\x90`\x12\x03`\x12\x81\x11a\x16\xD4Wa\x17\xD0\x90a\x18\x06V[\x02\x90V[`\0\x19\x81\x01\x91\x90\x82\x11a\x16\xD4WV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x16\xD4WV[\x91\x90\x82\x03\x91\x82\x11a\x16\xD4WV[`M\x81\x11a\x16\xD4W`\n\n\x90V[`\x12\x03`\x12\x81\x11a\x16\xD4Wa\x0E\x94\x90a\x18\x06V[\x90`\x12\x03`\x12\x81\x11a\x16\xD4Wa\x18=\x90a\x18\x06V[\x90\x04\x90V[`@Q\x90a\x18O\x82a\x08\x18V[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[``\x81\x01\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94\x93\x91\x92\x90`\x01`\x01`@\x1B\x03\x16\x92\x85;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x94\x90\x94\x16`\x04\x85\x01Ra\x01\0\x90\x81\x85`$\x81\x8AZ\xFA\x94\x85\x15a\x04\x94W`\0\x95a\x1B\xB1W[Pa\x19xa\x19ma\x19a\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[` \x1Cb\xFF\xFF\xFF\x16\x90V[\x90\x87;\x15a\x04\x99W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`\x80\x80\x83`$\x81\x8CZ\xFA\x93\x84\x15a\x04\x94Wa\x11&`\xA0a\x1A\xAD\x93a\x1A\xBA\x97a\x0Bz\x97`\0\x91a\x1B\x94W[Pa\x19\xCDa\x18mV[\x94a\x19\xDA\x83\x8C\x01Q\x15\x15\x90V[\x15a\x1B1W`@\x82a\x1A\x03a\x1A=\x93a\x19\xFA` a\x1AM\x97\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x8A\x01RV[a\x1A\x1Fa\x1A\x14``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x8A\x01RV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x89\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x86\x01RV[a\x1Aaa\r\xE4\x8AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81\x85\x01Ra\x1A\xA6a\x1A\x99a\x1A\x90` \x8C\x01\x96a\x1A\x87a\r\xE4\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x82\x01Ra\x1B\xD0V[\x92\x83\x01Qa76V[`\x01`\x01`\x80\x1B\x03\x16\x8ARV[\x01Qa76V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92\x85;\x15a\x04\x99W` `\x04\x96`@Q\x97\x88\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x93\x84\x15a\x04\x94Wa\x1A\xFD\x96`\0\x95a\x1B\rW[Pa\x1A\xF7\x90a\x14,V[\x90a%\xFDV[\x90\x91Pa\x1B\n\x81\x83a\x17\x15V[\x92V[a\x1A\xF7\x91\x95Pa\x1B*\x90` =\x81\x11a\x04\x8DWa\x04~\x81\x83a\x08SV[\x94\x90a\x1A\xEDV[\x81a\x1BSa\x1A=\x92a\x1BJ``a\x1B\x8F\x96\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x89\x01RV[a\x1Boa\x1Bd` \x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x89\x01RV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01RQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x1AMV[a\x1B\xAB\x91P\x82=\x84\x11a\x04\xDAWa\x04\xCB\x81\x83a\x08SV[8a\x19\xC4V[a\x1B\xC9\x91\x95P\x82=\x84\x11a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x938a\x19IV[a\x1B\xD8a\x18mV[P`@\x81\x01\x80Q\x90a\x1B\xF4a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90a\x17\xBBV[\x90R``\x82\x01a\x1C\n\x81Q`\xFF\x84Q\x16\x90a\x17\xBBV[\x90Ra\x1C!`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90a\x17\xBBV[\x90R`\xA0\x81\x01a\x1C;\x81Q`\xFFa\x01 \x85\x01Q\x16\x90a\x17\xBBV[\x90R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x16\xD4WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x16\xD4W`\0\x19\x83\x05\x03a\x16\xD4WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x16\xD4WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x16\xD4W\x81\x84\x05\x14\x90\x15\x17\x15a\x16\xD4WV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x16\xD4WV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x80\x83\x10\x15a\x1D~W\x82\x15a\x1DpWa\x1D\x07`\x80\x83\x01Qa4\xC5V[\x90a\x1D\x11\x83a\x1D\x86V[\x93\x81\x03\x90\x80\x82\x11a\x16\xD4Wa\x1Da`@\x93g\x1B\xC1mgN\xC8\0\0a\x1DZa\x1DFa\x0E\x94\x99a\x1DAa\x1Dg\x98a-\xE5V[a\x1C\x97V[\x92a\x1DU``\x8A\x01Q\x80a\x1C\xBAV[a\x1C\xBAV[\x04\x90a\x16\xFCV[\x05a1hV[\x91\x01Q\x90a5\x07V[PPP`\x01`\x01`\x80\x1B\x03\x90V[PPP`\0\x90V[a\x1D\x9Ba\x1D\x96`\x80\x83\x01Qa4\xC5V[a6\x10V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x16\xD4W``a\x0E\x94\x92\x01Qa5\x07V[`@\x81\x01Q`\0\x93\x92a\x1D\xD1\x91\x90a5(V[\x80a\x1D\xE4W[Pa\x0E\x94\x90\x83\x81Ra\x1F\xDAV[a\x1D\xF3a\x1E\x1D\x91\x94\x92\x94a3$V[a\x1E=a\x1E\x03`\x80\x87\x01Qa4\xC5V[a\x1E7``\x88\x01Qg\x1B\xC1mgN\xC8\0\0\x95\x81\x87\x92a\x1C\xBAV[\x04\x91a\x1E1a\x1E+\x8Aa\x1D\x86V[\x95a\x1C@V[\x92a\x1C\x97V[\x90a\x1F\xBEV[\x90\x80\x15a\x1E\xABW`\x01`\xFF\x1B\x82\x14`\0\x19\x82\x14\x16a\x16\xD4Wa\x1Exg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1Era\x1E}\x93a\x1E\x82\x95\x05a\x1C@V[\x05a,vV[a,\xAFV[a\x1C@V[\x05g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x03\x92\x90\x91\x12\x80\x15\x82\x84\x13\x16\x91\x83\x12\x16\x17a\x16\xD4W\x91a\x0E\x94a\x1D\xD7V[a\x1C\xCDV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x16\xD4WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x16\xD4WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x16\xD4WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x16\xD4WV[`@\x81\x01\x80Q\x82Q\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x10\x15a KW\x15a DWPa \x04\x83a\x1D\x86V[\x90\x83Q\x81\x03\x90\x81\x11a\x16\xD4Wa\x0E\x94\x93`\xA0a .a <\x94a )a 7\x95a-\xE5V[a\x16\xFCV[\x91\x01Q\x90a\x1F\xBEV[a,\x87V[\x90Q\x90a5\x07V[\x92PPP\x90V[PPPPP`\0\x90V[`\xA0`@Qa c\x81a\x088V[`\0\x91\x81\x83\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x01R`\x01`\x01`\x80\x1B\x03\x82Q\x16\x91c\xFF\xFF\xFF\xFF`@a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x83` \x86\x01Q\x16\x02\x04\x92\x01Q\x16\x90`@Q\x93a \xBF\x85a\x088V[\x83\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[\x92\x93\x91\x90\x93c\xFF\xFF\xFF\xFF\x92\x83a \xFD\x86Tc\xFF\xFF\xFF\xFF\x90`\xC0\x1C\x16\x90V[\x16a\"wW\x80\x15a\"\x16W\x84T`\xFF`\xE0\x1B\x19\x16\x90\x15\x15`\xE0\x1B`\xFF`\xE0\x1B\x16\x17\x84UP\x82Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16c\x01\xE1\x85Y`\xA0\x1B\x17\x83U[aa\xA8\x80\x82\x10\x90\x82\x14\x17`\x01\x82\x11`\x01\x83\x14\x17\x16\x15a\"\x04Wa!^a!~\x91a7NV[\x83Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x80\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\x80\x1B\x16\x17\x83UV[`\x01`\x01`\x80\x1B\x03\x80\x84\x10\x90\x84\x14\x17`\x01\x84\x11`\x01\x85\x14\x17\x16\x15a!\xF2Wa!\xD0a!\xABa\x02y\x94a76V[\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x83UV[\x81Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16B\x91\x90\x91\x16`\xC0\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x17\x90UV[`@Qc\xB2B\xE3A`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9]8\x19`\xE0\x1B\x81R`\x04\x90\xFD[Pb\x01Q\x80\x81\x81\x14\x90\x82\x11\x17c\x05\xA4\x90\x0B\x82\x81\x14\x90\x83\x10\x17\x16\x15a\"eWa\"@a\"`\x91a7NV[\x84Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x84UV[a!9V[`@Qc\xB5\x97\x03\x0F`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x1E\x13\x89\xA1`\xE2\x1B\x81R`\x04\x90\xFD[`\x80\x81\x01Qa\"\xAEWa\"\xA0c\xFF\xFF\xFF\xFF\x91a\"\xB8V[\x16\x80\x82\x10\x90\x82\x03\x02\x81\x03\x03\x90V[PPc\x01\xE1\x85Y\x90V[`\x80\x81\x01Qa\"\xDAWc\xFF\xFF\xFF\xFF\x90\x81`@\x81``\x84\x01Q\x16\x92\x01Q\x16\x01\x16\x90V[`@Qc\xB0\x19\x84\x97`\xE0\x1B\x81R`\x04\x90\xFD[a\"\xF4a\x18BV[P`\xA0\x81Q\x03a#pW`\xA0\x81\x80Q\x81\x01\x03\x12a\x01\xC6W`\xA0`@Q\x91a#\x1A\x83a\x08\x18V[` \x81\x01Qa#(\x81a\t\xEEV[\x83Ra#6`@\x82\x01a\x14\xD9V[` \x84\x01Ra#G``\x82\x01a\x14\xD9V[`@\x84\x01Ra#X`\x80\x82\x01a\x14\xD9V[``\x84\x01R\x01Qa#h\x81a\x02QV[`\x80\x82\x01R\x90V[`@Qc\x01YW\xD3`\xE2\x1B\x81R`\x04\x90\xFD[a\x0E\x94\x91a#\x9Aa\r\xE4\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91`\x01`\x01`\x80\x1B\x03a#\xE4a#\xC7a#\xC0a\r\xE4`@\x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x96a5(V[\x94a#\xDFa\r\xE4` \x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a5(V[\x92a$Ia#\xF9\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a$Cc\xFF\xFF\xFF\xFF``a$6a$$a$\x1B` \x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[g\r\xE0\xB6\xB3\xA7d\0\0a'\x10\x91\x02\x04\x90V[\x97\x01Q\x16c\xFF\xFF\xFF\xFF\x16\x90V[\x90a\"\x89V[\x93a$Ra\x08\x81V[\x95\x86R` \x86\x01R\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\0`\xA0\x82\x01R[a\x0E\x94\x90a$\xD6a$\x82\x82a\x1D\x86V[\x91a$\x91\x81`@\x01Q\x90`\0\x90V[\x82Q\x90\x92\x91\x90g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a%\x0CWP`\x01\x92[` \x83\x01Q\x91\x82\x10a$\xDBWPPPa )a$\xD0g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92[a-\xE5V[\x91a-\xE5V[a\x1F\xBEV[\x81\x11a$\xF1WPPa )a$\xD0`\x01\x92a-\xE5V[a%\x06a$\xD0\x91`@a )\x94\x01Q\x90a5\xE0V[\x92a-\xE5V[\x80a%!WPg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92a$\xABV[a%*\x90a\x17\xE3V[\x92a$\xABV[\x90`\x01`\x01`\x80\x1B\x03a\x0E\x94\x94\x92a%Ra\r\xE4\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a%ia\r\xE4` \x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95\x15a%\xCDWa%\x93a%\x8Ca\r\xE4`@a%\x99\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a5(V[\x95a5\xE0V[\x92[a$Ia%\xAF\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a%\xC7a$$a$\x1B` \x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x94a\"\x89V[a%\xF1a%\xEAa\r\xE4`@a%\xF7\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a5\xE0V[\x95a5(V[\x92a%\x9BV[a\r\xFFa\r\xE4a'\x15\x93a'\x06`@a&\x8Ba&&\x9A\x99\x98\x99a&-\x8A`\x80\x8D\x01\x9D\x8EQ\x15\x15\x90V[\x87\x8Ba%0V[\x9Ca&?\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[` \x8A\x01Q`\xC0\x8B\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x03a':W`\xA0\x8A\x01Qa\xFF\xFF\x16\x9C[`\x01`\x01`\x80\x1B\x03\x9Da\xFF\xFF\x16\x92\x8E\x16\x91\x8E\x16\x90a'IV[\x98\x92P\x94\x90Pa&\xC3a&\xA5\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9Aa&\xBDa$$a$\x1B` \x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92a\"\x89V[\x90a&\xCCa\x08\x81V[\x9B`\0\x8DR` \x8D\x01\x9B`\0\x8DR\x16\x84\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\0`\xA0\x8B\x01R\x01\x91a\r\xFFa\r\xE4\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87RQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81Ra'*a'#\x83a$rV[\x93Q\x15\x15\x90V[\x15a'4WPQ\x92V[\x90PQ\x92V[`\x80\x8A\x01Qa\xFF\xFF\x16\x9Ca&rV[\x91\x94\x92\x94`\0a'd`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16a5\x92V[\x95\x80\x15\x80\x15a(\x97W[PP\x80\x95\x80\x97`\x80\x86\x01\x92a'\xBAa'\x86\x85Q\x15\x15\x90V[\x93\x84\x15a(\x90W\x87\x94[\x15a(\x86Wa'\xB5\x87\x95[a'\xAFa\r\xE4\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a(\xB3V[a\x17\xF9V[\x90\x81\x81\x11a(tWa'\xCF` \x91\x85\x93a\x17\xF9V[\x97\x01\x91a'\xE3\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11a(bWa(\x0B\x91a\x10\xC1a\r\xE4a(\x04\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91Q\x15\x15\x90V[\x90\x81\x15a(YW\x84\x85\x92[\x15a(QWP\x92[\x14a(?W\x81\x14a(-W\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92a(\x1EV[\x80\x94\x85\x92a(\x16V[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[a'\xB5\x88\x95a'\x9BV[\x86\x94a'\x90V[\x90\x91Pa\x1E\xABW\x85\x04\x94\x85\x81\x03\x90\x81\x11a\x16\xD4W\x948\x80a'nV[\x91\x90\x82\x01\x80\x92\x11a\x16\xD4WV[`\0\x19\x81\x14a\x16\xD4W`\x01\x01\x90V[\x93\x91a)\0a(\xE8`\x80\x93\x97a)\x0B\x95\x87\x85\x8B\x8Ba%\xFDV[P\x95\x90\x97a(\xF5\x81a UV[\x96`\xA0\x88\x01Ra\"\x89V[\x82\x85\x01R\x01Q\x15\x15\x90V[\x92\x83\x15a*=W\x81Ra)\x1D\x81a\x1F\xDAV[\x80` \x83\x01R[\x80\x15a*\x0CW\x91a)\xDBa)\xC4a)\xBFa)\xA3`\x01`\x01`\x80\x1B\x03\x95\x87a)\xB1a)Za)Ta\x0E\x94\x9C\x9Ba5JV[\x92a5\xB8V[\x92`@Q\x94\x85\x91` \x83\x01\x91\x90\x91`\xA0\x80`\xC0\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x85R\x84a\x08SV[\x88\x15a*\x04W`\x02\x92a*SV[a(\xC0V[a\x10\xE9a\r\xE4`@\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x92\x15a)\xF5W` \x01Q`\x01`\x01`\x80\x1B\x03\x16[\x16a\x17\xF9V[Q`\x01`\x01`\x80\x1B\x03\x16a)\xEFV[`\x01\x92a*SV[PP`\x01`\x01`\x80\x1B\x03\x91`\0\x14a*/W` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[` \x82\x01Ra*K\x81a+\x82V[\x80\x82Ra)$V[\x91\x90\x93\x92\x93`\0\x94`\0\x92\x80\x83\x11a+aWa*p\x83\x86\x84a7zV[\x90a*|\x81\x87\x85a7zV[\x85a*\x87\x82\x85a\x1C\x97V[\x13a+@WP\x95\x94\x93a*\x9A\x84\x88a\x17\xF9V[\x94\x81`\x01\x96\x87\x80[a*\xB4W[PPPPPPPPP\x90PV[\x15a+\x1BW[P\x86\x97\x98\x99P\x80\x92a*\xD5a*\xCF\x8C\x89a(\xB3V[`\x01\x1C\x90V[\x9A\x8B\x90a*\xE3\x8D\x86\x8Aa7zV[\x90\x84a*\xEF\x89\x84a\x1C\x97V[\x13a+\rWPP\x93[\x88a+\x03\x89\x87a\x17\xF9V[\x92\x01\x94\x9A\x99a*\xA2V[\x95\x96P\x97PP\x8A\x96\x94a*\xF8V[\x87\x10\x80a+5W[\x15a+.W\x89a*\xBAV[\x80\x80a*\xA7V[Pa\x01\0\x83\x10a+#V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@\x81\x01\x80Q\x91` \x81\x01\x92\x83Q\x90\x81\x10\x15a+\xF1W\x15a+\xE2W`\xA0a+\xC3a+\xCC\x94a$\xD6a$\xCBa 7\x96a+\xB9\x87a\x1D\x86V[\x93Q\x90Q\x90a5\xE0V[\x91\x01Q\x90a\x16\xFCV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x16\xD4W\x90V[PPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[PPPP`\0\x90V[\x90\x81`\xC0\x91\x03\x12a\x01\xC6W`\xA0`@Q\x91a,\x14\x83a\x088V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R\x01Q`\xA0\x82\x01R\x90V[`\x80\x82\x01Qa,oW``\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91a,i\x90a\"\xB8V[\x16\x11\x15\x90V[PP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\x16\xD4W`\0\x03\x90V[a,\xABa\x1E}a\x1Exg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1Erg\x1B\xC1mgN\xC8\0\0\x95a\x1C@V[\x05\x90V[\x80\x15a-\xD8WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a-\xD2WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a-\xC5Wa-\xB3a,\xE2\x82a/\x90V[a-va-\xAEa-\x01a,\xFCa,\xF7\x85a5gV[a\x1E\xB0V[a1'V[\x92a$\xD6a-\xA9a-\xA4a-\x9Da-\x97a-\x92a-\x8Ca-\x87a-\x81a-|\x8Da-va-qa-ka-fa-`a-[a-Ua-Pa-Ja-E\x8Aa/\xBDV[a\x1E\xC8V[\x89a0_V[a\x1E\xE2V[\x87a0_V[a\x1E\xFAV[\x85a0_V[a\x1F\x14V[\x83a0_V[a\x1F,V[\x90a0_V[a\x1FFV[\x8Ca0_V[a\x1F^V[\x8Aa0_V[a\x1FvV[\x88a0_V[\x93\x80a0_V[a\x1C]V[a\x16\xBBV[a1hV[\x90`\0\x13\x15a\x0E\x94Wa\x0E\x94\x90a\x16\xD9V[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a-\xD2Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a/:W\x81\x15a/[W`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x16\xD4W`\0\x83\x12\x80\x15a/\x7FW[a/mW\x82\x15a/:Wg\x1B\xC1mgN\xC8\0\0\x83\x14a/[W\x82\x12\x91\x82\x15a/LW\x92[a.T\x84a0\xA7V[\x80\x15a/:Wa.\xB1a.\x80a.{a\x1D\x96a.va.\xB6\x95\x99\x97\x96\x99a3$V[a/\xE8V[a\x1C~V[a )a.\x94a.\x8F\x83a0\xD2V[a\x1F\x8EV[a.\xABa,\xF7a-`a.\xA6\x86a0\xFDV[a\x1F\xA6V[\x90a1FV[a0\x10V[\x93`\0\x92[\x81\x84\x10a.\xEDWPPPPa\x0E\x94\x91a.\xDA\x91`\0\x14a.\xDFWa0\x80V[a,vV[a.\xE8\x90a,vV[a0\x80V[\x90\x91a/0\x86a\x1E7a/\x05\x85a )\x86\x99\x9Ba,\xAFV[a.\xABa/ a/\x1Ba-\xAEa.\xDA\x87\x80a0_V[a08V[a/*\x83\x86a0_V[\x90a\x16\xFCV[\x95\x01\x92\x91\x90a.\xBBV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a/U\x90a\x16\xD9V[\x92a.KV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a.'V[`\x01`\xFF\x1B\x81\x14a/\xABW`\0\x81\x12\x15a\x0E\x94W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x026Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x026W\x05\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a-\xD2Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a2\xB8We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x15a2\xF3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a3P`\0\x82\x13a2\xECV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a3l\x82a6\xCEV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026Wc\x01\xE1\x85Y\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x02\x04\x90\x81\x14`\x01\x16\x15a\x026W\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x026W\x04\x90V[`2\x81\x02\x90`2\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026W`d\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x96\x81\x02\x90`\x96\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x026W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a6\xB7W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a6\xAAW[e\x01\0\0\0\0\0\x81\x10\x15a6\x9DW[c\x01\0\0\0\x81\x10\x15a6\x90W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a6TV[` \x1C\x91`\x10\x1B\x91a6GV[`@\x1C\x91` \x1B\x91a68V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca6 V[a6\xD9\x81\x15\x15a2\xECV[\x80`\x01`\x01`\x80\x1B\x03\x10`\x07\x1B\x81\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\x80\x1B\x81\x10\x15a\x026W`\x01`\x01`\x80\x1B\x03\x16\x90V[d\x01\0\0\0\0\x81\x10\x15a\x026Wc\xFF\xFF\xFF\xFF\x16\x90V[cNH{q`\xE0\x1B`\0R`Q`\x04R`$`\0\xFD[\x80`\x02\x14a7\xD0W`\x01\x03a7dW\x80` \x80a7\x9C\x93Q\x83\x01\x01\x91\x01a+\xFAV[\x90\x81R`\xA0a7\xAA\x82a$rV[\x91\x01Q`\x01\x81\x01\x90`\0`\x01\x83\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x16\xD4Wa\x0E\x94\x91a\x16\xFCV[P\x80` \x80a7\xE4\x93Q\x83\x01\x01\x91\x01a+\xFAV[\x90` \x82\x01R`\xA0a7\xAA\x82a$rV\xFE\xA2dipfsX\"\x12 f\xA6I\x97\x99\xA0+{\xCC\xF1\xC4zN\x06\xF2F$~\xEC\x1E\xA5\xD9\xD3\xF22\x0BEOo\xD2e\x1AdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static NORMALSTRATEGY_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x12BW`\x005`\xE0\x1C\x80c\x16\xED\xE0\x16\x14a\0\xDCW\x80c\x19\x05x\x07\x14a\0\xD7W\x80c4\xDB\xC7;\x14a\0\xD2W\x80c9CMZ\x14a\0\xCDW\x80cE-/\x18\x14a\0\xC8W\x80cK\xF3F\xBF\x14a\0\xC3W\x80c\x80\xAF\x9Dv\x14a\0\xBEW\x80c\xA4G\x89\x19\x14a\0\xB9W\x80c\xE0hx\x7F\x14a\0\xB4W\x80c\xE31\xBA4\x14a\0\xAFW\x80c\xE6\x04{\x19\x14a\0\xAAW\x80c\xECshT\x14a\0\xA5Wc\xF0{\x87\x9E\x03a\x12BWa\x0F\x05V[a\x0E\x97V[a\x0E6V[a\x0C\xFCV[a\x0B\xD9V[a\n\xAEV[a\t\xFFV[a\t)V[a\x06\xCAV[a\x05\x85V[a\x05\x10V[a\x02{V[a\x01\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xCBW`\x006`\x03\x19\x01\x12a\x01\xC6W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[a\x011V[a\0\xE1V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x026WV[`\0\x80\xFD[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x026WV[\x80\x15\x15\x03a\x026WV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x026WV[`\xC45\x90a\x02y\x82a\x02[V[V[4a\x01\xCBW`\x806`\x03\x19\x01\x12a\x01\xC6Wa\x02\x94a\x02 V[`$5a\x02\xA0\x81a\x02QV[`d5\x91a\x02\xAD\x83a\x02[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01Ra\x01\0\x91\x90\x82\x81`$\x81\x85Z\xFA\x92\x83\x15a\x04\x94W`\0\x93a\x04\xE1W[PP\x80;\x15a\x04\x99W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x84\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x91`\x80\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x94W`\0\x93a\x04\xB1W[P\x85\x15a\x04\x9EWa\x03t`\xFFa\x03n\x84\x86\x01Q`\xFF\x16\x90V[\x16a\x18\x14V[\x96a\x03\xDAa\x03\xA2a\x03\x98\x88`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x99`D5\x02a76V[\x96a\x03\xBDa\x03\xAEa\x08tV[`\x01`\x01`\x80\x1B\x03\x90\x99\x16\x89RV[`\x01\x88\x86\x01R`\0`@\x89\x01R`\x01`\x01`@\x1B\x03\x16``\x88\x01RV[\x86\x15\x15`\x80\x87\x01R\x81;\x15a\x04\x99W\x82`\x04\x92`@Q\x93\x84\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x80\x15a\x04\x94Wa\x04R\x98a\x04A\x97`\xFF\x97a\x04.\x95`\0\x94a\x04eW[Pa\x04(B\x93a\x14,V[\x90a(\xCFV[\x94\x15a\x04VWP``\x01Q`\xFF\x16a\x03nV[`@Q\x91\x04\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x01Q`\xFF\x16a\x03nV[a\x03nV[a\x04\x86\x91\x94P\x87=\x89\x11a\x04\x8DW[a\x04~\x81\x83a\x08SV[\x81\x01\x90a\x17\xACV[\x928a\x04\x1DV[P=a\x04tV[a\x15\xB3V[a\x14{V[a\x03t`\xFFa\x04```\x86\x01Q`\xFF\x16\x90V[a\x04\xD3\x91\x93P`\x80=\x81\x11a\x04\xDAW[a\x04\xCB\x81\x83a\x08SV[\x81\x01\x90a\x17>V[\x918a\x03UV[P=a\x04\xC1V[a\x05\x01\x92\x93P\x80=\x10a\x05\tW[a\x04\xF9\x81\x83a\x08SV[\x81\x01\x90a\x15\x04V[\x908\x80a\x03\x16V[P=a\x04\xEFV[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6W`\x01`\x01`@\x1B\x03a\x051a\x02 V[\x16`\0R`\0` R`\xA0`@`\0 T`\xFF`@Q\x91`\x01`\x01`\x80\x1B\x03\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x80\x82`\x80\x1C\x16` \x85\x01R\x80\x82\x86\x1C\x16`@\x85\x01R\x81`\xC0\x1C\x16``\x84\x01R`\xE0\x1C\x16\x15\x15`\x80\x82\x01R\xF3[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6Wa\x05\x9Ea\x02 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x94Wa\x04R\x93a\x06=\x93`\0\x93a\x06MW[PPa\x062a\x067\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x14,V[\x90a#\x82V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x067\x92\x93Pa\x062\x91\x81a\x06m\x92\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x92\x91a\x06\x10V[\x90\x93\x92\x91\x93``\x82R\x80Q\x90\x81``\x84\x01R`\0[\x82\x81\x10a\x06\xB4WPP\x90`\x80\x82`\0\x82`@\x95\x85\x01\x01R`\x1F\x80\x19\x91\x01\x16\x82\x01\x01\x94` \x82\x01R\x01RV[\x80` \x80\x92\x84\x01\x01Q`\x80\x82\x87\x01\x01R\x01a\x06\x89V[4a\x01\xCBW`\xA06`\x03\x19\x01\x12a\x01\xC6Wa\x04R`d5a\x06\xEA\x81a\x02QV[a\x07\x9Aa\x06\xF8`\x045a76V[a\x07\x03`$5a7NV[\x92a\x07\x0F`D5a7NV[`\x01`\x01`\x80\x1B\x03`@Q\x93a\x07$\x85a\x08\x18V[\x16\x94\x85\x84R` \x84\x01\x90c\xFF\xFF\xFF\xFF\x92\x83\x80\x92\x16\x83R\x81`@\x87\x01\x91\x16\x81R\x81``\x87\x01\x93`\0\x85R`\x80\x88\x01\x96\x15\x15\x87R`@Q\x99` \x8B\x01RQ\x16`@\x89\x01RQ\x16``\x87\x01RQ\x16`\x80\x85\x01RQ\x15\x15`\xA0\x84\x01R`\xA0\x83Ra\x07\x89\x83a\x088V[a\x07\x95`\x845\x91a UV[a\x1D\xBEV[`@\x93\x91\x93Q\x93\x84\x93\x84a\x06tV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@RV[a\x08\x02V[`\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@RV[`@Q\x90a\x02y\x82a\x08\x18V[`@Q\x90a\x02y\x82a\x088V[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x083W`@Q\x91a\x08\xB7`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x08SV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x08\xD4W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FABI decoding: invalid byte array`D\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xCBW`@6`\x03\x19\x01\x12a\x01\xC6W`$5`\x01`\x01`@\x1B\x03\x81\x11a\t\x98W6`#\x82\x01\x12\x15a\t\x93Wa\t\x81\x90a\x07\x95a\t|a\tw`\x045\x936\x90`$\x81`\x04\x015\x91\x01a\x08\x8EV[a\"\xECV[a UV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[a\x07\xA9V[a\x01\xD0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x026WV[4a\x01\xCBW6`\x03\x19\x01`\xE0\x81\x12a\x01\xC6W`\xA0\x13a\n\xA9Wa\x04Ra\n\x8A`@Qa\n*\x81a\x08\x18V[`\x045a\n6\x81a\t\xEEV[\x81R`$5a\nD\x81a\t\xEEV[` \x82\x01R`D5a\nU\x81a\x02QV[`@\x82\x01Ra\nba\x02;V[``\x82\x01R`\x845a\ns\x81a\x02QV[`\x80\x82\x01Ra\n\x80a\x02lV[\x90`\xA45\x90a\x18\xCAV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[a\t\x9DV[4a\x01\xCBW`\x806`\x03\x19\x01\x12a\x01\xC6Wa\n\xC7a\x02 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01Ra\x01\0\x92\x90\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x04\x94Wa\x0B\x93\x93`\0\x93a\x0B\xB2W[PPa\x062a\x067\x91a\x0B\\a\x0BO`D5a76V[`\x01`\x01`\x80\x1B\x03\x16\x85RV[a\x0Bza\x0Bj`d5a76V[`\x01`\x01`\x80\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x0B\x9F\x81`$5a\x17\x15V[`@\x80Q\x91\x15\x15\x82R` \x82\x01\x92\x90\x92R\xF3[a\x067\x92\x93Pa\x062\x91\x81a\x0B\xD2\x92\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x92\x91a\x0B8V[4a\x01\xCBW`@6`\x03\x19\x01\x12a\x01\xC6Wa\x0B\xF2a\x02 V[`$5`\x01`\x01`@\x1B\x03\x80\x82\x11a\t\x98W6`#\x83\x01\x12\x15a\t\x93W\x81`\x04\x015\x90\x81\x11a\x0C\xA3W6`$\x82\x84\x01\x01\x11a\x0CJWa\x04R\x92`$a\x0C8\x93\x01\x90a\x12\xA5V[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6Wa\r\x15a\x02 V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01R\x91a\x01\0\x91\x82\x90\x84\x90`$\x90\x82\x90Z\xFA\x90\x81\x15a\x04\x94Wa\x04R\x93a\x06=\x93`\0\x93a\x0E\x0BW[PPa\r\xAEa\x062a\x0E\x05\x92`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91a\r\xC3a\r\xBB\x84a UV[\x93B\x90a\"\x89V[`\x80\x84\x01Ra\r\xFFa\r\xE4`@a\r\xF0a\r\xE4\x85Q`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x93\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a5(V[\x90a\x1C\xE3V[a\x0E\x05\x92\x93Pa\x0E.a\r\xAE\x92\x82a\x062\x93\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x93\x92Pa\r\x89V[4a\x01\xCBW` 6`\x03\x19\x01\x12a\x01\xC6Wa\x0EOa\x02 V[P` `@Q`\x01\x81R\xF3[``\x90`\x03\x19\x01\x12a\x01\xC6W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x026W\x90`$5a\x0E\x87\x81a\x02QV[\x90`D5a\x0E\x94\x81a\x02[V[\x90V[4a\x01\xCBWa\x0E\xA56a\x0E[V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0E\xF3Wa\x0E\xDF\x91a\x15\xBFV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`@Qc:#%k`\xE2\x1B\x81R`\x04\x90\xFD[4a\x01\xCBWa\x0F\x136a\x0E[V[Pa\x0F\x1Ca\x18BV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x99W`@\x80Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01Ra\x01\0\x94\x91\x92\x91\x90\x85\x81`$\x81\x86Z\xFA\x95\x86\x15a\x04\x94W`\0\x96a\x12#W[PP\x81;\x15a\x04\x99W\x82Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x82\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x92`\x80\x90\x84\x90\x81\x80`$\x81\x01\x03\x91Z\xFA\x80\x15a\x04\x94Wa\x04R\x96\x85\x94`\0\x92a\x12\x03W[Pa\x10\x04a\x0F\xFAa\t|a\x062\x87`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[`@\x01Q\x90`\0\x90V[\x95\x90a\x10\x0Ea\x08tV[\x98`\0\x8ARa\x106\x86\x8B\x01\x97`\0\x89R`\0\x85\x8D\x01R``\x8C\x01\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x15\x80\x15`\x80\x8B\x01Ra\x11\x84WPa\x11\0``a\x11\x0C\x94a\x11\x1Ea\x11\x11a\x11\x0C\x87a\x11\x06a\x11\0a\x10\xF6a\x113\x9Fa\r\xE4a\x11&\x9F\x9Da\x10\xEF\x8F\x93a\x11\x06\x9Fa\r\xE4a\x10\xDBa\x10\xC7\x97a\x10\xCCa\x10\xC7\x88a\x10\xC1a\r\xE4a\x10\xB3a\x10\xAEa\x10\xE9\x9A`\x01`\x01`\x80\x1B\x03\x9E\x01\x9E\x8FQ`\x01`\x01`\x80\x1B\x03\x16\x90V[a4\xECV[\x92Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x17\xF9V[a\x17\xD4V[\x9C\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a5\x07V[\x91\x16a\x17\xF9V[\x9A\x87\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x90a\x18(V[a76V[`\x01`\x01`\x80\x1B\x03\x16\x8CRV[\x01Q`\xFF\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90RV[Q\x91\x82\x91\x82\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[a\x11\xFE\x96P\x84a\x11\x0C\x94a\x11\x1Ea\x11\x11a\x11\x0Ca\x11\xEC`\x01`\x01`\x80\x1B\x03a\x11\xE6a\x11\xD8a\x10\xC7\x8Ca\x11&\x9Fa\r\xE4a\x10\xC1\x91a\x11\x06\x9Fa\x11\0\x9Fa\x10\xE9a\r\xE4a\r\xF0\x93\x88\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16a\x17\xD4V[\x97a\x11\x06a\x11\0``\x87\x01Q`\xFF\x16\x90V[a\x113V[a\x12\x1C\x91\x92P`\x80=\x81\x11a\x04\xDAWa\x04\xCB\x81\x83a\x08SV[\x908a\x0F\xD3V[a\x12:\x92\x96P\x80=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x938\x80a\x0F\x8AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x913\x83\x90\x03a\x0E\xF3Wa\twa\x12\xEB\x91a\x13X\x936\x91a\x08\x8EV[a\x13\x08\x84`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x81Q`\x01`\x01`\x80\x1B\x03\x16` \x83\x01Qc\xFF\xFF\xFF\xFF\x16\x91`\x01`\x01`\x80\x1B\x03a\x13E`\x80a\x13=`@\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x96\x01Q\x15\x15\x90V[\x94c\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x92\x16\x90a \xDFV[\x7F\x8Di\xFAt\x9A\xF4\x15\xF9<\x96\x86>\xBF6\x92\x93&#\xC0\x10\xEB\xA8\x11\x8B\x02\xF0DXZ\xABe\x13`\x01`\x01`@\x1B\x03a\x13\xA1a\x062\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x93a\x14$a\x13\xB6\x86Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95a\x13\xC8` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x13\xE7`\x80a\x13\xDF`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92\x01Q\x15\x15\x90V[\x91`@Q\x95\x86\x95\x16\x98\x85\x92\x91``\x92\x95\x94\x91\x95`\x01`\x01`\x80\x1B\x03`\x80\x86\x01\x97\x16\x85Rc\xFF\xFF\xFF\xFF\x80\x92\x16` \x86\x01R\x16`@\x84\x01R\x15\x15\x91\x01RV[\x03\x90\xA3`\0\x90V[\x90`@Qa\x149\x81a\x08\x18V[`\x80`\xFF\x82\x94T`\x01`\x01`\x80\x1B\x03\x81\x16\x84Rc\xFF\xFF\xFF\xFF\x80\x82\x85\x1C\x16` \x86\x01R\x80\x82`\xA0\x1C\x16`@\x86\x01R\x81`\xC0\x1C\x16``\x85\x01R`\xE0\x1C\x16\x15\x15\x91\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90a\x02y\x82a\t\xEEV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x026WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\x026WV[Q\x90a\x02y\x82a\x02[V[\x80\x91a\x01\0\x92\x83\x91\x03\x12a\x01\xC6W`@Q\x91\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083Wa\x15\xAB\x91`\xE0\x91`@Ra\x15=\x81a\x14\xCEV[\x84Ra\x15K` \x82\x01a\x14\xCEV[` \x85\x01Ra\x15\\`@\x82\x01a\x14\xCEV[`@\x85\x01Ra\x15m``\x82\x01a\x14\xD9V[``\x85\x01Ra\x15~`\x80\x82\x01a\x14\xEAV[`\x80\x85\x01Ra\x15\x8F`\xA0\x82\x01a\x14\xEAV[`\xA0\x85\x01Ra\x15\xA0`\xC0\x82\x01a\x14\xF9V[`\xC0\x85\x01R\x01a\x14\xF9V[`\xE0\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90\x83;\x15a\x04\x99W`@Q\x80\x94c\"i|!`\xE2\x1B\x82R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x83\x01R\x81`$a\x01\0\x97\x88\x93Z\xFA\x94\x85\x15a\x04\x94W`\0\x95a\x16tW[PP\x90a\x16^a\x16Ta\x062a\x16d\x94`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91B\x90\x83\x87a%0V[\x93a,JV[a\x16nW`\x01\x91\x90V[`\0\x91\x90V[a\x16d\x93\x92\x95Pa\x062a\x16\x9Ba\x16^\x93\x83a\x16T\x94\x90=\x10a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x96\x93\x94PPa\x16+V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x16\xD4WV[a\x16\xA5V[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x16\xD4WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x16\xD4WV[a\x17!\x90`\x01\x92a\x16\xFCV[\x12a\x17+W`\x01\x90V[`\0\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x026WV[\x90\x81`\x80\x91\x03\x12a\x01\xC6W`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083Wa\x17\xA4\x91``\x91`@R\x80Qa\x17x\x81a\x02[V[\x84Ra\x17\x86` \x82\x01a\x170V[` \x85\x01R`@\x81\x01Qa\x17\x99\x81a\x02[V[`@\x85\x01R\x01a\x170V[``\x82\x01R\x90V[\x90\x81` \x91\x03\x12a\x01\xC6WQ\x90V[\x90`\x12\x03`\x12\x81\x11a\x16\xD4Wa\x17\xD0\x90a\x18\x06V[\x02\x90V[`\0\x19\x81\x01\x91\x90\x82\x11a\x16\xD4WV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x16\xD4WV[\x91\x90\x82\x03\x91\x82\x11a\x16\xD4WV[`M\x81\x11a\x16\xD4W`\n\n\x90V[`\x12\x03`\x12\x81\x11a\x16\xD4Wa\x0E\x94\x90a\x18\x06V[\x90`\x12\x03`\x12\x81\x11a\x16\xD4Wa\x18=\x90a\x18\x06V[\x90\x04\x90V[`@Q\x90a\x18O\x82a\x08\x18V[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x083W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[``\x81\x01\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94\x93\x91\x92\x90`\x01`\x01`@\x1B\x03\x16\x92\x85;\x15a\x04\x99W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x94\x90\x94\x16`\x04\x85\x01Ra\x01\0\x90\x81\x85`$\x81\x8AZ\xFA\x94\x85\x15a\x04\x94W`\0\x95a\x1B\xB1W[Pa\x19xa\x19ma\x19a\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[` \x1Cb\xFF\xFF\xFF\x16\x90V[\x90\x87;\x15a\x04\x99W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`\x80\x80\x83`$\x81\x8CZ\xFA\x93\x84\x15a\x04\x94Wa\x11&`\xA0a\x1A\xAD\x93a\x1A\xBA\x97a\x0Bz\x97`\0\x91a\x1B\x94W[Pa\x19\xCDa\x18mV[\x94a\x19\xDA\x83\x8C\x01Q\x15\x15\x90V[\x15a\x1B1W`@\x82a\x1A\x03a\x1A=\x93a\x19\xFA` a\x1AM\x97\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x8A\x01RV[a\x1A\x1Fa\x1A\x14``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x8A\x01RV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x89\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x86\x01RV[a\x1Aaa\r\xE4\x8AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81\x85\x01Ra\x1A\xA6a\x1A\x99a\x1A\x90` \x8C\x01\x96a\x1A\x87a\r\xE4\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x82\x01Ra\x1B\xD0V[\x92\x83\x01Qa76V[`\x01`\x01`\x80\x1B\x03\x16\x8ARV[\x01Qa76V[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92\x85;\x15a\x04\x99W` `\x04\x96`@Q\x97\x88\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x93\x84\x15a\x04\x94Wa\x1A\xFD\x96`\0\x95a\x1B\rW[Pa\x1A\xF7\x90a\x14,V[\x90a%\xFDV[\x90\x91Pa\x1B\n\x81\x83a\x17\x15V[\x92V[a\x1A\xF7\x91\x95Pa\x1B*\x90` =\x81\x11a\x04\x8DWa\x04~\x81\x83a\x08SV[\x94\x90a\x1A\xEDV[\x81a\x1BSa\x1A=\x92a\x1BJ``a\x1B\x8F\x96\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x89\x01RV[a\x1Boa\x1Bd` \x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x89\x01RV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01RQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x1AMV[a\x1B\xAB\x91P\x82=\x84\x11a\x04\xDAWa\x04\xCB\x81\x83a\x08SV[8a\x19\xC4V[a\x1B\xC9\x91\x95P\x82=\x84\x11a\x05\tWa\x04\xF9\x81\x83a\x08SV[\x938a\x19IV[a\x1B\xD8a\x18mV[P`@\x81\x01\x80Q\x90a\x1B\xF4a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90a\x17\xBBV[\x90R``\x82\x01a\x1C\n\x81Q`\xFF\x84Q\x16\x90a\x17\xBBV[\x90Ra\x1C!`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90a\x17\xBBV[\x90R`\xA0\x81\x01a\x1C;\x81Q`\xFFa\x01 \x85\x01Q\x16\x90a\x17\xBBV[\x90R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x16\xD4WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x16\xD4W`\0\x19\x83\x05\x03a\x16\xD4WV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x16\xD4WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x16\xD4W\x81\x84\x05\x14\x90\x15\x17\x15a\x16\xD4WV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x16\xD4WV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x80\x83\x10\x15a\x1D~W\x82\x15a\x1DpWa\x1D\x07`\x80\x83\x01Qa4\xC5V[\x90a\x1D\x11\x83a\x1D\x86V[\x93\x81\x03\x90\x80\x82\x11a\x16\xD4Wa\x1Da`@\x93g\x1B\xC1mgN\xC8\0\0a\x1DZa\x1DFa\x0E\x94\x99a\x1DAa\x1Dg\x98a-\xE5V[a\x1C\x97V[\x92a\x1DU``\x8A\x01Q\x80a\x1C\xBAV[a\x1C\xBAV[\x04\x90a\x16\xFCV[\x05a1hV[\x91\x01Q\x90a5\x07V[PPP`\x01`\x01`\x80\x1B\x03\x90V[PPP`\0\x90V[a\x1D\x9Ba\x1D\x96`\x80\x83\x01Qa4\xC5V[a6\x10V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x16\xD4W``a\x0E\x94\x92\x01Qa5\x07V[`@\x81\x01Q`\0\x93\x92a\x1D\xD1\x91\x90a5(V[\x80a\x1D\xE4W[Pa\x0E\x94\x90\x83\x81Ra\x1F\xDAV[a\x1D\xF3a\x1E\x1D\x91\x94\x92\x94a3$V[a\x1E=a\x1E\x03`\x80\x87\x01Qa4\xC5V[a\x1E7``\x88\x01Qg\x1B\xC1mgN\xC8\0\0\x95\x81\x87\x92a\x1C\xBAV[\x04\x91a\x1E1a\x1E+\x8Aa\x1D\x86V[\x95a\x1C@V[\x92a\x1C\x97V[\x90a\x1F\xBEV[\x90\x80\x15a\x1E\xABW`\x01`\xFF\x1B\x82\x14`\0\x19\x82\x14\x16a\x16\xD4Wa\x1Exg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1Era\x1E}\x93a\x1E\x82\x95\x05a\x1C@V[\x05a,vV[a,\xAFV[a\x1C@V[\x05g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x03\x92\x90\x91\x12\x80\x15\x82\x84\x13\x16\x91\x83\x12\x16\x17a\x16\xD4W\x91a\x0E\x94a\x1D\xD7V[a\x1C\xCDV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x16\xD4WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x16\xD4WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x16\xD4WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x16\xD4WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x16\xD4WV[`@\x81\x01\x80Q\x82Q\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x10\x15a KW\x15a DWPa \x04\x83a\x1D\x86V[\x90\x83Q\x81\x03\x90\x81\x11a\x16\xD4Wa\x0E\x94\x93`\xA0a .a <\x94a )a 7\x95a-\xE5V[a\x16\xFCV[\x91\x01Q\x90a\x1F\xBEV[a,\x87V[\x90Q\x90a5\x07V[\x92PPP\x90V[PPPPP`\0\x90V[`\xA0`@Qa c\x81a\x088V[`\0\x91\x81\x83\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x01R`\x01`\x01`\x80\x1B\x03\x82Q\x16\x91c\xFF\xFF\xFF\xFF`@a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x83` \x86\x01Q\x16\x02\x04\x92\x01Q\x16\x90`@Q\x93a \xBF\x85a\x088V[\x83\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[\x92\x93\x91\x90\x93c\xFF\xFF\xFF\xFF\x92\x83a \xFD\x86Tc\xFF\xFF\xFF\xFF\x90`\xC0\x1C\x16\x90V[\x16a\"wW\x80\x15a\"\x16W\x84T`\xFF`\xE0\x1B\x19\x16\x90\x15\x15`\xE0\x1B`\xFF`\xE0\x1B\x16\x17\x84UP\x82Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16c\x01\xE1\x85Y`\xA0\x1B\x17\x83U[aa\xA8\x80\x82\x10\x90\x82\x14\x17`\x01\x82\x11`\x01\x83\x14\x17\x16\x15a\"\x04Wa!^a!~\x91a7NV[\x83Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x80\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\x80\x1B\x16\x17\x83UV[`\x01`\x01`\x80\x1B\x03\x80\x84\x10\x90\x84\x14\x17`\x01\x84\x11`\x01\x85\x14\x17\x16\x15a!\xF2Wa!\xD0a!\xABa\x02y\x94a76V[\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x83UV[\x81Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16B\x91\x90\x91\x16`\xC0\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x17\x90UV[`@Qc\xB2B\xE3A`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9]8\x19`\xE0\x1B\x81R`\x04\x90\xFD[Pb\x01Q\x80\x81\x81\x14\x90\x82\x11\x17c\x05\xA4\x90\x0B\x82\x81\x14\x90\x83\x10\x17\x16\x15a\"eWa\"@a\"`\x91a7NV[\x84Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x84UV[a!9V[`@Qc\xB5\x97\x03\x0F`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x1E\x13\x89\xA1`\xE2\x1B\x81R`\x04\x90\xFD[`\x80\x81\x01Qa\"\xAEWa\"\xA0c\xFF\xFF\xFF\xFF\x91a\"\xB8V[\x16\x80\x82\x10\x90\x82\x03\x02\x81\x03\x03\x90V[PPc\x01\xE1\x85Y\x90V[`\x80\x81\x01Qa\"\xDAWc\xFF\xFF\xFF\xFF\x90\x81`@\x81``\x84\x01Q\x16\x92\x01Q\x16\x01\x16\x90V[`@Qc\xB0\x19\x84\x97`\xE0\x1B\x81R`\x04\x90\xFD[a\"\xF4a\x18BV[P`\xA0\x81Q\x03a#pW`\xA0\x81\x80Q\x81\x01\x03\x12a\x01\xC6W`\xA0`@Q\x91a#\x1A\x83a\x08\x18V[` \x81\x01Qa#(\x81a\t\xEEV[\x83Ra#6`@\x82\x01a\x14\xD9V[` \x84\x01Ra#G``\x82\x01a\x14\xD9V[`@\x84\x01Ra#X`\x80\x82\x01a\x14\xD9V[``\x84\x01R\x01Qa#h\x81a\x02QV[`\x80\x82\x01R\x90V[`@Qc\x01YW\xD3`\xE2\x1B\x81R`\x04\x90\xFD[a\x0E\x94\x91a#\x9Aa\r\xE4\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91`\x01`\x01`\x80\x1B\x03a#\xE4a#\xC7a#\xC0a\r\xE4`@\x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x96a5(V[\x94a#\xDFa\r\xE4` \x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a5(V[\x92a$Ia#\xF9\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a$Cc\xFF\xFF\xFF\xFF``a$6a$$a$\x1B` \x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[g\r\xE0\xB6\xB3\xA7d\0\0a'\x10\x91\x02\x04\x90V[\x97\x01Q\x16c\xFF\xFF\xFF\xFF\x16\x90V[\x90a\"\x89V[\x93a$Ra\x08\x81V[\x95\x86R` \x86\x01R\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\0`\xA0\x82\x01R[a\x0E\x94\x90a$\xD6a$\x82\x82a\x1D\x86V[\x91a$\x91\x81`@\x01Q\x90`\0\x90V[\x82Q\x90\x92\x91\x90g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a%\x0CWP`\x01\x92[` \x83\x01Q\x91\x82\x10a$\xDBWPPPa )a$\xD0g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92[a-\xE5V[\x91a-\xE5V[a\x1F\xBEV[\x81\x11a$\xF1WPPa )a$\xD0`\x01\x92a-\xE5V[a%\x06a$\xD0\x91`@a )\x94\x01Q\x90a5\xE0V[\x92a-\xE5V[\x80a%!WPg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92a$\xABV[a%*\x90a\x17\xE3V[\x92a$\xABV[\x90`\x01`\x01`\x80\x1B\x03a\x0E\x94\x94\x92a%Ra\r\xE4\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a%ia\r\xE4` \x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95\x15a%\xCDWa%\x93a%\x8Ca\r\xE4`@a%\x99\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a5(V[\x95a5\xE0V[\x92[a$Ia%\xAF\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a%\xC7a$$a$\x1B` \x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x94a\"\x89V[a%\xF1a%\xEAa\r\xE4`@a%\xF7\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a5\xE0V[\x95a5(V[\x92a%\x9BV[a\r\xFFa\r\xE4a'\x15\x93a'\x06`@a&\x8Ba&&\x9A\x99\x98\x99a&-\x8A`\x80\x8D\x01\x9D\x8EQ\x15\x15\x90V[\x87\x8Ba%0V[\x9Ca&?\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[` \x8A\x01Q`\xC0\x8B\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x03a':W`\xA0\x8A\x01Qa\xFF\xFF\x16\x9C[`\x01`\x01`\x80\x1B\x03\x9Da\xFF\xFF\x16\x92\x8E\x16\x91\x8E\x16\x90a'IV[\x98\x92P\x94\x90Pa&\xC3a&\xA5\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9Aa&\xBDa$$a$\x1B` \x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92a\"\x89V[\x90a&\xCCa\x08\x81V[\x9B`\0\x8DR` \x8D\x01\x9B`\0\x8DR\x16\x84\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\0`\xA0\x8B\x01R\x01\x91a\r\xFFa\r\xE4\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87RQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81Ra'*a'#\x83a$rV[\x93Q\x15\x15\x90V[\x15a'4WPQ\x92V[\x90PQ\x92V[`\x80\x8A\x01Qa\xFF\xFF\x16\x9Ca&rV[\x91\x94\x92\x94`\0a'd`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16a5\x92V[\x95\x80\x15\x80\x15a(\x97W[PP\x80\x95\x80\x97`\x80\x86\x01\x92a'\xBAa'\x86\x85Q\x15\x15\x90V[\x93\x84\x15a(\x90W\x87\x94[\x15a(\x86Wa'\xB5\x87\x95[a'\xAFa\r\xE4\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a(\xB3V[a\x17\xF9V[\x90\x81\x81\x11a(tWa'\xCF` \x91\x85\x93a\x17\xF9V[\x97\x01\x91a'\xE3\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11a(bWa(\x0B\x91a\x10\xC1a\r\xE4a(\x04\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91Q\x15\x15\x90V[\x90\x81\x15a(YW\x84\x85\x92[\x15a(QWP\x92[\x14a(?W\x81\x14a(-W\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92a(\x1EV[\x80\x94\x85\x92a(\x16V[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[a'\xB5\x88\x95a'\x9BV[\x86\x94a'\x90V[\x90\x91Pa\x1E\xABW\x85\x04\x94\x85\x81\x03\x90\x81\x11a\x16\xD4W\x948\x80a'nV[\x91\x90\x82\x01\x80\x92\x11a\x16\xD4WV[`\0\x19\x81\x14a\x16\xD4W`\x01\x01\x90V[\x93\x91a)\0a(\xE8`\x80\x93\x97a)\x0B\x95\x87\x85\x8B\x8Ba%\xFDV[P\x95\x90\x97a(\xF5\x81a UV[\x96`\xA0\x88\x01Ra\"\x89V[\x82\x85\x01R\x01Q\x15\x15\x90V[\x92\x83\x15a*=W\x81Ra)\x1D\x81a\x1F\xDAV[\x80` \x83\x01R[\x80\x15a*\x0CW\x91a)\xDBa)\xC4a)\xBFa)\xA3`\x01`\x01`\x80\x1B\x03\x95\x87a)\xB1a)Za)Ta\x0E\x94\x9C\x9Ba5JV[\x92a5\xB8V[\x92`@Q\x94\x85\x91` \x83\x01\x91\x90\x91`\xA0\x80`\xC0\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x85R\x84a\x08SV[\x88\x15a*\x04W`\x02\x92a*SV[a(\xC0V[a\x10\xE9a\r\xE4`@\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x92\x15a)\xF5W` \x01Q`\x01`\x01`\x80\x1B\x03\x16[\x16a\x17\xF9V[Q`\x01`\x01`\x80\x1B\x03\x16a)\xEFV[`\x01\x92a*SV[PP`\x01`\x01`\x80\x1B\x03\x91`\0\x14a*/W` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[` \x82\x01Ra*K\x81a+\x82V[\x80\x82Ra)$V[\x91\x90\x93\x92\x93`\0\x94`\0\x92\x80\x83\x11a+aWa*p\x83\x86\x84a7zV[\x90a*|\x81\x87\x85a7zV[\x85a*\x87\x82\x85a\x1C\x97V[\x13a+@WP\x95\x94\x93a*\x9A\x84\x88a\x17\xF9V[\x94\x81`\x01\x96\x87\x80[a*\xB4W[PPPPPPPPP\x90PV[\x15a+\x1BW[P\x86\x97\x98\x99P\x80\x92a*\xD5a*\xCF\x8C\x89a(\xB3V[`\x01\x1C\x90V[\x9A\x8B\x90a*\xE3\x8D\x86\x8Aa7zV[\x90\x84a*\xEF\x89\x84a\x1C\x97V[\x13a+\rWPP\x93[\x88a+\x03\x89\x87a\x17\xF9V[\x92\x01\x94\x9A\x99a*\xA2V[\x95\x96P\x97PP\x8A\x96\x94a*\xF8V[\x87\x10\x80a+5W[\x15a+.W\x89a*\xBAV[\x80\x80a*\xA7V[Pa\x01\0\x83\x10a+#V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@\x81\x01\x80Q\x91` \x81\x01\x92\x83Q\x90\x81\x10\x15a+\xF1W\x15a+\xE2W`\xA0a+\xC3a+\xCC\x94a$\xD6a$\xCBa 7\x96a+\xB9\x87a\x1D\x86V[\x93Q\x90Q\x90a5\xE0V[\x91\x01Q\x90a\x16\xFCV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x16\xD4W\x90V[PPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[PPPP`\0\x90V[\x90\x81`\xC0\x91\x03\x12a\x01\xC6W`\xA0`@Q\x91a,\x14\x83a\x088V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R\x01Q`\xA0\x82\x01R\x90V[`\x80\x82\x01Qa,oW``\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91a,i\x90a\"\xB8V[\x16\x11\x15\x90V[PP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\x16\xD4W`\0\x03\x90V[a,\xABa\x1E}a\x1Exg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1Erg\x1B\xC1mgN\xC8\0\0\x95a\x1C@V[\x05\x90V[\x80\x15a-\xD8WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a-\xD2WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a-\xC5Wa-\xB3a,\xE2\x82a/\x90V[a-va-\xAEa-\x01a,\xFCa,\xF7\x85a5gV[a\x1E\xB0V[a1'V[\x92a$\xD6a-\xA9a-\xA4a-\x9Da-\x97a-\x92a-\x8Ca-\x87a-\x81a-|\x8Da-va-qa-ka-fa-`a-[a-Ua-Pa-Ja-E\x8Aa/\xBDV[a\x1E\xC8V[\x89a0_V[a\x1E\xE2V[\x87a0_V[a\x1E\xFAV[\x85a0_V[a\x1F\x14V[\x83a0_V[a\x1F,V[\x90a0_V[a\x1FFV[\x8Ca0_V[a\x1F^V[\x8Aa0_V[a\x1FvV[\x88a0_V[\x93\x80a0_V[a\x1C]V[a\x16\xBBV[a1hV[\x90`\0\x13\x15a\x0E\x94Wa\x0E\x94\x90a\x16\xD9V[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a-\xD2Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a/:W\x81\x15a/[W`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x16\xD4W`\0\x83\x12\x80\x15a/\x7FW[a/mW\x82\x15a/:Wg\x1B\xC1mgN\xC8\0\0\x83\x14a/[W\x82\x12\x91\x82\x15a/LW\x92[a.T\x84a0\xA7V[\x80\x15a/:Wa.\xB1a.\x80a.{a\x1D\x96a.va.\xB6\x95\x99\x97\x96\x99a3$V[a/\xE8V[a\x1C~V[a )a.\x94a.\x8F\x83a0\xD2V[a\x1F\x8EV[a.\xABa,\xF7a-`a.\xA6\x86a0\xFDV[a\x1F\xA6V[\x90a1FV[a0\x10V[\x93`\0\x92[\x81\x84\x10a.\xEDWPPPPa\x0E\x94\x91a.\xDA\x91`\0\x14a.\xDFWa0\x80V[a,vV[a.\xE8\x90a,vV[a0\x80V[\x90\x91a/0\x86a\x1E7a/\x05\x85a )\x86\x99\x9Ba,\xAFV[a.\xABa/ a/\x1Ba-\xAEa.\xDA\x87\x80a0_V[a08V[a/*\x83\x86a0_V[\x90a\x16\xFCV[\x95\x01\x92\x91\x90a.\xBBV[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a/U\x90a\x16\xD9V[\x92a.KV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a.'V[`\x01`\xFF\x1B\x81\x14a/\xABW`\0\x81\x12\x15a\x0E\x94W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x026Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x026W\x05\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a-\xD2Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a2\xB8We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x15a2\xF3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a3P`\0\x82\x13a2\xECV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a3l\x82a6\xCEV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026Wc\x01\xE1\x85Y\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x02\x04\x90\x81\x14`\x01\x16\x15a\x026W\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x026W\x04\x90V[`2\x81\x02\x90`2\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026W`d\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x96\x81\x02\x90`\x96\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x026W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x026W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a6\xB7W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a6\xAAW[e\x01\0\0\0\0\0\x81\x10\x15a6\x9DW[c\x01\0\0\0\x81\x10\x15a6\x90W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a6TV[` \x1C\x91`\x10\x1B\x91a6GV[`@\x1C\x91` \x1B\x91a68V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca6 V[a6\xD9\x81\x15\x15a2\xECV[\x80`\x01`\x01`\x80\x1B\x03\x10`\x07\x1B\x81\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\x80\x1B\x81\x10\x15a\x026W`\x01`\x01`\x80\x1B\x03\x16\x90V[d\x01\0\0\0\0\x81\x10\x15a\x026Wc\xFF\xFF\xFF\xFF\x16\x90V[cNH{q`\xE0\x1B`\0R`Q`\x04R`$`\0\xFD[\x80`\x02\x14a7\xD0W`\x01\x03a7dW\x80` \x80a7\x9C\x93Q\x83\x01\x01\x91\x01a+\xFAV[\x90\x81R`\xA0a7\xAA\x82a$rV[\x91\x01Q`\x01\x81\x01\x90`\0`\x01\x83\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x16\xD4Wa\x0E\x94\x91a\x16\xFCV[P\x80` \x80a7\xE4\x93Q\x83\x01\x01\x91\x01a+\xFAV[\x90` \x82\x01R`\xA0a7\xAA\x82a$rV\xFE\xA2dipfsX\"\x12 f\xA6I\x97\x99\xA0+{\xCC\xF1\xC4zN\x06\xF2F$~\xEC\x1E\xA5\xD9\xD3\xF22\x0BEOo\xD2e\x1AdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static NORMALSTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct NormalStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NormalStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NormalStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NormalStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NormalStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NormalStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NormalStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                NORMALSTRATEGY_ABI.clone(),
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
                NORMALSTRATEGY_ABI.clone(),
                NORMALSTRATEGY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `afterCreate` (0xe068787f) function
        pub fn after_create(
            &self,
            pool_id: u64,
            strategy_args: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([224, 104, 120, 127], (pool_id, strategy_args))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approximateReservesGivenPrice` (0x4bf346bf) function
        pub fn approximate_reserves_given_price(
            &self,
            price_wad: ::ethers::core::types::U256,
            strategy_args: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([75, 243, 70, 191], (price_wad, strategy_args))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `beforeSwap` (0xec736854) function
        pub fn before_swap(
            &self,
            pool_id: u64,
            sell_asset: bool,
            swapper: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, ::ethers::core::types::I256)>
        {
            self.0
                .method_hash([236, 115, 104, 84], (pool_id, sell_asset, swapper))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `configs` (0x34dbc73b) function
        pub fn configs(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, (u128, u32, u32, u32, bool)> {
            self.0
                .method_hash([52, 219, 199, 59], pool_id)
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
        ///Calls the contract's `getSpotPrice` (0xe331ba34) function
        pub fn get_spot_price(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([227, 49, 186, 52], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getStrategyData` (0x452d2f18) function
        pub fn get_strategy_data(
            &self,
            strike_price_wad: ::ethers::core::types::U256,
            volatility_basis_points: ::ethers::core::types::U256,
            duration_seconds: ::ethers::core::types::U256,
            is_perpetual: bool,
            price_wad: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Bytes,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash(
                    [69, 45, 47, 24],
                    (
                        strike_price_wad,
                        volatility_basis_points,
                        duration_seconds,
                        is_perpetual,
                        price_wad,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `portfolio` (0x16ede016) function
        pub fn portfolio(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([22, 237, 224, 22], ())
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
        ///Calls the contract's `validatePool` (0xe6047b19) function
        pub fn validate_pool(
            &self,
            pool_id: u64,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([230, 4, 123, 25], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSwap` (0xa4478919) function
        pub fn validate_swap(
            &self,
            pool_id: u64,
            invariant: ::ethers::core::types::I256,
            reserve_x: ::ethers::core::types::U256,
            reserve_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, ::ethers::core::types::I256)>
        {
            self.0
                .method_hash(
                    [164, 71, 137, 25],
                    (pool_id, invariant, reserve_x, reserve_y),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AfterCreate` event
        pub fn after_create_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AfterCreateFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Genesis` event
        pub fn genesis_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GenesisFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, NormalStrategyEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for NormalStrategy<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BisectionLib_InvalidBounds` with signature `BisectionLib_InvalidBounds(uint256,uint256)` and selector `0x6105bfb6`
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
        name = "BisectionLib_InvalidBounds",
        abi = "BisectionLib_InvalidBounds(uint256,uint256)"
    )]
    pub struct BisectionLib_InvalidBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    ///Custom Error type `BisectionLib_RootOutsideBounds` with signature `BisectionLib_RootOutsideBounds(int256,int256)` and selector `0x1bc6f974`
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
        name = "BisectionLib_RootOutsideBounds",
        abi = "BisectionLib_RootOutsideBounds(int256,int256)"
    )]
    pub struct BisectionLib_RootOutsideBounds {
        pub lower_result: ::ethers::core::types::I256,
        pub upper_result: ::ethers::core::types::I256,
    }
    ///Custom Error type `Infinity` with signature `Infinity()` and selector `0x07a02127`
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
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
    ///Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
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
    #[etherror(name = "Min", abi = "Min()")]
    pub struct Min;
    ///Custom Error type `NegativeInfinity` with signature `NegativeInfinity()` and selector `0x8bb56614`
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
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    ///Custom Error type `NormalStrategyLib_ConfigExists` with signature `NormalStrategyLib_ConfigExists()` and selector `0x784e2684`
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
        name = "NormalStrategyLib_ConfigExists",
        abi = "NormalStrategyLib_ConfigExists()"
    )]
    pub struct NormalStrategyLib_ConfigExists;
    ///Custom Error type `NormalStrategyLib_InvalidDuration` with signature `NormalStrategyLib_InvalidDuration()` and selector `0xb597030f`
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
        name = "NormalStrategyLib_InvalidDuration",
        abi = "NormalStrategyLib_InvalidDuration()"
    )]
    pub struct NormalStrategyLib_InvalidDuration;
    ///Custom Error type `NormalStrategyLib_InvalidStrategyArgs` with signature `NormalStrategyLib_InvalidStrategyArgs()` and selector `0x05655f4c`
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
        name = "NormalStrategyLib_InvalidStrategyArgs",
        abi = "NormalStrategyLib_InvalidStrategyArgs()"
    )]
    pub struct NormalStrategyLib_InvalidStrategyArgs;
    ///Custom Error type `NormalStrategyLib_InvalidStrikePrice` with signature `NormalStrategyLib_InvalidStrikePrice()` and selector `0xb242e341`
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
        name = "NormalStrategyLib_InvalidStrikePrice",
        abi = "NormalStrategyLib_InvalidStrikePrice()"
    )]
    pub struct NormalStrategyLib_InvalidStrikePrice;
    ///Custom Error type `NormalStrategyLib_InvalidVolatility` with signature `NormalStrategyLib_InvalidVolatility()` and selector `0x395d3819`
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
        name = "NormalStrategyLib_InvalidVolatility",
        abi = "NormalStrategyLib_InvalidVolatility()"
    )]
    pub struct NormalStrategyLib_InvalidVolatility;
    ///Custom Error type `NormalStrategyLib_NonExpiringPool` with signature `NormalStrategyLib_NonExpiringPool()` and selector `0xb0198497`
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
        name = "NormalStrategyLib_NonExpiringPool",
        abi = "NormalStrategyLib_NonExpiringPool()"
    )]
    pub struct NormalStrategyLib_NonExpiringPool;
    ///Custom Error type `NormalStrategy_NotPortfolio` with signature `NormalStrategy_NotPortfolio()` and selector `0xe88c95ac`
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
        name = "NormalStrategy_NotPortfolio",
        abi = "NormalStrategy_NotPortfolio()"
    )]
    pub struct NormalStrategy_NotPortfolio;
    ///Custom Error type `OutOfBounds` with signature `OutOfBounds()` and selector `0xb4120f14`
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
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum NormalStrategyErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NormalStrategyLib_ConfigExists(NormalStrategyLib_ConfigExists),
        NormalStrategyLib_InvalidDuration(NormalStrategyLib_InvalidDuration),
        NormalStrategyLib_InvalidStrategyArgs(NormalStrategyLib_InvalidStrategyArgs),
        NormalStrategyLib_InvalidStrikePrice(NormalStrategyLib_InvalidStrikePrice),
        NormalStrategyLib_InvalidVolatility(NormalStrategyLib_InvalidVolatility),
        NormalStrategyLib_NonExpiringPool(NormalStrategyLib_NonExpiringPool),
        NormalStrategy_NotPortfolio(NormalStrategy_NotPortfolio),
        OutOfBounds(OutOfBounds),
        SwapLib_OutputExceedsReserves(SwapLib_OutputExceedsReserves),
        SwapLib_ProtocolFeeTooHigh(SwapLib_ProtocolFeeTooHigh),
        SwapLib_ZeroXAdjustment(SwapLib_ZeroXAdjustment),
        SwapLib_ZeroYAdjustment(SwapLib_ZeroYAdjustment),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for NormalStrategyErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded) =
                <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) =
                <NormalStrategyLib_ConfigExists as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NormalStrategyLib_ConfigExists(decoded));
            }
            if let Ok(decoded) =
                <NormalStrategyLib_InvalidDuration as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NormalStrategyLib_InvalidDuration(decoded));
            }
            if let Ok(decoded) =
                <NormalStrategyLib_InvalidStrategyArgs as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NormalStrategyLib_InvalidStrategyArgs(decoded));
            }
            if let Ok(decoded) =
                <NormalStrategyLib_InvalidStrikePrice as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NormalStrategyLib_InvalidStrikePrice(decoded));
            }
            if let Ok(decoded) =
                <NormalStrategyLib_InvalidVolatility as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::NormalStrategyLib_InvalidVolatility(decoded));
            }
            if let Ok(decoded) =
                <NormalStrategyLib_NonExpiringPool as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NormalStrategyLib_NonExpiringPool(decoded));
            }
            if let Ok(decoded) =
                <NormalStrategy_NotPortfolio as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NormalStrategy_NotPortfolio(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NormalStrategyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NormalStrategyLib_ConfigExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidDuration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidStrategyArgs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidStrikePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_InvalidVolatility(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategyLib_NonExpiringPool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NormalStrategy_NotPortfolio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for NormalStrategyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BisectionLib_InvalidBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BisectionLib_RootOutsideBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_ConfigExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidDuration as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidStrategyArgs as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidStrikePrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_InvalidVolatility as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategyLib_NonExpiringPool as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NormalStrategy_NotPortfolio as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for NormalStrategyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NormalStrategyLib_ConfigExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidDuration(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidStrategyArgs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidStrikePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_InvalidVolatility(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategyLib_NonExpiringPool(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NormalStrategy_NotPortfolio(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_ZeroXAdjustment(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_ZeroYAdjustment(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for NormalStrategyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for NormalStrategyErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for NormalStrategyErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for NormalStrategyErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for NormalStrategyErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for NormalStrategyErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_ConfigExists> for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_ConfigExists) -> Self {
            Self::NormalStrategyLib_ConfigExists(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidDuration> for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidDuration) -> Self {
            Self::NormalStrategyLib_InvalidDuration(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidStrategyArgs> for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidStrategyArgs) -> Self {
            Self::NormalStrategyLib_InvalidStrategyArgs(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidStrikePrice> for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidStrikePrice) -> Self {
            Self::NormalStrategyLib_InvalidStrikePrice(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidVolatility> for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidVolatility) -> Self {
            Self::NormalStrategyLib_InvalidVolatility(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_NonExpiringPool> for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_NonExpiringPool) -> Self {
            Self::NormalStrategyLib_NonExpiringPool(value)
        }
    }
    impl ::core::convert::From<NormalStrategy_NotPortfolio> for NormalStrategyErrors {
        fn from(value: NormalStrategy_NotPortfolio) -> Self {
            Self::NormalStrategy_NotPortfolio(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for NormalStrategyErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    impl ::core::convert::From<SwapLib_OutputExceedsReserves> for NormalStrategyErrors {
        fn from(value: SwapLib_OutputExceedsReserves) -> Self {
            Self::SwapLib_OutputExceedsReserves(value)
        }
    }
    impl ::core::convert::From<SwapLib_ProtocolFeeTooHigh> for NormalStrategyErrors {
        fn from(value: SwapLib_ProtocolFeeTooHigh) -> Self {
            Self::SwapLib_ProtocolFeeTooHigh(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroXAdjustment> for NormalStrategyErrors {
        fn from(value: SwapLib_ZeroXAdjustment) -> Self {
            Self::SwapLib_ZeroXAdjustment(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroYAdjustment> for NormalStrategyErrors {
        fn from(value: SwapLib_ZeroYAdjustment) -> Self {
            Self::SwapLib_ZeroYAdjustment(value)
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
        name = "AfterCreate",
        abi = "AfterCreate(address,uint64,uint256,uint256,uint256,bool)"
    )]
    pub struct AfterCreateFilter {
        #[ethevent(indexed)]
        pub portfolio: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_id: u64,
        pub strike_price_wad: ::ethers::core::types::U256,
        pub volatility_basis_points: ::ethers::core::types::U256,
        pub duration_seconds: ::ethers::core::types::U256,
        pub is_perpetual: bool,
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
    #[ethevent(name = "Genesis", abi = "Genesis(address)")]
    pub struct GenesisFilter {
        #[ethevent(indexed)]
        pub portfolio: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum NormalStrategyEvents {
        AfterCreateFilter(AfterCreateFilter),
        GenesisFilter(GenesisFilter),
    }
    impl ::ethers::contract::EthLogDecode for NormalStrategyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AfterCreateFilter::decode_log(log) {
                return Ok(NormalStrategyEvents::AfterCreateFilter(decoded));
            }
            if let Ok(decoded) = GenesisFilter::decode_log(log) {
                return Ok(NormalStrategyEvents::GenesisFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for NormalStrategyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AfterCreateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenesisFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AfterCreateFilter> for NormalStrategyEvents {
        fn from(value: AfterCreateFilter) -> Self {
            Self::AfterCreateFilter(value)
        }
    }
    impl ::core::convert::From<GenesisFilter> for NormalStrategyEvents {
        fn from(value: GenesisFilter) -> Self {
            Self::GenesisFilter(value)
        }
    }
    ///Container type for all input parameters for the `afterCreate` function with signature `afterCreate(uint64,bytes)` and selector `0xe068787f`
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
    #[ethcall(name = "afterCreate", abi = "afterCreate(uint64,bytes)")]
    pub struct AfterCreateCall {
        pub pool_id: u64,
        pub strategy_args: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `approximateReservesGivenPrice` function with signature `approximateReservesGivenPrice(uint256,bytes)` and selector `0x4bf346bf`
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
        name = "approximateReservesGivenPrice",
        abi = "approximateReservesGivenPrice(uint256,bytes)"
    )]
    pub struct ApproximateReservesGivenPriceCall {
        pub price_wad: ::ethers::core::types::U256,
        pub strategy_args: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `beforeSwap` function with signature `beforeSwap(uint64,bool,address)` and selector `0xec736854`
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
    #[ethcall(name = "beforeSwap", abi = "beforeSwap(uint64,bool,address)")]
    pub struct BeforeSwapCall {
        pub pool_id: u64,
        pub sell_asset: bool,
        pub swapper: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `configs` function with signature `configs(uint64)` and selector `0x34dbc73b`
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
    #[ethcall(name = "configs", abi = "configs(uint64)")]
    pub struct ConfigsCall {
        pub pool_id: u64,
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
    ///Container type for all input parameters for the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,bool,uint256)` and selector `0x452d2f18`
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
        name = "getStrategyData",
        abi = "getStrategyData(uint256,uint256,uint256,bool,uint256)"
    )]
    pub struct GetStrategyDataCall {
        pub strike_price_wad: ::ethers::core::types::U256,
        pub volatility_basis_points: ::ethers::core::types::U256,
        pub duration_seconds: ::ethers::core::types::U256,
        pub is_perpetual: bool,
        pub price_wad: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `portfolio` function with signature `portfolio()` and selector `0x16ede016`
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
    #[ethcall(name = "portfolio", abi = "portfolio()")]
    pub struct PortfolioCall;
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
    ///Container type for all input parameters for the `validatePool` function with signature `validatePool(uint64)` and selector `0xe6047b19`
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
    #[ethcall(name = "validatePool", abi = "validatePool(uint64)")]
    pub struct ValidatePoolCall {
        pub pool_id: u64,
    }
    ///Container type for all input parameters for the `validateSwap` function with signature `validateSwap(uint64,int256,uint256,uint256)` and selector `0xa4478919`
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
        name = "validateSwap",
        abi = "validateSwap(uint64,int256,uint256,uint256)"
    )]
    pub struct ValidateSwapCall {
        pub pool_id: u64,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum NormalStrategyCalls {
        AfterCreate(AfterCreateCall),
        ApproximateReservesGivenPrice(ApproximateReservesGivenPriceCall),
        BeforeSwap(BeforeSwapCall),
        Configs(ConfigsCall),
        GetAmountOut(GetAmountOutCall),
        GetInvariant(GetInvariantCall),
        GetMaxOrder(GetMaxOrderCall),
        GetSpotPrice(GetSpotPriceCall),
        GetStrategyData(GetStrategyDataCall),
        Portfolio(PortfolioCall),
        SimulateSwap(SimulateSwapCall),
        ValidatePool(ValidatePoolCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for NormalStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AfterCreateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AfterCreate(decoded));
            }
            if let Ok(decoded) =
                <ApproximateReservesGivenPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApproximateReservesGivenPrice(decoded));
            }
            if let Ok(decoded) = <BeforeSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BeforeSwap(decoded));
            }
            if let Ok(decoded) = <ConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Configs(decoded));
            }
            if let Ok(decoded) = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded) = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded) = <GetMaxOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxOrder(decoded));
            }
            if let Ok(decoded) = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded) =
                <GetStrategyDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetStrategyData(decoded));
            }
            if let Ok(decoded) = <PortfolioCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Portfolio(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <ValidatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidatePool(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NormalStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AfterCreate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproximateReservesGivenPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Configs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInvariant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetMaxOrder(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetSpotPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetStrategyData(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Portfolio(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidatePool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for NormalStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AfterCreate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproximateReservesGivenPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BeforeSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Configs(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAmountOut(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMaxOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetStrategyData(element) => ::core::fmt::Display::fmt(element, f),
                Self::Portfolio(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidatePool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AfterCreateCall> for NormalStrategyCalls {
        fn from(value: AfterCreateCall) -> Self {
            Self::AfterCreate(value)
        }
    }
    impl ::core::convert::From<ApproximateReservesGivenPriceCall> for NormalStrategyCalls {
        fn from(value: ApproximateReservesGivenPriceCall) -> Self {
            Self::ApproximateReservesGivenPrice(value)
        }
    }
    impl ::core::convert::From<BeforeSwapCall> for NormalStrategyCalls {
        fn from(value: BeforeSwapCall) -> Self {
            Self::BeforeSwap(value)
        }
    }
    impl ::core::convert::From<ConfigsCall> for NormalStrategyCalls {
        fn from(value: ConfigsCall) -> Self {
            Self::Configs(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for NormalStrategyCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for NormalStrategyCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetMaxOrderCall> for NormalStrategyCalls {
        fn from(value: GetMaxOrderCall) -> Self {
            Self::GetMaxOrder(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for NormalStrategyCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetStrategyDataCall> for NormalStrategyCalls {
        fn from(value: GetStrategyDataCall) -> Self {
            Self::GetStrategyData(value)
        }
    }
    impl ::core::convert::From<PortfolioCall> for NormalStrategyCalls {
        fn from(value: PortfolioCall) -> Self {
            Self::Portfolio(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for NormalStrategyCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<ValidatePoolCall> for NormalStrategyCalls {
        fn from(value: ValidatePoolCall) -> Self {
            Self::ValidatePool(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for NormalStrategyCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    ///Container type for all return fields from the `afterCreate` function with signature `afterCreate(uint64,bytes)` and selector `0xe068787f`
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
    pub struct AfterCreateReturn {
        pub success: bool,
    }
    ///Container type for all return fields from the `approximateReservesGivenPrice` function with signature `approximateReservesGivenPrice(uint256,bytes)` and selector `0x4bf346bf`
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
    pub struct ApproximateReservesGivenPriceReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `beforeSwap` function with signature `beforeSwap(uint64,bool,address)` and selector `0xec736854`
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
    pub struct BeforeSwapReturn(pub bool, pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `configs` function with signature `configs(uint64)` and selector `0x34dbc73b`
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
    pub struct ConfigsReturn {
        pub strike_price_wad: u128,
        pub volatility_basis_points: u32,
        pub duration_seconds: u32,
        pub creation_timestamp: u32,
        pub is_perpetual: bool,
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
    pub struct GetInvariantReturn {
        pub invariant: ::ethers::core::types::I256,
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
    ///Container type for all return fields from the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,bool,uint256)` and selector `0x452d2f18`
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
    pub struct GetStrategyDataReturn {
        pub strategy_data: ::ethers::core::types::Bytes,
        pub initial_x: ::ethers::core::types::U256,
        pub initial_y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `portfolio` function with signature `portfolio()` and selector `0x16ede016`
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
    pub struct PortfolioReturn(pub ::ethers::core::types::Address);
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
    ///Container type for all return fields from the `validatePool` function with signature `validatePool(uint64)` and selector `0xe6047b19`
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
    pub struct ValidatePoolReturn(pub bool);
    ///Container type for all return fields from the `validateSwap` function with signature `validateSwap(uint64,int256,uint256,uint256)` and selector `0xa4478919`
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
    pub struct ValidateSwapReturn(pub bool, pub ::ethers::core::types::I256);
}
