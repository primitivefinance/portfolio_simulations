pub use normal_strategy::*;
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
pub mod normal_strategy {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("portfolio_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("afterCreate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("success"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approximateReservesGivenPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "approximateReservesGivenPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("priceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("beforeSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("configs"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("configs"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        128usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint128"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAmountOut"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("output"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInvariant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInvariant"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getMaxOrder"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
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
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getSpotPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getStrategyData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getStrategyData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("strikePriceWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("strategyData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("portfolio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("portfolio"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                            inputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validatePool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validatePool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint64"),
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
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
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
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AfterCreate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "volatilityBasisPoints",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("durationSeconds"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isPerpetual"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Genesis"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Genesis"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("portfolio"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_InvalidBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upper"),
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
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_RootOutsideBounds",
                            ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Infinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategyLib_ConfigExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_ConfigExists",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidDuration",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidDuration",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidStrategyArgs",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidStrategyArgs",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidStrikePrice",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidStrikePrice",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_InvalidVolatility",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_InvalidVolatility",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned(
                        "NormalStrategyLib_NonExpiringPool",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategyLib_NonExpiringPool",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NormalStrategy_NotPortfolio"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NormalStrategy_NotPortfolio",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_OutputExceedsReserves"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_OutputExceedsReserves",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ProtocolFeeTooHigh"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ProtocolFeeTooHigh",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroXAdjustment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ZeroXAdjustment",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SwapLib_ZeroYAdjustment"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "SwapLib_ZeroYAdjustment",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static NORMALSTRATEGY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA04b\0\0\xD6W`\x1Fb\x005;8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\0\xDBW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12b\0\0\xD6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03b\0\0\xD6W`\x80R`@Q\x90\x7F;5\xCD\xE5\x85\"y\xBF\xD1\xCE\x80S\x8F\x94\xC2\xE4*S\x0F\x11\x1FVB\x9B\x91,\x85\xCF\xA6P\xD4\xD3`\0\x80\xA2a4I\x90\x81b\0\0\xF2\x829`\x80Q\x81\x81\x81`\xFD\x01R\x81\x81a\x01\xDF\x01R\x81\x81a\x04\xC0\x01R\x81\x81a\x08\xB9\x01R\x81\x81a\nV\x01R\x81\x81a\x0B\xAE\x01R\x81\x81a\x0CN\x01R\x81\x81a\x0F:\x01R\x81\x81a\x12(\x01Ra\x15/\x01R\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x16\xED\xE0\x16\x14a\0\xE7W\x80c\x19\x05x\x07\x14a\0\xE2W\x80c4\xDB\xC7;\x14a\0\xDDW\x80c9CMZ\x14a\0\xD8W\x80cE-/\x18\x14a\0\xD3W\x80cK\xF3F\xBF\x14a\0\xCEW\x80c\x80\xAF\x9Dv\x14a\0\xC9W\x80c\xA4G\x89\x19\x14a\0\xC4W\x80c\xE0hx\x7F\x14a\0\xBFW\x80c\xE31\xBA4\x14a\0\xBAW\x80c\xE6\x04{\x19\x14a\0\xB5W\x80c\xECshT\x14a\0\xB0Wc\xF0{\x87\x9E\x14a\0\xABW`\0\x80\xFD[a\x0C\x0BV[a\x0B\x9DV[a\x0B<V[a\n\rV[a\t\x9CV[a\x08yV[a\x07\xCFV[a\x07TV[a\x05\xA3V[a\x04|V[a\x04\x07V[a\x01\x87V[4a\x01,W`\x006`\x03\x19\x01\x12a\x01,W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\0\x80\xFD[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01,WV[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01,WV[\x80\x15\x15\x03a\x01,WV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01,WV[`\xC45\x90a\x01\x85\x82a\x01gV[V[4a\x01,W`\x806`\x03\x19\x01\x12a\x01,Wa\x01\xA0a\x011V[`$5a\x01\xAC\x81a\x01]V[`d5\x91a\x01\xB9\x83a\x01gV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93a\x01\0\x91\x82\x81`$\x81\x89Z\xFA\x92\x83\x15a\x03\x8CW`\0\x93a\x03\xD8W[PP`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x84\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x91`\x80\x83`$\x81\x8AZ\xFA\x92\x83\x15a\x03\x8CW`\0\x93a\x03\xA5W[P`\x04\x90\x86\x15a\x03\x91W\x82a\x02v`\xFFa\x02p\x83\x88\x01Q`\xFF\x16\x90V[\x16a\x14JV[\x98a\x02\xDCa\x02\xA4a\x02\x9A\x8A`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x9B`D5\x02a3TV[\x98a\x02\xBFa\x02\xB0a\x06\xF4V[`\x01`\x01`\x80\x1B\x03\x90\x9B\x16\x8BRV[`\x01\x8A\x85\x01R`\0`@\x8B\x01R`\x01`\x01`@\x1B\x03\x16``\x8A\x01RV[\x88\x15\x15`\x80\x89\x01R`@QcXq\x0FE`\xE1\x1B\x81R\x93\x84\x91\x82\x90Z\xFA\x80\x15a\x03\x8CWa\x03J\x98a\x039\x97`\xFF\x97a\x03&\x95`\0\x94a\x03]W[Pa\x03 B\x93a\x10\xBEV[\x90a$\xEDV[\x94\x15a\x03NWP``\x01Q`\xFF\x16a\x02pV[`@Q\x91\x04\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x01Q`\xFF\x16a\x02pV[a\x02pV[a\x03~\x91\x94P\x87=\x89\x11a\x03\x85W[a\x03v\x81\x83a\x06\xD3V[\x81\x01\x90a\x13\xE2V[\x928a\x03\x15V[P=a\x03lV[a\x11\xF2V[\x82a\x02v`\xFFa\x03X``\x88\x01Q`\xFF\x16\x90V[`\x04\x91\x93Pa\x03\xCA\x90`\x80=\x81\x11a\x03\xD1W[a\x03\xC2\x81\x83a\x06\xD3V[\x81\x01\x90a\x13tV[\x92\x90a\x02SV[P=a\x03\xB8V[a\x03\xF8\x92\x93P\x80=\x10a\x04\0W[a\x03\xF0\x81\x83a\x06\xD3V[\x81\x01\x90a\x11CV[\x908\x80a\x02\x1BV[P=a\x03\xE6V[4a\x01,W` 6`\x03\x19\x01\x12a\x01,W`\x01`\x01`@\x1B\x03a\x04(a\x011V[\x16`\0R`\0` R`\xA0`@`\0 T`\xFF`@Q\x91`\x01`\x01`\x80\x1B\x03\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x80\x82`\x80\x1C\x16` \x85\x01R\x80\x82\x86\x1C\x16`@\x85\x01R\x81`\xC0\x1C\x16``\x84\x01R`\xE0\x1C\x16\x15\x15`\x80\x82\x01R\xF3[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\x04\x95a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x91\x90\x91\x16`\x04\x82\x01\x81\x90Ra\x01\0\x90\x81\x83`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x8CWa\x03J\x93a\x05\x1E\x93`\0\x93a\x05.W[PP`\0R`\0` Ra\x05\x18`@`\0 a\x10\xBEV[\x90a\x1F\xA0V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x05E\x92\x93P\x80=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x908\x80a\x05\x01V[\x90\x93\x92\x91\x93``\x82R\x80Q\x90\x81``\x84\x01R`\0[\x82\x81\x10a\x05\x8DWPP\x90`\x80\x82`\0\x82`@\x95\x85\x01\x01R`\x1F\x80\x19\x91\x01\x16\x82\x01\x01\x94` \x82\x01R\x01RV[\x80` \x80\x92\x84\x01\x01Q`\x80\x82\x87\x01\x01R\x01a\x05bV[4a\x01,W`\xA06`\x03\x19\x01\x12a\x01,Wa\x03J`d5a\x05\xC3\x81a\x01]V[a\x06sa\x05\xD1`\x045a3TV[a\x05\xDC`$5a3lV[\x92a\x05\xE8`D5a3lV[`\x01`\x01`\x80\x1B\x03`@Q\x93a\x05\xFD\x85a\x06\x98V[\x16\x94\x85\x84R` \x84\x01\x90c\xFF\xFF\xFF\xFF\x92\x83\x80\x92\x16\x83R\x81`@\x87\x01\x91\x16\x81R\x81``\x87\x01\x93`\0\x85R`\x80\x88\x01\x96\x15\x15\x87R`@Q\x99` \x8B\x01RQ\x16`@\x89\x01RQ\x16``\x87\x01RQ\x16`\x80\x85\x01RQ\x15\x15`\xA0\x84\x01R`\xA0\x83Ra\x06b\x83a\x06\xB8V[a\x06n`\x845\x91a\x1CsV[a\x19\xDCV[`@\x93\x91\x93Q\x93\x84\x93\x84a\x05MV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[a\x06\x82V[`\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[`@Q\x90a\x01\x85\x82a\x06\x98V[`@Q\x90a\x01\x85\x82a\x06\xB8V[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x06\xB3W`@Q\x91a\x077`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x06\xD3V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01,W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[4a\x01,W`@6`\x03\x19\x01\x12a\x01,W`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01,W6`#\x82\x01\x12\x15a\x01,Wa\x07\xAC\x90a\x06na\x07\xA7a\x07\xA2`\x045\x936\x90`$\x81`\x04\x015\x91\x01a\x07\x0EV[a\x1F\nV[a\x1CsV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x01,WV[4a\x01,W6`\x03\x19\x01`\xE0\x81\x12a\x01,W`\xA0\x13a\x01,Wa\x03Ja\x08Z`@Qa\x07\xFA\x81a\x06\x98V[`\x045a\x08\x06\x81a\x07\xBEV[\x81R`$5a\x08\x14\x81a\x07\xBEV[` \x82\x01R`D5a\x08%\x81a\x01]V[`@\x82\x01Ra\x082a\x01GV[``\x82\x01R`\x845a\x08C\x81a\x01]V[`\x80\x82\x01Ra\x08Pa\x01xV[\x90`\xA45\x90a\x15\0V[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x01,W`\x806`\x03\x19\x01\x12a\x01,Wa\x08\x92a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01Ra\x01\0\x91\x82\x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03\x8CWa\tV\x93`\0\x93a\tuW[PPa\tQa\x05\x18\x91a\t\x1Aa\t\r`D5a3TV[`\x01`\x01`\x80\x1B\x03\x16\x85RV[a\t8a\t(`d5a3TV[`\x01`\x01`\x80\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x10\xBEV[a\tb\x81`$5a\x13KV[`@\x80Q\x91\x15\x15\x82R` \x82\x01\x92\x90\x92R\xF3[a\x05\x18\x92\x93Pa\tQ\x91\x81a\t\x95\x92\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x92\x91a\x08\xF6V[4a\x01,W`@6`\x03\x19\x01\x12a\x01,Wa\t\xB5a\x011V[`$5`\x01`\x01`@\x1B\x03\x80\x82\x11a\x01,W6`#\x83\x01\x12\x15a\x01,W\x81`\x04\x015\x90\x81\x11a\x01,W6`$\x82\x84\x01\x01\x11a\x01,Wa\x03J\x92`$a\t\xFB\x93\x01\x90a\x0F7V[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\n&a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R\x90a\x01\0\x90\x81\x83`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03\x8CWa\x03J\x93a\x05\x1E\x93`\0\x93a\x0B\x11W[PPa\n\xB4a\tQa\x0B\x0B\x92`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91a\n\xC9a\n\xC1\x84a\x1CsV[\x93B\x90a\x1E\xA7V[`\x80\x84\x01Ra\x0B\x05a\n\xEA`@a\n\xF6a\n\xEA\x85Q`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x93\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a1FV[\x90a\x18\xFFV[a\x0B\x0B\x92\x93Pa\x0B4a\n\xB4\x92\x82a\tQ\x93\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x93\x92Pa\n\x8FV[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\x0BUa\x011V[P` `@Q`\x01\x81R\xF3[``\x90`\x03\x19\x01\x12a\x01,W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x01,W\x90`$5a\x0B\x8D\x81a\x01]V[\x90`D5a\x0B\x9A\x81a\x01gV[\x90V[4a\x01,Wa\x0B\xAB6a\x0BaV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0B\xF9Wa\x0B\xE5\x91a\x11\xFEV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`@Qc:#%k`\xE2\x1B\x81R`\x04\x90\xFD[4a\x01,Wa\x0C\x196a\x0BaV[Pa\x0C\"a\x14xV[P`@\x80Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01Ra\x01\0\x93`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x81`$\x81\x86Z\xFA\x95\x86\x15a\x03\x8CW`\0\x96a\x0F\x18W[PP\x82Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x82\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x92`\x80\x90\x84\x90\x81\x80`$\x81\x01\x03\x91Z\xFA\x80\x15a\x03\x8CWa\x03J\x96\x85\x94`\0\x92a\x0E\xF8W[Pa\x0C\xF9a\x0C\xEFa\x07\xA7a\tQ\x87`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[`@\x01Q\x90`\0\x90V[\x95\x90a\r\x03a\x06\xF4V[\x98`\0\x8ARa\r+\x86\x8B\x01\x97`\0\x89R`\0\x85\x8D\x01R``\x8C\x01\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x15\x80\x15`\x80\x8B\x01Ra\x0EyWPa\r\xF5``a\x0E\x01\x94a\x0E\x13a\x0E\x06a\x0E\x01\x87a\r\xFBa\r\xF5a\r\xEBa\x0E(\x9Fa\n\xEAa\x0E\x1B\x9F\x9Da\r\xE4\x8F\x93a\r\xFB\x9Fa\n\xEAa\r\xD0a\r\xBC\x97a\r\xC1a\r\xBC\x88a\r\xB6a\n\xEAa\r\xA8a\r\xA3a\r\xDE\x9A`\x01`\x01`\x80\x1B\x03\x9E\x01\x9E\x8FQ`\x01`\x01`\x80\x1B\x03\x16\x90V[a1\nV[\x92Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x14/V[a\x14\nV[\x9C\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a1%V[\x91\x16a\x14/V[\x9A\x87\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x90a\x14^V[a3TV[`\x01`\x01`\x80\x1B\x03\x16\x8CRV[\x01Q`\xFF\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90RV[Q\x91\x82\x91\x82\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[a\x0E\xF3\x96P\x84a\x0E\x01\x94a\x0E\x13a\x0E\x06a\x0E\x01a\x0E\xE1`\x01`\x01`\x80\x1B\x03a\x0E\xDBa\x0E\xCDa\r\xBC\x8Ca\x0E\x1B\x9Fa\n\xEAa\r\xB6\x91a\r\xFB\x9Fa\r\xF5\x9Fa\r\xDEa\n\xEAa\n\xF6\x93\x88\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16a\x14\nV[\x97a\r\xFBa\r\xF5``\x87\x01Q`\xFF\x16\x90V[a\x0E(V[a\x0F\x11\x91\x92P`\x80=\x81\x11a\x03\xD1Wa\x03\xC2\x81\x83a\x06\xD3V[\x908a\x0C\xC8V[a\x0F/\x92\x96P\x80=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x938\x80a\x0C\x86V[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x913\x83\x90\x03a\x0B\xF9Wa\x07\xA2a\x0F}\x91a\x0F\xEA\x936\x91a\x07\x0EV[a\x0F\x9A\x84`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x81Q`\x01`\x01`\x80\x1B\x03\x16` \x83\x01Qc\xFF\xFF\xFF\xFF\x16\x91`\x01`\x01`\x80\x1B\x03a\x0F\xD7`\x80a\x0F\xCF`@\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x96\x01Q\x15\x15\x90V[\x94c\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x92\x16\x90a\x1C\xFDV[\x7F\x8Di\xFAt\x9A\xF4\x15\xF9<\x96\x86>\xBF6\x92\x93&#\xC0\x10\xEB\xA8\x11\x8B\x02\xF0DXZ\xABe\x13`\x01`\x01`@\x1B\x03a\x103a\tQ\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x93a\x10\xB6a\x10H\x86Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95a\x10Z` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x10y`\x80a\x10q`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92\x01Q\x15\x15\x90V[\x91`@Q\x95\x86\x95\x16\x98\x85\x92\x91``\x92\x95\x94\x91\x95`\x01`\x01`\x80\x1B\x03`\x80\x86\x01\x97\x16\x85Rc\xFF\xFF\xFF\xFF\x80\x92\x16` \x86\x01R\x16`@\x84\x01R\x15\x15\x91\x01RV[\x03\x90\xA3`\0\x90V[\x90`@Qa\x10\xCB\x81a\x06\x98V[`\x80`\xFF\x82\x94T`\x01`\x01`\x80\x1B\x03\x81\x16\x84Rc\xFF\xFF\xFF\xFF\x80\x82\x85\x1C\x16` \x86\x01R\x80\x82`\xA0\x1C\x16`@\x86\x01R\x81`\xC0\x1C\x16``\x85\x01R`\xE0\x1C\x16\x15\x15\x91\x01RV[Q\x90a\x01\x85\x82a\x07\xBEV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01,WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\x01,WV[Q\x90a\x01\x85\x82a\x01gV[\x80\x91a\x01\0\x92\x83\x91\x03\x12a\x01,W`@Q\x91\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3Wa\x11\xEA\x91`\xE0\x91`@Ra\x11|\x81a\x11\rV[\x84Ra\x11\x8A` \x82\x01a\x11\rV[` \x85\x01Ra\x11\x9B`@\x82\x01a\x11\rV[`@\x85\x01Ra\x11\xAC``\x82\x01a\x11\x18V[``\x85\x01Ra\x11\xBD`\x80\x82\x01a\x11)V[`\x80\x85\x01Ra\x11\xCE`\xA0\x82\x01a\x11)V[`\xA0\x85\x01Ra\x11\xDF`\xC0\x82\x01a\x118V[`\xC0\x85\x01R\x01a\x118V[`\xE0\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01Ra\x01\0\x93\x92\x91\x90\x84\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x94\x85\x15a\x03\x8CW`\0\x95a\x12\xAAW[PP\x90a\x12\x94a\x12\x8Aa\tQa\x12\x9A\x94`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91B\x90\x83\x87a!NV[\x93a(hV[a\x12\xA4W`\x01\x91\x90V[`\0\x91\x90V[a\x12\x9A\x93\x92\x95Pa\tQa\x12\xD1a\x12\x94\x93\x83a\x12\x8A\x94\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x96\x93\x94PPa\x12aV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x13\nWV[a\x12\xDBV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x13\nWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x13\nWV[a\x13W\x90`\x01\x92a\x132V[\x12a\x13aW`\x01\x90V[`\0\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x01,WV[\x90\x81`\x80\x91\x03\x12a\x01,W`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3Wa\x13\xDA\x91``\x91`@R\x80Qa\x13\xAE\x81a\x01gV[\x84Ra\x13\xBC` \x82\x01a\x13fV[` \x85\x01R`@\x81\x01Qa\x13\xCF\x81a\x01gV[`@\x85\x01R\x01a\x13fV[``\x82\x01R\x90V[\x90\x81` \x91\x03\x12a\x01,WQ\x90V[\x90`\x12\x03`\x12\x81\x11a\x13\nWa\x14\x06\x90a\x14<V[\x02\x90V[`\0\x19\x81\x01\x91\x90\x82\x11a\x13\nWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x13\nWV[\x91\x90\x82\x03\x91\x82\x11a\x13\nWV[`M\x81\x11a\x13\nW`\n\n\x90V[`\x12\x03`\x12\x81\x11a\x13\nWa\x0B\x9A\x90a\x14<V[\x90`\x12\x03`\x12\x81\x11a\x13\nWa\x14s\x90a\x14<V[\x90\x04\x90V[`@Q\x90a\x14\x85\x82a\x06\x98V[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[``\x81\x01\x80Q`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R\x93\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x90\x91a\x01\0\x80\x87`$\x81\x88Z\xFA\x91\x82\x15a\x03\x8CWa\x15\xCB\x97`\0\x93a\x17\xCDW[Pa\x15\x9Fa\x15\x94a\x15\x88\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[` \x1Cb\xFF\xFF\xFF\x16\x90V[`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x80\x98\x90\x91\x90\x89\x90\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x8AZ\xFA\x90\x81\x15a\x03\x8CWa\x16\xDC` \x94a\x0E\x1B`\xA0`\x04\x9Da\x16\xD5a\x0E\x06a\x16\xCC\x8B\x8Ea\t8\x9Ba\x16\xE9\x9D`\0\x91a\x17\xB0W[Pa\x16\ta\x14\xA3V[\x80\x9Aa\x16\x17\x89\x85\x01Q\x15\x15\x90V[\x15a\x17TW\x82a\x16?a\x16\x85\x93`@\x93a\x167\x89a\x16\x95\x98\x01Q`\xFF\x16\x90V[`\xFF\x16\x91\x01RV[a\x16T\x8Da\x01 a\x167``\x85\x01Q`\xFF\x16\x90V[\x80Qa\x16w\x90\x8E\x90`\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x8B\x01RV[a\x16\xA9a\n\xEA\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x8A\x01R\x01\x96a\x16\xC3a\n\xEA\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x82\x01Ra\x17\xECV[\x92\x83\x01Qa3TV[\x01Qa3TV[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x94`@Q\x97\x88\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x93\x84\x15a\x03\x8CWa\x17 \x96`\0\x95a\x170W[Pa\x17\x1A\x90a\x10\xBEV[\x90a\"\x1BV[\x90\x91Pa\x17-\x81\x83a\x13KV[\x92V[a\x17\x1A\x91\x95Pa\x17M\x90` =\x81\x11a\x03\x85Wa\x03v\x81\x83a\x06\xD3V[\x94\x90a\x17\x10V[\x91a\x17na\x16\x85\x92a\x17\xAB\x94a\x167``\x85\x01Q`\xFF\x16\x90V[a\x17\x82\x8Ca\x01 a\x167\x88\x85\x01Q`\xFF\x16\x90V[`@\x81\x01Qa\x17\x9E\x90\x8D\x90`\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16a\x16iV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x16\x95V[a\x17\xC7\x91P\x87=\x89\x11a\x03\xD1Wa\x03\xC2\x81\x83a\x06\xD3V[8a\x16\0V[a\x17\xE5\x91\x93P\x82=\x84\x11a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x918a\x15pV[a\x17\xF4a\x14\xA3V[P`@\x81\x01\x80Q\x90a\x18\x10a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90a\x13\xF1V[\x90R``\x82\x01a\x18&\x81Q`\xFF\x84Q\x16\x90a\x13\xF1V[\x90Ra\x18=`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90a\x13\xF1V[\x90R`\xA0\x81\x01a\x18W\x81Q`\xFFa\x01 \x85\x01Q\x16\x90a\x13\xF1V[\x90R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x13\nW`\0\x19\x83\x05\x03a\x13\nWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x13\nW\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x13\nWV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x80\x83\x10\x15a\x19\x9AW\x82\x15a\x19\x8CWa\x19#`\x80\x83\x01Qa0\xE3V[\x90a\x19-\x83a\x19\xA4V[\x93\x81\x03\x90\x80\x82\x11a\x13\nWa\x19}`@\x93g\x1B\xC1mgN\xC8\0\0a\x19va\x19ba\x0B\x9A\x99a\x19]a\x19\x83\x98a*\x03V[a\x18\xB3V[\x92a\x19q``\x8A\x01Q\x80a\x18\xD6V[a\x18\xD6V[\x04\x90a\x132V[\x05a-\x86V[\x91\x01Q\x90a1%V[PPP`\x01`\x01`\x80\x1B\x03\x90V[P`@\x91P\x01Q\x90V[a\x19\xB9a\x19\xB4`\x80\x83\x01Qa0\xE3V[a2.V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x13\nW``a\x0B\x9A\x92\x01Qa1%V[`@\x81\x01Q`\0\x93\x92a\x19\xEF\x91\x90a1FV[\x80a\x1A\x02W[Pa\x0B\x9A\x90\x83\x81Ra\x1B\xF8V[a\x1A\x11a\x1A;\x91\x94\x92\x94a/BV[a\x1A[a\x1A!`\x80\x87\x01Qa0\xE3V[a\x1AU``\x88\x01Qg\x1B\xC1mgN\xC8\0\0\x95\x81\x87\x92a\x18\xD6V[\x04\x91a\x1AOa\x1AI\x8Aa\x19\xA4V[\x95a\x18\\V[\x92a\x18\xB3V[\x90a\x1B\xDCV[\x90\x80\x15a\x1A\xC9W`\x01`\xFF\x1B\x82\x14`\0\x19\x82\x14\x16a\x13\nWa\x1A\x96g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1A\x90a\x1A\x9B\x93a\x1A\xA0\x95\x05a\x18\\V[\x05a(\x94V[a(\xCDV[a\x18\\V[\x05g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x03\x92\x90\x91\x12\x80\x15\x82\x84\x13\x16\x91\x83\x12\x16\x17a\x13\nW\x91a\x0B\x9Aa\x19\xF5V[a\x18\xE9V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x13\nWV[`@\x81\x01\x80Q\x82Q\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x10\x15a\x1CiW\x15a\x1CbWPa\x1C\"\x83a\x19\xA4V[\x90\x83Q\x81\x03\x90\x81\x11a\x13\nWa\x0B\x9A\x93`\xA0a\x1CLa\x1CZ\x94a\x1CGa\x1CU\x95a*\x03V[a\x132V[\x91\x01Q\x90a\x1B\xDCV[a(\xA5V[\x90Q\x90a1%V[\x92PPP\x90V[PPPPP`\0\x90V[`\xA0`@Qa\x1C\x81\x81a\x06\xB8V[`\0\x91\x81\x83\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x01R`\x01`\x01`\x80\x1B\x03\x82Q\x16\x91c\xFF\xFF\xFF\xFF`@a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x83` \x86\x01Q\x16\x02\x04\x92\x01Q\x16\x90`@Q\x93a\x1C\xDD\x85a\x06\xB8V[\x83\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[\x92\x93\x91\x90\x93c\xFF\xFF\xFF\xFF\x92\x83a\x1D\x1B\x86Tc\xFF\xFF\xFF\xFF\x90`\xC0\x1C\x16\x90V[\x16a\x1E\x95W\x80\x15a\x1E4W\x84T`\xFF`\xE0\x1B\x19\x16\x90\x15\x15`\xE0\x1B`\xFF`\xE0\x1B\x16\x17\x84UP\x82Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16c\x01\xE1\x85Y`\xA0\x1B\x17\x83U[aa\xA8\x80\x82\x10\x90\x82\x14\x17`\x01\x82\x11`\x01\x83\x14\x17\x16\x15a\x1E\"Wa\x1D|a\x1D\x9C\x91a3lV[\x83Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x80\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\x80\x1B\x16\x17\x83UV[`\x01`\x01`\x80\x1B\x03\x80\x84\x10\x90\x84\x14\x17`\x01\x84\x11`\x01\x85\x14\x17\x16\x15a\x1E\x10Wa\x1D\xEEa\x1D\xC9a\x01\x85\x94a3TV[\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x83UV[\x81Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16B\x91\x90\x91\x16`\xC0\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x17\x90UV[`@Qc\xB2B\xE3A`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9]8\x19`\xE0\x1B\x81R`\x04\x90\xFD[Pb\x01Q\x80\x81\x81\x14\x90\x82\x11\x17c\x05\xA4\x90\x0B\x82\x81\x14\x90\x83\x10\x17\x16\x15a\x1E\x83Wa\x1E^a\x1E~\x91a3lV[\x84Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x84UV[a\x1DWV[`@Qc\xB5\x97\x03\x0F`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x1E\x13\x89\xA1`\xE2\x1B\x81R`\x04\x90\xFD[`\x80\x81\x01Qa\x1E\xCCWa\x1E\xBEc\xFF\xFF\xFF\xFF\x91a\x1E\xD6V[\x16\x80\x82\x10\x90\x82\x03\x02\x81\x03\x03\x90V[PPc\x01\xE1\x85Y\x90V[`\x80\x81\x01Qa\x1E\xF8Wc\xFF\xFF\xFF\xFF\x90\x81`@\x81``\x84\x01Q\x16\x92\x01Q\x16\x01\x16\x90V[`@Qc\xB0\x19\x84\x97`\xE0\x1B\x81R`\x04\x90\xFD[a\x1F\x12a\x14xV[P`\xA0\x81Q\x03a\x1F\x8EW`\xA0\x81\x80Q\x81\x01\x03\x12a\x01,W`\xA0`@Q\x91a\x1F8\x83a\x06\x98V[` \x81\x01Qa\x1FF\x81a\x07\xBEV[\x83Ra\x1FT`@\x82\x01a\x11\x18V[` \x84\x01Ra\x1Fe``\x82\x01a\x11\x18V[`@\x84\x01Ra\x1Fv`\x80\x82\x01a\x11\x18V[``\x84\x01R\x01Qa\x1F\x86\x81a\x01]V[`\x80\x82\x01R\x90V[`@Qc\x01YW\xD3`\xE2\x1B\x81R`\x04\x90\xFD[a\x0B\x9A\x91a\x1F\xB8a\n\xEA\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91`\x01`\x01`\x80\x1B\x03a \x02a\x1F\xE5a\x1F\xDEa\n\xEA`@\x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x96a1FV[\x94a\x1F\xFDa\n\xEA` \x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a1FV[\x92a ga \x17\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a ac\xFF\xFF\xFF\xFF``a Ta Ba 9` \x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[g\r\xE0\xB6\xB3\xA7d\0\0a'\x10\x91\x02\x04\x90V[\x97\x01Q\x16c\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x1E\xA7V[\x93a pa\x07\x01V[\x95\x86R` \x86\x01R\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\0`\xA0\x82\x01R[a\x0B\x9A\x90a \xF4a \xA0\x82a\x19\xA4V[\x91a \xAF\x81`@\x01Q\x90`\0\x90V[\x82Q\x90\x92\x91\x90g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a!*WP`\x01\x92[` \x83\x01Q\x91\x82\x10a \xF9WPPPa\x1CGa \xEEg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92[a*\x03V[\x91a*\x03V[a\x1B\xDCV[\x81\x11a!\x0FWPPa\x1CGa \xEE`\x01\x92a*\x03V[a!$a \xEE\x91`@a\x1CG\x94\x01Q\x90a1\xFEV[\x92a*\x03V[\x80a!?WPg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92a \xC9V[a!H\x90a\x14\x19V[\x92a \xC9V[\x90`\x01`\x01`\x80\x1B\x03a\x0B\x9A\x94\x92a!pa\n\xEA\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a!\x87a\n\xEA` \x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95\x15a!\xEBWa!\xB1a!\xAAa\n\xEA`@a!\xB7\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a1FV[\x95a1\xFEV[\x92[a ga!\xCD\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a!\xE5a Ba 9` \x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x94a\x1E\xA7V[a\"\x0Fa\"\x08a\n\xEA`@a\"\x15\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a1\xFEV[\x95a1FV[\x92a!\xB9V[a\x0B\x05a\n\xEAa#3\x93a#$`@a\"\xA9a\"D\x9A\x99\x98\x99a\"K\x8A`\x80\x8D\x01\x9D\x8EQ\x15\x15\x90V[\x87\x8Ba!NV[\x9Ca\"]\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[` \x8A\x01Q`\xC0\x8B\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x03a#XW`\xA0\x8A\x01Qa\xFF\xFF\x16\x9C[`\x01`\x01`\x80\x1B\x03\x9Da\xFF\xFF\x16\x92\x8E\x16\x91\x8E\x16\x90a#gV[\x98\x92P\x94\x90Pa\"\xE1a\"\xC3\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9Aa\"\xDBa Ba 9` \x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92a\x1E\xA7V[\x90a\"\xEAa\x07\x01V[\x9B`\0\x8DR` \x8D\x01\x9B`\0\x8DR\x16\x84\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\0`\xA0\x8B\x01R\x01\x91a\x0B\x05a\n\xEA\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87RQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81Ra#Ha#A\x83a \x90V[\x93Q\x15\x15\x90V[\x15a#RWPQ\x92V[\x90PQ\x92V[`\x80\x8A\x01Qa\xFF\xFF\x16\x9Ca\"\x90V[\x91\x94\x92\x94`\0a#\x82`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16a1\xB0V[\x95\x80\x15\x80\x15a$\xB5W[PP\x80\x95\x80\x97`\x80\x86\x01\x92a#\xD8a#\xA4\x85Q\x15\x15\x90V[\x93\x84\x15a$\xAEW\x87\x94[\x15a$\xA4Wa#\xD3\x87\x95[a#\xCDa\n\xEA\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a$\xD1V[a\x14/V[\x90\x81\x81\x11a$\x92Wa#\xED` \x91\x85\x93a\x14/V[\x97\x01\x91a$\x01\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11a$\x80Wa$)\x91a\r\xB6a\n\xEAa$\"\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91Q\x15\x15\x90V[\x90\x81\x15a$wW\x84\x85\x92[\x15a$oWP\x92[\x14a$]W\x81\x14a$KW\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92a$<V[\x80\x94\x85\x92a$4V[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[a#\xD3\x88\x95a#\xB9V[\x86\x94a#\xAEV[\x90\x91Pa\x1A\xC9W\x85\x04\x94\x85\x81\x03\x90\x81\x11a\x13\nW\x948\x80a#\x8CV[\x91\x90\x82\x01\x80\x92\x11a\x13\nWV[`\0\x19\x81\x14a\x13\nW`\x01\x01\x90V[\x93\x91a%\x1Ea%\x06`\x80\x93\x97a%)\x95\x87\x85\x8B\x8Ba\"\x1BV[P\x95\x90\x97a%\x13\x81a\x1CsV[\x96`\xA0\x88\x01Ra\x1E\xA7V[\x82\x85\x01R\x01Q\x15\x15\x90V[\x92\x83\x15a&[W\x81Ra%;\x81a\x1B\xF8V[\x80` \x83\x01R[\x80\x15a&*W\x91a%\xF9a%\xE2a%\xDDa%\xC1`\x01`\x01`\x80\x1B\x03\x95\x87a%\xCFa%xa%ra\x0B\x9A\x9C\x9Ba1hV[\x92a1\xD6V[\x92`@Q\x94\x85\x91` \x83\x01\x91\x90\x91`\xA0\x80`\xC0\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x85R\x84a\x06\xD3V[\x88\x15a&\"W`\x02\x92a&qV[a$\xDEV[a\r\xDEa\n\xEA`@\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x92\x15a&\x13W` \x01Q`\x01`\x01`\x80\x1B\x03\x16[\x16a\x14/V[Q`\x01`\x01`\x80\x1B\x03\x16a&\rV[`\x01\x92a&qV[PP`\x01`\x01`\x80\x1B\x03\x91`\0\x14a&MW` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[` \x82\x01Ra&i\x81a'\xA0V[\x80\x82Ra%BV[\x91\x90\x93\x92\x93`\0\x94`\0\x92\x80\x83\x11a'\x7FWa&\x8E\x83\x86\x84a3\x98V[\x90a&\x9A\x81\x87\x85a3\x98V[\x85a&\xA5\x82\x85a\x18\xB3V[\x13a'^WP\x95\x94\x93a&\xB8\x84\x88a\x14/V[\x94\x81`\x01\x96\x87\x80[a&\xD2W[PPPPPPPPP\x90PV[\x15a'9W[P\x86\x97\x98\x99P\x80\x92a&\xF3a&\xED\x8C\x89a$\xD1V[`\x01\x1C\x90V[\x9A\x8B\x90a'\x01\x8D\x86\x8Aa3\x98V[\x90\x84a'\r\x89\x84a\x18\xB3V[\x13a'+WPP\x93[\x88a'!\x89\x87a\x14/V[\x92\x01\x94\x9A\x99a&\xC0V[\x95\x96P\x97PP\x8A\x96\x94a'\x16V[\x87\x10\x80a'SW[\x15a'LW\x89a&\xD8V[\x80\x80a&\xC5V[Pa\x01\0\x83\x10a'AV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@\x81\x01\x80Q\x91` \x81\x01\x92\x83Q\x90\x81\x10\x15a(\x0FW\x15a(\0W`\xA0a'\xE1a'\xEA\x94a \xF4a \xE9a\x1CU\x96a'\xD7\x87a\x19\xA4V[\x93Q\x90Q\x90a1\xFEV[\x91\x01Q\x90a\x132V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x13\nW\x90V[PPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[PPPP`\0\x90V[\x90\x81`\xC0\x91\x03\x12a\x01,W`\xA0`@Q\x91a(2\x83a\x06\xB8V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R\x01Q`\xA0\x82\x01R\x90V[`\x80\x82\x01Qa(\x8DW``\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91a(\x87\x90a\x1E\xD6V[\x16\x11\x15\x90V[PP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\x13\nW`\0\x03\x90V[a(\xC9a\x1A\x9Ba\x1A\x96g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1A\x90g\x1B\xC1mgN\xC8\0\0\x95a\x18\\V[\x05\x90V[\x80\x15a)\xF6WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a)\xF0WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a)\xE3Wa)\xD1a)\0\x82a+\xAEV[a)\x94a)\xCCa)\x1Fa)\x1Aa)\x15\x85a1\x85V[a\x1A\xCEV[a-EV[\x92a \xF4a)\xC7a)\xC2a)\xBBa)\xB5a)\xB0a)\xAAa)\xA5a)\x9Fa)\x9A\x8Da)\x94a)\x8Fa)\x89a)\x84a)~a)ya)sa)na)ha)c\x8Aa+\xDBV[a\x1A\xE6V[\x89a,}V[a\x1B\0V[\x87a,}V[a\x1B\x18V[\x85a,}V[a\x1B2V[\x83a,}V[a\x1BJV[\x90a,}V[a\x1BdV[\x8Ca,}V[a\x1B|V[\x8Aa,}V[a\x1B\x94V[\x88a,}V[\x93\x80a,}V[a\x18yV[a\x12\xF1V[a-\x86V[\x90`\0\x13\x15a\x0B\x9AWa\x0B\x9A\x90a\x13\x0FV[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a)\xF0Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a+XW\x81\x15a+yW`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x13\nW`\0\x83\x12\x80\x15a+\x9DW[a+\x8BW\x82\x15a+XWg\x1B\xC1mgN\xC8\0\0\x83\x14a+yW\x82\x12\x91\x82\x15a+jW\x92[a*r\x84a,\xC5V[\x80\x15a+XWa*\xCFa*\x9Ea*\x99a\x19\xB4a*\x94a*\xD4\x95\x99\x97\x96\x99a/BV[a,\x06V[a\x18\x9AV[a\x1CGa*\xB2a*\xAD\x83a,\xF0V[a\x1B\xACV[a*\xC9a)\x15a)~a*\xC4\x86a-\x1BV[a\x1B\xC4V[\x90a-dV[a,.V[\x93`\0\x92[\x81\x84\x10a+\x0BWPPPPa\x0B\x9A\x91a*\xF8\x91`\0\x14a*\xFDWa,\x9EV[a(\x94V[a+\x06\x90a(\x94V[a,\x9EV[\x90\x91a+N\x86a\x1AUa+#\x85a\x1CG\x86\x99\x9Ba(\xCDV[a*\xC9a+>a+9a)\xCCa*\xF8\x87\x80a,}V[a,VV[a+H\x83\x86a,}V[\x90a\x132V[\x95\x01\x92\x91\x90a*\xD9V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a+s\x90a\x13\x0FV[\x92a*iV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a*EV[`\x01`\xFF\x1B\x81\x14a+\xC9W`\0\x81\x12\x15a\x0B\x9AW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x01,Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01,W\x05\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a)\xF0Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a.\xD6We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x15a/\x11WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a/n`\0\x82\x13a/\nV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a/\x8A\x82a2\xECV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wc\x01\xE1\x85Y\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x02\x04\x90\x81\x14`\x01\x16\x15a\x01,W\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01,W\x04\x90V[`2\x81\x02\x90`2\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`d\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x96\x81\x02\x90`\x96\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01,W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a2\xD5W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a2\xC8W[e\x01\0\0\0\0\0\x81\x10\x15a2\xBBW[c\x01\0\0\0\x81\x10\x15a2\xAEW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a2rV[` \x1C\x91`\x10\x1B\x91a2eV[`@\x1C\x91` \x1B\x91a2VV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca2>V[a2\xF7\x81\x15\x15a/\nV[\x80`\x01`\x01`\x80\x1B\x03\x10`\x07\x1B\x81\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\x80\x1B\x81\x10\x15a\x01,W`\x01`\x01`\x80\x1B\x03\x16\x90V[d\x01\0\0\0\0\x81\x10\x15a\x01,Wc\xFF\xFF\xFF\xFF\x16\x90V[cNH{q`\xE0\x1B`\0R`Q`\x04R`$`\0\xFD[\x80`\x02\x14a3\xEEW`\x01\x03a3\x82W\x80` \x80a3\xBA\x93Q\x83\x01\x01\x91\x01a(\x18V[\x90\x81R`\xA0a3\xC8\x82a \x90V[\x91\x01Q`\x01\x81\x01\x90`\0`\x01\x83\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x13\nWa\x0B\x9A\x91a\x132V[P\x80` \x80a4\x02\x93Q\x83\x01\x01\x91\x01a(\x18V[\x90` \x82\x01R`\xA0a3\xC8\x82a \x90V\xFE\xA2dipfsX\"\x12 \x15\xAE\xFC\xB37\x02\xB8X\xED\x0B\x82[\x1F\x81\xE0\x0F#}\x05\xEE\x7FD\xDBK\xF1\x10\xB9\xF9n\x8BT\x19dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static NORMALSTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x16\xED\xE0\x16\x14a\0\xE7W\x80c\x19\x05x\x07\x14a\0\xE2W\x80c4\xDB\xC7;\x14a\0\xDDW\x80c9CMZ\x14a\0\xD8W\x80cE-/\x18\x14a\0\xD3W\x80cK\xF3F\xBF\x14a\0\xCEW\x80c\x80\xAF\x9Dv\x14a\0\xC9W\x80c\xA4G\x89\x19\x14a\0\xC4W\x80c\xE0hx\x7F\x14a\0\xBFW\x80c\xE31\xBA4\x14a\0\xBAW\x80c\xE6\x04{\x19\x14a\0\xB5W\x80c\xECshT\x14a\0\xB0Wc\xF0{\x87\x9E\x14a\0\xABW`\0\x80\xFD[a\x0C\x0BV[a\x0B\x9DV[a\x0B<V[a\n\rV[a\t\x9CV[a\x08yV[a\x07\xCFV[a\x07TV[a\x05\xA3V[a\x04|V[a\x04\x07V[a\x01\x87V[4a\x01,W`\x006`\x03\x19\x01\x12a\x01,W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\0\x80\xFD[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01,WV[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01,WV[\x80\x15\x15\x03a\x01,WV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01,WV[`\xC45\x90a\x01\x85\x82a\x01gV[V[4a\x01,W`\x806`\x03\x19\x01\x12a\x01,Wa\x01\xA0a\x011V[`$5a\x01\xAC\x81a\x01]V[`d5\x91a\x01\xB9\x83a\x01gV[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x93a\x01\0\x91\x82\x81`$\x81\x89Z\xFA\x92\x83\x15a\x03\x8CW`\0\x93a\x03\xD8W[PP`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x84\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x91`\x80\x83`$\x81\x8AZ\xFA\x92\x83\x15a\x03\x8CW`\0\x93a\x03\xA5W[P`\x04\x90\x86\x15a\x03\x91W\x82a\x02v`\xFFa\x02p\x83\x88\x01Q`\xFF\x16\x90V[\x16a\x14JV[\x98a\x02\xDCa\x02\xA4a\x02\x9A\x8A`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x9B`D5\x02a3TV[\x98a\x02\xBFa\x02\xB0a\x06\xF4V[`\x01`\x01`\x80\x1B\x03\x90\x9B\x16\x8BRV[`\x01\x8A\x85\x01R`\0`@\x8B\x01R`\x01`\x01`@\x1B\x03\x16``\x8A\x01RV[\x88\x15\x15`\x80\x89\x01R`@QcXq\x0FE`\xE1\x1B\x81R\x93\x84\x91\x82\x90Z\xFA\x80\x15a\x03\x8CWa\x03J\x98a\x039\x97`\xFF\x97a\x03&\x95`\0\x94a\x03]W[Pa\x03 B\x93a\x10\xBEV[\x90a$\xEDV[\x94\x15a\x03NWP``\x01Q`\xFF\x16a\x02pV[`@Q\x91\x04\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x01Q`\xFF\x16a\x02pV[a\x02pV[a\x03~\x91\x94P\x87=\x89\x11a\x03\x85W[a\x03v\x81\x83a\x06\xD3V[\x81\x01\x90a\x13\xE2V[\x928a\x03\x15V[P=a\x03lV[a\x11\xF2V[\x82a\x02v`\xFFa\x03X``\x88\x01Q`\xFF\x16\x90V[`\x04\x91\x93Pa\x03\xCA\x90`\x80=\x81\x11a\x03\xD1W[a\x03\xC2\x81\x83a\x06\xD3V[\x81\x01\x90a\x13tV[\x92\x90a\x02SV[P=a\x03\xB8V[a\x03\xF8\x92\x93P\x80=\x10a\x04\0W[a\x03\xF0\x81\x83a\x06\xD3V[\x81\x01\x90a\x11CV[\x908\x80a\x02\x1BV[P=a\x03\xE6V[4a\x01,W` 6`\x03\x19\x01\x12a\x01,W`\x01`\x01`@\x1B\x03a\x04(a\x011V[\x16`\0R`\0` R`\xA0`@`\0 T`\xFF`@Q\x91`\x01`\x01`\x80\x1B\x03\x81\x16\x83Rc\xFF\xFF\xFF\xFF\x80\x82`\x80\x1C\x16` \x85\x01R\x80\x82\x86\x1C\x16`@\x85\x01R\x81`\xC0\x1C\x16``\x84\x01R`\xE0\x1C\x16\x15\x15`\x80\x82\x01R\xF3[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\x04\x95a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x91\x90\x91\x16`\x04\x82\x01\x81\x90Ra\x01\0\x90\x81\x83`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x8CWa\x03J\x93a\x05\x1E\x93`\0\x93a\x05.W[PP`\0R`\0` Ra\x05\x18`@`\0 a\x10\xBEV[\x90a\x1F\xA0V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x05E\x92\x93P\x80=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x908\x80a\x05\x01V[\x90\x93\x92\x91\x93``\x82R\x80Q\x90\x81``\x84\x01R`\0[\x82\x81\x10a\x05\x8DWPP\x90`\x80\x82`\0\x82`@\x95\x85\x01\x01R`\x1F\x80\x19\x91\x01\x16\x82\x01\x01\x94` \x82\x01R\x01RV[\x80` \x80\x92\x84\x01\x01Q`\x80\x82\x87\x01\x01R\x01a\x05bV[4a\x01,W`\xA06`\x03\x19\x01\x12a\x01,Wa\x03J`d5a\x05\xC3\x81a\x01]V[a\x06sa\x05\xD1`\x045a3TV[a\x05\xDC`$5a3lV[\x92a\x05\xE8`D5a3lV[`\x01`\x01`\x80\x1B\x03`@Q\x93a\x05\xFD\x85a\x06\x98V[\x16\x94\x85\x84R` \x84\x01\x90c\xFF\xFF\xFF\xFF\x92\x83\x80\x92\x16\x83R\x81`@\x87\x01\x91\x16\x81R\x81``\x87\x01\x93`\0\x85R`\x80\x88\x01\x96\x15\x15\x87R`@Q\x99` \x8B\x01RQ\x16`@\x89\x01RQ\x16``\x87\x01RQ\x16`\x80\x85\x01RQ\x15\x15`\xA0\x84\x01R`\xA0\x83Ra\x06b\x83a\x06\xB8V[a\x06n`\x845\x91a\x1CsV[a\x19\xDCV[`@\x93\x91\x93Q\x93\x84\x93\x84a\x05MV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[a\x06\x82V[`\xC0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@RV[`@Q\x90a\x01\x85\x82a\x06\x98V[`@Q\x90a\x01\x85\x82a\x06\xB8V[\x92\x91\x92`\x01`\x01`@\x1B\x03\x82\x11a\x06\xB3W`@Q\x91a\x077`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x06\xD3V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01,W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[4a\x01,W`@6`\x03\x19\x01\x12a\x01,W`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01,W6`#\x82\x01\x12\x15a\x01,Wa\x07\xAC\x90a\x06na\x07\xA7a\x07\xA2`\x045\x936\x90`$\x81`\x04\x015\x91\x01a\x07\x0EV[a\x1F\nV[a\x1CsV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x01,WV[4a\x01,W6`\x03\x19\x01`\xE0\x81\x12a\x01,W`\xA0\x13a\x01,Wa\x03Ja\x08Z`@Qa\x07\xFA\x81a\x06\x98V[`\x045a\x08\x06\x81a\x07\xBEV[\x81R`$5a\x08\x14\x81a\x07\xBEV[` \x82\x01R`D5a\x08%\x81a\x01]V[`@\x82\x01Ra\x082a\x01GV[``\x82\x01R`\x845a\x08C\x81a\x01]V[`\x80\x82\x01Ra\x08Pa\x01xV[\x90`\xA45\x90a\x15\0V[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x01,W`\x806`\x03\x19\x01\x12a\x01,Wa\x08\x92a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01Ra\x01\0\x91\x82\x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03\x8CWa\tV\x93`\0\x93a\tuW[PPa\tQa\x05\x18\x91a\t\x1Aa\t\r`D5a3TV[`\x01`\x01`\x80\x1B\x03\x16\x85RV[a\t8a\t(`d5a3TV[`\x01`\x01`\x80\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x10\xBEV[a\tb\x81`$5a\x13KV[`@\x80Q\x91\x15\x15\x82R` \x82\x01\x92\x90\x92R\xF3[a\x05\x18\x92\x93Pa\tQ\x91\x81a\t\x95\x92\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x92\x91a\x08\xF6V[4a\x01,W`@6`\x03\x19\x01\x12a\x01,Wa\t\xB5a\x011V[`$5`\x01`\x01`@\x1B\x03\x80\x82\x11a\x01,W6`#\x83\x01\x12\x15a\x01,W\x81`\x04\x015\x90\x81\x11a\x01,W6`$\x82\x84\x01\x01\x11a\x01,Wa\x03J\x92`$a\t\xFB\x93\x01\x90a\x0F7V[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\n&a\x011V[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01R\x90a\x01\0\x90\x81\x83`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03\x8CWa\x03J\x93a\x05\x1E\x93`\0\x93a\x0B\x11W[PPa\n\xB4a\tQa\x0B\x0B\x92`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91a\n\xC9a\n\xC1\x84a\x1CsV[\x93B\x90a\x1E\xA7V[`\x80\x84\x01Ra\x0B\x05a\n\xEA`@a\n\xF6a\n\xEA\x85Q`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x93\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a1FV[\x90a\x18\xFFV[a\x0B\x0B\x92\x93Pa\x0B4a\n\xB4\x92\x82a\tQ\x93\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x93\x92Pa\n\x8FV[4a\x01,W` 6`\x03\x19\x01\x12a\x01,Wa\x0BUa\x011V[P` `@Q`\x01\x81R\xF3[``\x90`\x03\x19\x01\x12a\x01,W`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x01,W\x90`$5a\x0B\x8D\x81a\x01]V[\x90`D5a\x0B\x9A\x81a\x01gV[\x90V[4a\x01,Wa\x0B\xAB6a\x0BaV[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x0B\xF9Wa\x0B\xE5\x91a\x11\xFEV[`@\x80Q\x92\x15\x15\x83R` \x83\x01\x91\x90\x91R\x90\xF3[`@Qc:#%k`\xE2\x1B\x81R`\x04\x90\xFD[4a\x01,Wa\x0C\x196a\x0BaV[Pa\x0C\"a\x14xV[P`@\x80Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01Ra\x01\0\x93`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x85\x81`$\x81\x86Z\xFA\x95\x86\x15a\x03\x8CW`\0\x96a\x0F\x18W[PP\x82Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x82\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x92`\x80\x90\x84\x90\x81\x80`$\x81\x01\x03\x91Z\xFA\x80\x15a\x03\x8CWa\x03J\x96\x85\x94`\0\x92a\x0E\xF8W[Pa\x0C\xF9a\x0C\xEFa\x07\xA7a\tQ\x87`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[`@\x01Q\x90`\0\x90V[\x95\x90a\r\x03a\x06\xF4V[\x98`\0\x8ARa\r+\x86\x8B\x01\x97`\0\x89R`\0\x85\x8D\x01R``\x8C\x01\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x15\x80\x15`\x80\x8B\x01Ra\x0EyWPa\r\xF5``a\x0E\x01\x94a\x0E\x13a\x0E\x06a\x0E\x01\x87a\r\xFBa\r\xF5a\r\xEBa\x0E(\x9Fa\n\xEAa\x0E\x1B\x9F\x9Da\r\xE4\x8F\x93a\r\xFB\x9Fa\n\xEAa\r\xD0a\r\xBC\x97a\r\xC1a\r\xBC\x88a\r\xB6a\n\xEAa\r\xA8a\r\xA3a\r\xDE\x9A`\x01`\x01`\x80\x1B\x03\x9E\x01\x9E\x8FQ`\x01`\x01`\x80\x1B\x03\x16\x90V[a1\nV[\x92Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x14/V[a\x14\nV[\x9C\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a1%V[\x91\x16a\x14/V[\x9A\x87\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x90a\x14^V[a3TV[`\x01`\x01`\x80\x1B\x03\x16\x8CRV[\x01Q`\xFF\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90RV[Q\x91\x82\x91\x82\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[a\x0E\xF3\x96P\x84a\x0E\x01\x94a\x0E\x13a\x0E\x06a\x0E\x01a\x0E\xE1`\x01`\x01`\x80\x1B\x03a\x0E\xDBa\x0E\xCDa\r\xBC\x8Ca\x0E\x1B\x9Fa\n\xEAa\r\xB6\x91a\r\xFB\x9Fa\r\xF5\x9Fa\r\xDEa\n\xEAa\n\xF6\x93\x88\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16a\x14\nV[\x97a\r\xFBa\r\xF5``\x87\x01Q`\xFF\x16\x90V[a\x0E(V[a\x0F\x11\x91\x92P`\x80=\x81\x11a\x03\xD1Wa\x03\xC2\x81\x83a\x06\xD3V[\x908a\x0C\xC8V[a\x0F/\x92\x96P\x80=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x938\x80a\x0C\x86V[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x913\x83\x90\x03a\x0B\xF9Wa\x07\xA2a\x0F}\x91a\x0F\xEA\x936\x91a\x07\x0EV[a\x0F\x9A\x84`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x81Q`\x01`\x01`\x80\x1B\x03\x16` \x83\x01Qc\xFF\xFF\xFF\xFF\x16\x91`\x01`\x01`\x80\x1B\x03a\x0F\xD7`\x80a\x0F\xCF`@\x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x96\x01Q\x15\x15\x90V[\x94c\xFF\xFF\xFF\xFF\x80\x91\x16\x94\x16\x92\x16\x90a\x1C\xFDV[\x7F\x8Di\xFAt\x9A\xF4\x15\xF9<\x96\x86>\xBF6\x92\x93&#\xC0\x10\xEB\xA8\x11\x8B\x02\xF0DXZ\xABe\x13`\x01`\x01`@\x1B\x03a\x103a\tQ\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x93a\x10\xB6a\x10H\x86Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95a\x10Z` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x10y`\x80a\x10q`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92\x01Q\x15\x15\x90V[\x91`@Q\x95\x86\x95\x16\x98\x85\x92\x91``\x92\x95\x94\x91\x95`\x01`\x01`\x80\x1B\x03`\x80\x86\x01\x97\x16\x85Rc\xFF\xFF\xFF\xFF\x80\x92\x16` \x86\x01R\x16`@\x84\x01R\x15\x15\x91\x01RV[\x03\x90\xA3`\0\x90V[\x90`@Qa\x10\xCB\x81a\x06\x98V[`\x80`\xFF\x82\x94T`\x01`\x01`\x80\x1B\x03\x81\x16\x84Rc\xFF\xFF\xFF\xFF\x80\x82\x85\x1C\x16` \x86\x01R\x80\x82`\xA0\x1C\x16`@\x86\x01R\x81`\xC0\x1C\x16``\x85\x01R`\xE0\x1C\x16\x15\x15\x91\x01RV[Q\x90a\x01\x85\x82a\x07\xBEV[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01,WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\x01,WV[Q\x90a\x01\x85\x82a\x01gV[\x80\x91a\x01\0\x92\x83\x91\x03\x12a\x01,W`@Q\x91\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3Wa\x11\xEA\x91`\xE0\x91`@Ra\x11|\x81a\x11\rV[\x84Ra\x11\x8A` \x82\x01a\x11\rV[` \x85\x01Ra\x11\x9B`@\x82\x01a\x11\rV[`@\x85\x01Ra\x11\xAC``\x82\x01a\x11\x18V[``\x85\x01Ra\x11\xBD`\x80\x82\x01a\x11)V[`\x80\x85\x01Ra\x11\xCE`\xA0\x82\x01a\x11)V[`\xA0\x85\x01Ra\x11\xDF`\xC0\x82\x01a\x118V[`\xC0\x85\x01R\x01a\x118V[`\xE0\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x82\x01Ra\x01\0\x93\x92\x91\x90\x84\x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x94\x85\x15a\x03\x8CW`\0\x95a\x12\xAAW[PP\x90a\x12\x94a\x12\x8Aa\tQa\x12\x9A\x94`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91B\x90\x83\x87a!NV[\x93a(hV[a\x12\xA4W`\x01\x91\x90V[`\0\x91\x90V[a\x12\x9A\x93\x92\x95Pa\tQa\x12\xD1a\x12\x94\x93\x83a\x12\x8A\x94\x90=\x10a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x96\x93\x94PPa\x12aV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x13\nWV[a\x12\xDBV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x13\nWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x13\nWV[a\x13W\x90`\x01\x92a\x132V[\x12a\x13aW`\x01\x90V[`\0\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x01,WV[\x90\x81`\x80\x91\x03\x12a\x01,W`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3Wa\x13\xDA\x91``\x91`@R\x80Qa\x13\xAE\x81a\x01gV[\x84Ra\x13\xBC` \x82\x01a\x13fV[` \x85\x01R`@\x81\x01Qa\x13\xCF\x81a\x01gV[`@\x85\x01R\x01a\x13fV[``\x82\x01R\x90V[\x90\x81` \x91\x03\x12a\x01,WQ\x90V[\x90`\x12\x03`\x12\x81\x11a\x13\nWa\x14\x06\x90a\x14<V[\x02\x90V[`\0\x19\x81\x01\x91\x90\x82\x11a\x13\nWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x13\nWV[\x91\x90\x82\x03\x91\x82\x11a\x13\nWV[`M\x81\x11a\x13\nW`\n\n\x90V[`\x12\x03`\x12\x81\x11a\x13\nWa\x0B\x9A\x90a\x14<V[\x90`\x12\x03`\x12\x81\x11a\x13\nWa\x14s\x90a\x14<V[\x90\x04\x90V[`@Q\x90a\x14\x85\x82a\x06\x98V[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x06\xB3W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[``\x81\x01\x80Q`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R\x93\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92\x90\x91a\x01\0\x80\x87`$\x81\x88Z\xFA\x91\x82\x15a\x03\x8CWa\x15\xCB\x97`\0\x93a\x17\xCDW[Pa\x15\x9Fa\x15\x94a\x15\x88\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[` \x1Cb\xFF\xFF\xFF\x16\x90V[`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF\x90\x91\x16`\x04\x82\x01R`\x80\x98\x90\x91\x90\x89\x90\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x8AZ\xFA\x90\x81\x15a\x03\x8CWa\x16\xDC` \x94a\x0E\x1B`\xA0`\x04\x9Da\x16\xD5a\x0E\x06a\x16\xCC\x8B\x8Ea\t8\x9Ba\x16\xE9\x9D`\0\x91a\x17\xB0W[Pa\x16\ta\x14\xA3V[\x80\x9Aa\x16\x17\x89\x85\x01Q\x15\x15\x90V[\x15a\x17TW\x82a\x16?a\x16\x85\x93`@\x93a\x167\x89a\x16\x95\x98\x01Q`\xFF\x16\x90V[`\xFF\x16\x91\x01RV[a\x16T\x8Da\x01 a\x167``\x85\x01Q`\xFF\x16\x90V[\x80Qa\x16w\x90\x8E\x90`\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x8B\x01RV[a\x16\xA9a\n\xEA\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x8A\x01R\x01\x96a\x16\xC3a\n\xEA\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x82\x01Ra\x17\xECV[\x92\x83\x01Qa3TV[\x01Qa3TV[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x94`@Q\x97\x88\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x93\x84\x15a\x03\x8CWa\x17 \x96`\0\x95a\x170W[Pa\x17\x1A\x90a\x10\xBEV[\x90a\"\x1BV[\x90\x91Pa\x17-\x81\x83a\x13KV[\x92V[a\x17\x1A\x91\x95Pa\x17M\x90` =\x81\x11a\x03\x85Wa\x03v\x81\x83a\x06\xD3V[\x94\x90a\x17\x10V[\x91a\x17na\x16\x85\x92a\x17\xAB\x94a\x167``\x85\x01Q`\xFF\x16\x90V[a\x17\x82\x8Ca\x01 a\x167\x88\x85\x01Q`\xFF\x16\x90V[`@\x81\x01Qa\x17\x9E\x90\x8D\x90`\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16a\x16iV[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x16\x95V[a\x17\xC7\x91P\x87=\x89\x11a\x03\xD1Wa\x03\xC2\x81\x83a\x06\xD3V[8a\x16\0V[a\x17\xE5\x91\x93P\x82=\x84\x11a\x04\0Wa\x03\xF0\x81\x83a\x06\xD3V[\x918a\x15pV[a\x17\xF4a\x14\xA3V[P`@\x81\x01\x80Q\x90a\x18\x10a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90a\x13\xF1V[\x90R``\x82\x01a\x18&\x81Q`\xFF\x84Q\x16\x90a\x13\xF1V[\x90Ra\x18=`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90a\x13\xF1V[\x90R`\xA0\x81\x01a\x18W\x81Q`\xFFa\x01 \x85\x01Q\x16\x90a\x13\xF1V[\x90R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x13\nW`\0\x19\x83\x05\x03a\x13\nWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x13\nW\x81\x84\x05\x14\x90\x15\x17\x15a\x13\nWV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x13\nWV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x80\x83\x10\x15a\x19\x9AW\x82\x15a\x19\x8CWa\x19#`\x80\x83\x01Qa0\xE3V[\x90a\x19-\x83a\x19\xA4V[\x93\x81\x03\x90\x80\x82\x11a\x13\nWa\x19}`@\x93g\x1B\xC1mgN\xC8\0\0a\x19va\x19ba\x0B\x9A\x99a\x19]a\x19\x83\x98a*\x03V[a\x18\xB3V[\x92a\x19q``\x8A\x01Q\x80a\x18\xD6V[a\x18\xD6V[\x04\x90a\x132V[\x05a-\x86V[\x91\x01Q\x90a1%V[PPP`\x01`\x01`\x80\x1B\x03\x90V[P`@\x91P\x01Q\x90V[a\x19\xB9a\x19\xB4`\x80\x83\x01Qa0\xE3V[a2.V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x13\nW``a\x0B\x9A\x92\x01Qa1%V[`@\x81\x01Q`\0\x93\x92a\x19\xEF\x91\x90a1FV[\x80a\x1A\x02W[Pa\x0B\x9A\x90\x83\x81Ra\x1B\xF8V[a\x1A\x11a\x1A;\x91\x94\x92\x94a/BV[a\x1A[a\x1A!`\x80\x87\x01Qa0\xE3V[a\x1AU``\x88\x01Qg\x1B\xC1mgN\xC8\0\0\x95\x81\x87\x92a\x18\xD6V[\x04\x91a\x1AOa\x1AI\x8Aa\x19\xA4V[\x95a\x18\\V[\x92a\x18\xB3V[\x90a\x1B\xDCV[\x90\x80\x15a\x1A\xC9W`\x01`\xFF\x1B\x82\x14`\0\x19\x82\x14\x16a\x13\nWa\x1A\x96g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1A\x90a\x1A\x9B\x93a\x1A\xA0\x95\x05a\x18\\V[\x05a(\x94V[a(\xCDV[a\x18\\V[\x05g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x03\x92\x90\x91\x12\x80\x15\x82\x84\x13\x16\x91\x83\x12\x16\x17a\x13\nW\x91a\x0B\x9Aa\x19\xF5V[a\x18\xE9V[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x13\nWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x13\nWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x13\nWV[`@\x81\x01\x80Q\x82Q\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x10\x15a\x1CiW\x15a\x1CbWPa\x1C\"\x83a\x19\xA4V[\x90\x83Q\x81\x03\x90\x81\x11a\x13\nWa\x0B\x9A\x93`\xA0a\x1CLa\x1CZ\x94a\x1CGa\x1CU\x95a*\x03V[a\x132V[\x91\x01Q\x90a\x1B\xDCV[a(\xA5V[\x90Q\x90a1%V[\x92PPP\x90V[PPPPP`\0\x90V[`\xA0`@Qa\x1C\x81\x81a\x06\xB8V[`\0\x91\x81\x83\x80\x93R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x01R`\x01`\x01`\x80\x1B\x03\x82Q\x16\x91c\xFF\xFF\xFF\xFF`@a'\x10g\r\xE0\xB6\xB3\xA7d\0\0\x83` \x86\x01Q\x16\x02\x04\x92\x01Q\x16\x90`@Q\x93a\x1C\xDD\x85a\x06\xB8V[\x83\x85R\x83` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x90V[\x92\x93\x91\x90\x93c\xFF\xFF\xFF\xFF\x92\x83a\x1D\x1B\x86Tc\xFF\xFF\xFF\xFF\x90`\xC0\x1C\x16\x90V[\x16a\x1E\x95W\x80\x15a\x1E4W\x84T`\xFF`\xE0\x1B\x19\x16\x90\x15\x15`\xE0\x1B`\xFF`\xE0\x1B\x16\x17\x84UP\x82Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16c\x01\xE1\x85Y`\xA0\x1B\x17\x83U[aa\xA8\x80\x82\x10\x90\x82\x14\x17`\x01\x82\x11`\x01\x83\x14\x17\x16\x15a\x1E\"Wa\x1D|a\x1D\x9C\x91a3lV[\x83Tc\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x80\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\x80\x1B\x16\x17\x83UV[`\x01`\x01`\x80\x1B\x03\x80\x84\x10\x90\x84\x14\x17`\x01\x84\x11`\x01\x85\x14\x17\x16\x15a\x1E\x10Wa\x1D\xEEa\x1D\xC9a\x01\x85\x94a3TV[\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x17\x83UV[\x81Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16B\x91\x90\x91\x16`\xC0\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x17\x90UV[`@Qc\xB2B\xE3A`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9]8\x19`\xE0\x1B\x81R`\x04\x90\xFD[Pb\x01Q\x80\x81\x81\x14\x90\x82\x11\x17c\x05\xA4\x90\x0B\x82\x81\x14\x90\x83\x10\x17\x16\x15a\x1E\x83Wa\x1E^a\x1E~\x91a3lV[\x84Tc\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17\x84UV[a\x1DWV[`@Qc\xB5\x97\x03\x0F`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc\x1E\x13\x89\xA1`\xE2\x1B\x81R`\x04\x90\xFD[`\x80\x81\x01Qa\x1E\xCCWa\x1E\xBEc\xFF\xFF\xFF\xFF\x91a\x1E\xD6V[\x16\x80\x82\x10\x90\x82\x03\x02\x81\x03\x03\x90V[PPc\x01\xE1\x85Y\x90V[`\x80\x81\x01Qa\x1E\xF8Wc\xFF\xFF\xFF\xFF\x90\x81`@\x81``\x84\x01Q\x16\x92\x01Q\x16\x01\x16\x90V[`@Qc\xB0\x19\x84\x97`\xE0\x1B\x81R`\x04\x90\xFD[a\x1F\x12a\x14xV[P`\xA0\x81Q\x03a\x1F\x8EW`\xA0\x81\x80Q\x81\x01\x03\x12a\x01,W`\xA0`@Q\x91a\x1F8\x83a\x06\x98V[` \x81\x01Qa\x1FF\x81a\x07\xBEV[\x83Ra\x1FT`@\x82\x01a\x11\x18V[` \x84\x01Ra\x1Fe``\x82\x01a\x11\x18V[`@\x84\x01Ra\x1Fv`\x80\x82\x01a\x11\x18V[``\x84\x01R\x01Qa\x1F\x86\x81a\x01]V[`\x80\x82\x01R\x90V[`@Qc\x01YW\xD3`\xE2\x1B\x81R`\x04\x90\xFD[a\x0B\x9A\x91a\x1F\xB8a\n\xEA\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91`\x01`\x01`\x80\x1B\x03a \x02a\x1F\xE5a\x1F\xDEa\n\xEA`@\x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x96a1FV[\x94a\x1F\xFDa\n\xEA` \x86\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a1FV[\x92a ga \x17\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a ac\xFF\xFF\xFF\xFF``a Ta Ba 9` \x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[g\r\xE0\xB6\xB3\xA7d\0\0a'\x10\x91\x02\x04\x90V[\x97\x01Q\x16c\xFF\xFF\xFF\xFF\x16\x90V[\x90a\x1E\xA7V[\x93a pa\x07\x01V[\x95\x86R` \x86\x01R\x16`@\x84\x01R``\x83\x01R`\x80\x82\x01R`\0`\xA0\x82\x01R[a\x0B\x9A\x90a \xF4a \xA0\x82a\x19\xA4V[\x91a \xAF\x81`@\x01Q\x90`\0\x90V[\x82Q\x90\x92\x91\x90g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10a!*WP`\x01\x92[` \x83\x01Q\x91\x82\x10a \xF9WPPPa\x1CGa \xEEg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92[a*\x03V[\x91a*\x03V[a\x1B\xDCV[\x81\x11a!\x0FWPPa\x1CGa \xEE`\x01\x92a*\x03V[a!$a \xEE\x91`@a\x1CG\x94\x01Q\x90a1\xFEV[\x92a*\x03V[\x80a!?WPg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x92a \xC9V[a!H\x90a\x14\x19V[\x92a \xC9V[\x90`\x01`\x01`\x80\x1B\x03a\x0B\x9A\x94\x92a!pa\n\xEA\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a!\x87a\n\xEA` \x84\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95\x15a!\xEBWa!\xB1a!\xAAa\n\xEA`@a!\xB7\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a1FV[\x95a1\xFEV[\x92[a ga!\xCD\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91a!\xE5a Ba 9` \x88\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x94a\x1E\xA7V[a\"\x0Fa\"\x08a\n\xEA`@a\"\x15\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x80\x92a1\xFEV[\x95a1FV[\x92a!\xB9V[a\x0B\x05a\n\xEAa#3\x93a#$`@a\"\xA9a\"D\x9A\x99\x98\x99a\"K\x8A`\x80\x8D\x01\x9D\x8EQ\x15\x15\x90V[\x87\x8Ba!NV[\x9Ca\"]\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[` \x8A\x01Q`\xC0\x8B\x01Q`\x01`\x01`\x80\x1B\x03\x90\x91\x16\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x03a#XW`\xA0\x8A\x01Qa\xFF\xFF\x16\x9C[`\x01`\x01`\x80\x1B\x03\x9Da\xFF\xFF\x16\x92\x8E\x16\x91\x8E\x16\x90a#gV[\x98\x92P\x94\x90Pa\"\xE1a\"\xC3\x82Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9Aa\"\xDBa Ba 9` \x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x92a\x1E\xA7V[\x90a\"\xEAa\x07\x01V[\x9B`\0\x8DR` \x8D\x01\x9B`\0\x8DR\x16\x84\x8D\x01R``\x8C\x01R`\x80\x8B\x01R`\0`\xA0\x8B\x01R\x01\x91a\x0B\x05a\n\xEA\x84Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x87RQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81Ra#Ha#A\x83a \x90V[\x93Q\x15\x15\x90V[\x15a#RWPQ\x92V[\x90PQ\x92V[`\x80\x8A\x01Qa\xFF\xFF\x16\x9Ca\"\x90V[\x91\x94\x92\x94`\0a#\x82`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16a1\xB0V[\x95\x80\x15\x80\x15a$\xB5W[PP\x80\x95\x80\x97`\x80\x86\x01\x92a#\xD8a#\xA4\x85Q\x15\x15\x90V[\x93\x84\x15a$\xAEW\x87\x94[\x15a$\xA4Wa#\xD3\x87\x95[a#\xCDa\n\xEA\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a$\xD1V[a\x14/V[\x90\x81\x81\x11a$\x92Wa#\xED` \x91\x85\x93a\x14/V[\x97\x01\x91a$\x01\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11a$\x80Wa$)\x91a\r\xB6a\n\xEAa$\"\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x91Q\x15\x15\x90V[\x90\x81\x15a$wW\x84\x85\x92[\x15a$oWP\x92[\x14a$]W\x81\x14a$KW\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92a$<V[\x80\x94\x85\x92a$4V[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[a#\xD3\x88\x95a#\xB9V[\x86\x94a#\xAEV[\x90\x91Pa\x1A\xC9W\x85\x04\x94\x85\x81\x03\x90\x81\x11a\x13\nW\x948\x80a#\x8CV[\x91\x90\x82\x01\x80\x92\x11a\x13\nWV[`\0\x19\x81\x14a\x13\nW`\x01\x01\x90V[\x93\x91a%\x1Ea%\x06`\x80\x93\x97a%)\x95\x87\x85\x8B\x8Ba\"\x1BV[P\x95\x90\x97a%\x13\x81a\x1CsV[\x96`\xA0\x88\x01Ra\x1E\xA7V[\x82\x85\x01R\x01Q\x15\x15\x90V[\x92\x83\x15a&[W\x81Ra%;\x81a\x1B\xF8V[\x80` \x83\x01R[\x80\x15a&*W\x91a%\xF9a%\xE2a%\xDDa%\xC1`\x01`\x01`\x80\x1B\x03\x95\x87a%\xCFa%xa%ra\x0B\x9A\x9C\x9Ba1hV[\x92a1\xD6V[\x92`@Q\x94\x85\x91` \x83\x01\x91\x90\x91`\xA0\x80`\xC0\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x85R\x84a\x06\xD3V[\x88\x15a&\"W`\x02\x92a&qV[a$\xDEV[a\r\xDEa\n\xEA`@\x85\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x92\x15a&\x13W` \x01Q`\x01`\x01`\x80\x1B\x03\x16[\x16a\x14/V[Q`\x01`\x01`\x80\x1B\x03\x16a&\rV[`\x01\x92a&qV[PP`\x01`\x01`\x80\x1B\x03\x91`\0\x14a&MW` \x01Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[Q`\x01`\x01`\x80\x1B\x03\x16\x16\x90V[` \x82\x01Ra&i\x81a'\xA0V[\x80\x82Ra%BV[\x91\x90\x93\x92\x93`\0\x94`\0\x92\x80\x83\x11a'\x7FWa&\x8E\x83\x86\x84a3\x98V[\x90a&\x9A\x81\x87\x85a3\x98V[\x85a&\xA5\x82\x85a\x18\xB3V[\x13a'^WP\x95\x94\x93a&\xB8\x84\x88a\x14/V[\x94\x81`\x01\x96\x87\x80[a&\xD2W[PPPPPPPPP\x90PV[\x15a'9W[P\x86\x97\x98\x99P\x80\x92a&\xF3a&\xED\x8C\x89a$\xD1V[`\x01\x1C\x90V[\x9A\x8B\x90a'\x01\x8D\x86\x8Aa3\x98V[\x90\x84a'\r\x89\x84a\x18\xB3V[\x13a'+WPP\x93[\x88a'!\x89\x87a\x14/V[\x92\x01\x94\x9A\x99a&\xC0V[\x95\x96P\x97PP\x8A\x96\x94a'\x16V[\x87\x10\x80a'SW[\x15a'LW\x89a&\xD8V[\x80\x80a&\xC5V[Pa\x01\0\x83\x10a'AV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@\x81\x01\x80Q\x91` \x81\x01\x92\x83Q\x90\x81\x10\x15a(\x0FW\x15a(\0W`\xA0a'\xE1a'\xEA\x94a \xF4a \xE9a\x1CU\x96a'\xD7\x87a\x19\xA4V[\x93Q\x90Q\x90a1\xFEV[\x91\x01Q\x90a\x132V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x13\nW\x90V[PPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[PPPP`\0\x90V[\x90\x81`\xC0\x91\x03\x12a\x01,W`\xA0`@Q\x91a(2\x83a\x06\xB8V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R`\x80\x81\x01Q`\x80\x84\x01R\x01Q`\xA0\x82\x01R\x90V[`\x80\x82\x01Qa(\x8DW``\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91a(\x87\x90a\x1E\xD6V[\x16\x11\x15\x90V[PP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\x13\nW`\0\x03\x90V[a(\xC9a\x1A\x9Ba\x1A\x96g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x1A\x90g\x1B\xC1mgN\xC8\0\0\x95a\x18\\V[\x05\x90V[\x80\x15a)\xF6WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a)\xF0WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a)\xE3Wa)\xD1a)\0\x82a+\xAEV[a)\x94a)\xCCa)\x1Fa)\x1Aa)\x15\x85a1\x85V[a\x1A\xCEV[a-EV[\x92a \xF4a)\xC7a)\xC2a)\xBBa)\xB5a)\xB0a)\xAAa)\xA5a)\x9Fa)\x9A\x8Da)\x94a)\x8Fa)\x89a)\x84a)~a)ya)sa)na)ha)c\x8Aa+\xDBV[a\x1A\xE6V[\x89a,}V[a\x1B\0V[\x87a,}V[a\x1B\x18V[\x85a,}V[a\x1B2V[\x83a,}V[a\x1BJV[\x90a,}V[a\x1BdV[\x8Ca,}V[a\x1B|V[\x8Aa,}V[a\x1B\x94V[\x88a,}V[\x93\x80a,}V[a\x18yV[a\x12\xF1V[a-\x86V[\x90`\0\x13\x15a\x0B\x9AWa\x0B\x9A\x90a\x13\x0FV[Pg\x1B\xC1mgN\xC8\0\0\x90V[P`\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a)\xF0Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a+XW\x81\x15a+yW`\x01\x82\x81\x1B\x91`\x02\x93\x83\x05\x84\x03a\x13\nW`\0\x83\x12\x80\x15a+\x9DW[a+\x8BW\x82\x15a+XWg\x1B\xC1mgN\xC8\0\0\x83\x14a+yW\x82\x12\x91\x82\x15a+jW\x92[a*r\x84a,\xC5V[\x80\x15a+XWa*\xCFa*\x9Ea*\x99a\x19\xB4a*\x94a*\xD4\x95\x99\x97\x96\x99a/BV[a,\x06V[a\x18\x9AV[a\x1CGa*\xB2a*\xAD\x83a,\xF0V[a\x1B\xACV[a*\xC9a)\x15a)~a*\xC4\x86a-\x1BV[a\x1B\xC4V[\x90a-dV[a,.V[\x93`\0\x92[\x81\x84\x10a+\x0BWPPPPa\x0B\x9A\x91a*\xF8\x91`\0\x14a*\xFDWa,\x9EV[a(\x94V[a+\x06\x90a(\x94V[a,\x9EV[\x90\x91a+N\x86a\x1AUa+#\x85a\x1CG\x86\x99\x9Ba(\xCDV[a*\xC9a+>a+9a)\xCCa*\xF8\x87\x80a,}V[a,VV[a+H\x83\x86a,}V[\x90a\x132V[\x95\x01\x92\x91\x90a*\xD9V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a+s\x90a\x13\x0FV[\x92a*iV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a*EV[`\x01`\xFF\x1B\x81\x14a+\xC9W`\0\x81\x12\x15a\x0B\x9AW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x01,Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01,W\x05\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a)\xF0Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a.\xD6We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x15a/\x11WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a/n`\0\x82\x13a/\nV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a/\x8A\x82a2\xECV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wc\x01\xE1\x85Y\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x81\x81\x02\x04\x90\x81\x14`\x01\x16\x15a\x01,W\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01,W\x04\x90V[`2\x81\x02\x90`2\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`d\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,Wg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x96\x81\x02\x90`\x96\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01,W`\x01`d`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01,W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a2\xD5W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a2\xC8W[e\x01\0\0\0\0\0\x81\x10\x15a2\xBBW[c\x01\0\0\0\x81\x10\x15a2\xAEW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a2rV[` \x1C\x91`\x10\x1B\x91a2eV[`@\x1C\x91` \x1B\x91a2VV[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca2>V[a2\xF7\x81\x15\x15a/\nV[\x80`\x01`\x01`\x80\x1B\x03\x10`\x07\x1B\x81\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\x80\x1B\x81\x10\x15a\x01,W`\x01`\x01`\x80\x1B\x03\x16\x90V[d\x01\0\0\0\0\x81\x10\x15a\x01,Wc\xFF\xFF\xFF\xFF\x16\x90V[cNH{q`\xE0\x1B`\0R`Q`\x04R`$`\0\xFD[\x80`\x02\x14a3\xEEW`\x01\x03a3\x82W\x80` \x80a3\xBA\x93Q\x83\x01\x01\x91\x01a(\x18V[\x90\x81R`\xA0a3\xC8\x82a \x90V[\x91\x01Q`\x01\x81\x01\x90`\0`\x01\x83\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x13\nWa\x0B\x9A\x91a\x132V[P\x80` \x80a4\x02\x93Q\x83\x01\x01\x91\x01a(\x18V[\x90` \x82\x01R`\xA0a3\xC8\x82a \x90V\xFE\xA2dipfsX\"\x12 \x15\xAE\xFC\xB37\x02\xB8X\xED\x0B\x82[\x1F\x81\xE0\x0F#}\x05\xEE\x7FD\xDBK\xF1\x10\xB9\xF9n\x8BT\x19dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static NORMALSTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
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
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    NORMALSTRATEGY_ABI.clone(),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256),
        > {
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
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
            (bool, ::ethers::core::types::I256, ::ethers::core::types::I256),
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::I256),
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AfterCreateFilter,
        > {
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
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NormalStrategyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for NormalStrategy<M> {
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded)
                = <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
            }
            if let Ok(decoded)
                = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded)
                = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded)
                = <NormalStrategyLib_ConfigExists as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NormalStrategyLib_ConfigExists(decoded));
            }
            if let Ok(decoded)
                = <NormalStrategyLib_InvalidDuration as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NormalStrategyLib_InvalidDuration(decoded));
            }
            if let Ok(decoded)
                = <NormalStrategyLib_InvalidStrategyArgs as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NormalStrategyLib_InvalidStrategyArgs(decoded));
            }
            if let Ok(decoded)
                = <NormalStrategyLib_InvalidStrikePrice as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NormalStrategyLib_InvalidStrikePrice(decoded));
            }
            if let Ok(decoded)
                = <NormalStrategyLib_InvalidVolatility as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NormalStrategyLib_InvalidVolatility(decoded));
            }
            if let Ok(decoded)
                = <NormalStrategyLib_NonExpiringPool as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NormalStrategyLib_NonExpiringPool(decoded));
            }
            if let Ok(decoded)
                = <NormalStrategy_NotPortfolio as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NormalStrategy_NotPortfolio(decoded));
            }
            if let Ok(decoded)
                = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            if let Ok(decoded)
                = <SwapLib_OutputExceedsReserves as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapLib_OutputExceedsReserves(decoded));
            }
            if let Ok(decoded)
                = <SwapLib_ProtocolFeeTooHigh as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapLib_ProtocolFeeTooHigh(decoded));
            }
            if let Ok(decoded)
                = <SwapLib_ZeroXAdjustment as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapLib_ZeroXAdjustment(decoded));
            }
            if let Ok(decoded)
                = <SwapLib_ZeroYAdjustment as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
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
                Self::Infinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::OutOfBounds(element) => {
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
                Self::BisectionLib_InvalidBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::NormalStrategy_NotPortfolio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapLib_OutputExceedsReserves(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ProtocolFeeTooHigh(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ZeroXAdjustment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapLib_ZeroYAdjustment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<NormalStrategyLib_InvalidDuration>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidDuration) -> Self {
            Self::NormalStrategyLib_InvalidDuration(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidStrategyArgs>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidStrategyArgs) -> Self {
            Self::NormalStrategyLib_InvalidStrategyArgs(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidStrikePrice>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidStrikePrice) -> Self {
            Self::NormalStrategyLib_InvalidStrikePrice(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_InvalidVolatility>
    for NormalStrategyErrors {
        fn from(value: NormalStrategyLib_InvalidVolatility) -> Self {
            Self::NormalStrategyLib_InvalidVolatility(value)
        }
    }
    impl ::core::convert::From<NormalStrategyLib_NonExpiringPool>
    for NormalStrategyErrors {
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    #[ethcall(name = "getAmountOut", abi = "getAmountOut(uint64,bool,uint256,address)")]
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
            if let Ok(decoded)
                = <AfterCreateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AfterCreate(decoded));
            }
            if let Ok(decoded)
                = <ApproximateReservesGivenPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ApproximateReservesGivenPrice(decoded));
            }
            if let Ok(decoded)
                = <BeforeSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BeforeSwap(decoded));
            }
            if let Ok(decoded)
                = <ConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Configs(decoded));
            }
            if let Ok(decoded)
                = <GetAmountOutCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAmountOut(decoded));
            }
            if let Ok(decoded)
                = <GetInvariantCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetInvariant(decoded));
            }
            if let Ok(decoded)
                = <GetMaxOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetMaxOrder(decoded));
            }
            if let Ok(decoded)
                = <GetSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSpotPrice(decoded));
            }
            if let Ok(decoded)
                = <GetStrategyDataCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetStrategyData(decoded));
            }
            if let Ok(decoded)
                = <PortfolioCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Portfolio(decoded));
            }
            if let Ok(decoded)
                = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded)
                = <ValidatePoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidatePool(decoded));
            }
            if let Ok(decoded)
                = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NormalStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AfterCreate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproximateReservesGivenPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BeforeSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Configs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAmountOut(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInvariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetMaxOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetStrategyData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Portfolio(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidatePool(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
    impl ::core::convert::From<ApproximateReservesGivenPriceCall>
    for NormalStrategyCalls {
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
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
        Hash
    )]
    pub struct ValidateSwapReturn(pub bool, pub ::ethers::core::types::I256);
}
