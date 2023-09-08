pub use geometric_mean_strategy::*;
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
pub mod geometric_mean_strategy {
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
                    ::std::borrow::ToOwned::to_owned("MINIMUM_INVARIANT_DELTA"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MINIMUM_INVARIANT_DELTA",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
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
                                    name: ::std::borrow::ToOwned::to_owned("assetWeightWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quoteWeightWad"),
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
                                    name: ::std::borrow::ToOwned::to_owned("assetWeightWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("quoteWeightWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("assetInWad"),
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
                                    name: ::std::borrow::ToOwned::to_owned("assetWeightWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("quoteWeightWad"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned(
                        "GeometricMeanStrategy_NotPortfolio",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "GeometricMeanStrategy_NotPortfolio",
                            ),
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
    pub static GEOMETRICMEANSTRATEGY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@\x90\x80\x82R4a\x01?WP\x80Q`\x1Fa#!8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01)W\x80\x84\x92` \x94\x87R\x839\x81\x01\x03\x12a\0\xDAWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\0\xD5W\x7F;5\xCD\xE5\x85\"y\xBF\xD1\xCE\x80S\x8F\x94\xC2\xE4*S\x0F\x11\x1FVB\x9B\x91,\x85\xCF\xA6P\xD4\xD3\x91` \x91`\x80R\x83Q\x90\x81R\xA1Qa!\x94\x90\x81a\x01\x8D\x829`\x80Q\x81\x81\x81a\x01\xBD\x01R\x81\x81a\x02\x99\x01R\x81\x81a\x05l\x01R\x81\x81a\x08\xE2\x01R\x81\x81a\x0B\xDC\x01R\x81\x81a\r\x0F\x01R\x81\x81a\r\xEC\x01R\x81\x81a\x10\xF5\x01Ra\x19\\\x01R\xF3[`\0\x80\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@R`\x046\x10a\x10\x8FW`\x005`\xE0\x1C\x80c\x14\xA0\x11\xFC\x14a\0\xDCW\x80c\x16\xED\xE0\x16\x14a\0\xD7W\x80c\x19\x05x\x07\x14a\0\xD2W\x80c4\xDB\xC7;\x14a\0\xCDW\x80c9CMZ\x14a\0\xC8W\x80c\x7FI\x125\x14a\0\xC3W\x80c\x80\xAF\x9Dv\x14a\0\xBEW\x80c\xA4G\x89\x19\x14a\0\xB9W\x80c\xE0hx\x7F\x14a\0\xB4W\x80c\xE31\xBA4\x14a\0\xAFW\x80c\xE6\x04{\x19\x14a\0\xAAW\x80c\xECshT\x14a\0\xA5Wc\xF0{\x87\x9E\x03a\x10\x8FWa\r\xD2V[a\x0C\xFDV[a\x0C\x9CV[a\x0B\xC1V[a\nKV[a\x08\xC7V[a\x08$V[a\x06\x96V[a\x05QV[a\x04\xFEV[a\x02jV[a\x01\xA7V[a\x01\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xA2W`\x006`\x03\x19\x01\x12a\x01\x9DW` `@Q`\0\x81R\xF3[a\x011V[a\0\xE1V[4a\x01\xA2W`\x006`\x03\x19\x01\x12a\x01\x9DW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02\x02WV[`\0\x80\xFD[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02\x02WV[`$5\x90\x81\x15\x15\x82\x03a\x02\x02WV[`D5\x90\x81\x15\x15\x82\x03a\x02\x02WV[`\x845\x90\x81\x15\x15\x82\x03a\x02\x02WV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\x02WV[`\xC45\x90a\x02h\x82a\x02JV[V[4a\x01\xA2W`\x806`\x03\x19\x01\x12a\x01\x9DWa\x02\x83a\x01\xECV[a\x02\x8Ba\x02\x1DV[`d5a\x02\x97\x81a\x02JV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01Ra\x01\0\x92\x90\x83\x81`$\x81\x85Z\xFA\x93\x84\x15a\x04\x82W`\0\x94a\x04\xCFW[PP\x80;\x15a\x04\x87W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x86\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x91`\x80\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x82W`\0\x93a\x04\x9FW[P\x85\x15a\x04\x8CWa\x03_`\xFFa\x03Y\x84\x86\x01Q`\xFF\x16\x90V[\x16a\x15\x13V[\x93a\x03\xC5a\x03\x8Da\x03\x83\x8A`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x96`D5\x02a!FV[\x98a\x03\xA8a\x03\x99a\x08\x06V[`\x01`\x01`\x80\x1B\x03\x90\x9B\x16\x8BRV[`\x01\x8A\x86\x01R`\0`@\x8B\x01R`\x01`\x01`@\x1B\x03\x16``\x8A\x01RV[\x86\x15\x15`\x80\x89\x01R\x81;\x15a\x04\x87W\x82`\x04\x92`@Q\x93\x84\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x80\x15a\x04\x82Wa\x04<\x98a\x04+\x97`\xFF\x97a\x04\x18\x95`\0\x94a\x04OW[Pa\x04\x12\x90a\x11\xF7V[\x90a\x15AV[\x94\x15a\x04@WP``\x01Q`\xFF\x16a\x03YV[`@Q\x91\x04\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x01Q`\xFF\x16a\x03YV[a\x03YV[a\x04\x12\x91\x94Pa\x04t\x90\x88=\x8A\x11a\x04{W[a\x04l\x81\x83a\x07\xE5V[\x81\x01\x90a\x14\xDDV[\x93\x90a\x04\x08V[P=a\x04bV[a\x13MV[a\x12\x15V[a\x03_`\xFFa\x04J``\x86\x01Q`\xFF\x16\x90V[a\x04\xC1\x91\x93P`\x80=\x81\x11a\x04\xC8W[a\x04\xB9\x81\x83a\x07\xE5V[\x81\x01\x90a\x14oV[\x918a\x03@V[P=a\x04\xAFV[a\x04\xEF\x92\x94P\x80=\x10a\x04\xF7W[a\x04\xE7\x81\x83a\x07\xE5V[\x81\x01\x90a\x12\x9EV[\x918\x80a\x03\x01V[P=a\x04\xDDV[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DW`\x01`\x01`@\x1B\x03a\x05\x1Fa\x01\xECV[\x16`\0R`\0` R`@`\0 `\x01\x81T\x91\x01T\x90a\x04<`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DWa\x05ja\x01\xECV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x93a\x06\t\x93`\0\x93a\x06\x19W[PPa\x05\xFEa\x06\x03\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x11\xF7V[\x90a\x13YV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x06\x03\x92\x93Pa\x05\xFE\x91\x81a\x069\x92\x90=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x92\x91a\x05\xDCV[\x90\x93\x92\x91\x93``\x82R\x80Q\x90\x81``\x84\x01R`\0[\x82\x81\x10a\x06\x80WPP\x90`\x80\x82`\0\x82`@\x95\x85\x01\x01R`\x1F\x80\x19\x91\x01\x16\x82\x01\x01\x94` \x82\x01R\x01RV[\x80` \x80\x92\x84\x01\x01Q`\x80\x82\x87\x01\x01R\x01a\x06UV[4a\x01\xA2W`\x806`\x03\x19\x01\x12a\x01\x9DW`\x045`d5\x90`@Qa\x06\xBA\x81a\x07~V[\x81\x81R` \x81\x01`$5\x81R`@Q\x92` \x84\x01RQ`@\x83\x01R`@\x82Ra\x06\xE2\x82a\x07\xCAV[Qg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x81\x03\x11a\x07(W\x81\x11a\x07(Wa\x07\x1Ca\x07\x16a\x07\x0E\x83a\x04<\x94a EV[`D5a gV[\x84a EV[`@Q\x93\x84\x93\x84a\x06@V[a\x13vV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[`@Q\x90a\x02h\x82a\x07\xAFV[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x02\x02WV[4a\x01\xA2W6`\x03\x19\x01`\xE0\x81\x12a\x01\x9DW`\xA0\x13a\x08\xC2Wa\x04<a\x08\xA3`@Qa\x08O\x81a\x07\xAFV[`\x045a\x08[\x81a\x08\x13V[\x81R`$5a\x08i\x81a\x08\x13V[` \x82\x01Ra\x08va\x02,V[`@\x82\x01Ra\x08\x83a\x02\x07V[``\x82\x01Ra\x08\x90a\x02;V[`\x80\x82\x01Ra\x08\x9Da\x02[V[\x90a\x19TV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[a\x07-V[4a\x01\xA2W`\x806`\x03\x19\x01\x12a\x01\x9DWa\x08\xE0a\x01\xECV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01Ra\x01\0\x92\x90\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x04\x82Wa\t\xAC\x93`\0\x93a\t\xCBW[PPa\x05\xFEa\x06\x03\x91a\tua\th`D5a!FV[`\x01`\x01`\x80\x1B\x03\x16\x85RV[a\t\x93a\t\x83`d5a!FV[`\x01`\x01`\x80\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\t\xB8\x81`$5a\x148V[`@\x80Q\x91\x15\x15\x82R` \x82\x01\x92\x90\x92R\xF3[a\x06\x03\x92\x93Pa\x05\xFE\x91\x81a\t\xEB\x92\x90=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x92\x91a\tQV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xA2W`@6`\x03\x19\x01\x12a\x01\x9DWa\nda\x01\xECV[`$\x90\x815\x90`\x01`\x01`@\x1B\x03\x90\x81\x83\x11a\x0BrW6`#\x84\x01\x12\x15a\x0B\x1AW\x82`\x04\x015\x91\x82\x11a\n\xC2W6\x84\x83\x85\x01\x01\x11a\n\xBDWa\x04<\x93a\n\xAB\x93\x01\x90a\x10\xF2V[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[a\t\xF2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+\x81\x86\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+\x81\x86\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"\x81\x86\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DWa\x0B\xDAa\x01\xECV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x93a\x06\t\x93`\0\x93a\x0CuW[PPa\x05\xFEa\x0Cn\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[Q\x90a\x18wV[a\x0Cn\x92\x93Pa\x05\xFE\x91\x81a\x0C\x95\x92\x90=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x92\x91a\x0CLV[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DWa\x0C\xB5a\x01\xECV[P` `@Q`\x01\x81R\xF3[``\x90`\x03\x19\x01\x12a\x01\x9DW`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x02\x02W\x90`$5\x80\x15\x15\x81\x03a\x02\x02W\x90`D5a\x0C\xFA\x81a\x02JV[\x90V[4a\x01\xA2Wa\r\x0B6a\x0C\xC1V[PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x81\x90\x03a\r\xC0W\x80;\x15a\x04\x87W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x93a\r\xA8\x93`\0\x93a\x06\x19WPPa\x05\xFEa\x06\x03\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[`@Q\x91\x82\x91\x82\x91\x90` `@\x84\x01\x93`\x01\x81R\x01RV[`@Qc;\xAAq\x99`\xE2\x1B\x81R`\x04\x90\xFD[4a\x01\xA2Wa\r\xE06a\x0C\xC1V[Pa\r\xE9a\x18\xB5V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01Ra\x01\0\x93\x90\x84\x81`$\x81\x86Z\xFA\x94\x85\x15a\x04\x82W`\0\x95a\x10pW[PP\x81;\x15a\x04\x87W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x82\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x94\x90\x92`\x80\x90\x86\x90\x81\x80`$\x81\x01\x03\x91Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x95`\0\x92a\x10PW[Pa\x0E\xBFa\x05\xFE\x84`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[Pa\x0E\xC8a\x08\x06V[\x94`\0\x86Ra\x0E\xF1\x85\x87\x01\x94`\0\x86R`\0`@\x89\x01R``\x88\x01\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x15\x80\x15`\x80\x87\x01Ra\x0F\xF0W\x83a\x0Fpa\x0Fj``a\x0F\x90\x95a\x0F\x88a\x0F{a\x0Fv\x88a\x0Fpa\x0Fja\x0F`a\x0F\x9D\x9Fa\x0FE\x90a\x0FQa\x0Fv\x9Fa\x0F@a\x0FE\x91Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a\x18\xE0V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9A\x87\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x90a\x15'V[a!FV[`\x01`\x01`\x80\x1B\x03\x16\x8BRV[\x01Q`\xFF\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90RV[`@Q\x91\x82\x91\x82\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[\x80a\x0Fpa\x0Fj\x86a\x0F\x90\x95a\x0F\x88a\x0F{a\x0Fva\x109a\x0FEa\x10+a\x0FEa\x10K\x9Fa\x0F@\x90a\x0Fv\x9E\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x99Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x97a\x0Fpa\x0Fj``\x87\x01Q`\xFF\x16\x90V[a\x0F\x9DV[a\x10i\x91\x92P`\x80=\x81\x11a\x04\xC8Wa\x04\xB9\x81\x83a\x07\xE5V[\x908a\x0E\x9EV[a\x10\x87\x92\x95P\x80=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x928\x80a\x0ESV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x903\x83\x90\x03a\r\xC0W\x81`@\x91`\0` \x84Qa\x11:\x81a\x07~V[\x82\x81R\x01R\x81\x01\x03\x12a\x01\x9DW`@Qa\x11\x90\x91a\x11W\x82a\x07~V[\x805\x82R` \x80\x83\x01\x91\x015\x81Ra\x11\x82\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91Q\x90Q\x91`\x01\x91\x81U\x01UV[\x7FP\xA3\xE9\x9Ea\xA2\xAC\xDE\xB3\x9B\xB5/\x9ECg>\xE5\xE8\x0C\xFF\x1D\xDB\x90I\xFC\xB6y}\x88g\xD1\xCC`\x01`\x01`@\x1B\x03a\x11\xD9a\x05\xFE\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x80Q` \x91\x82\x01Q`@\x80Q\x92\x83R\x92\x82\x01R\x94\x90\x91\x16\x93\xA3`\x01\x90V[\x90`@Qa\x12\x04\x81a\x07~V[` `\x01\x82\x94\x80T\x84R\x01T\x91\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90a\x02h\x82a\x08\x13V[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x02WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\x02\x02WV[Q\x90a\x02h\x82a\x02JV[\x80\x91a\x01\0\x92\x83\x91\x03\x12a\x01\x9DW`@Q\x91\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99Wa\x13E\x91`\xE0\x91`@Ra\x12\xD7\x81a\x12hV[\x84Ra\x12\xE5` \x82\x01a\x12hV[` \x85\x01Ra\x12\xF6`@\x82\x01a\x12hV[`@\x85\x01Ra\x13\x07``\x82\x01a\x12sV[``\x85\x01Ra\x13\x18`\x80\x82\x01a\x12\x84V[`\x80\x85\x01Ra\x13)`\xA0\x82\x01a\x12\x84V[`\xA0\x85\x01Ra\x13:`\xC0\x82\x01a\x12\x93V[`\xC0\x85\x01R\x01a\x12\x93V[`\xE0\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[\x90a\x0C\xFA\x91`\x01`\x01`\x80\x1B\x03` \x81\x83Q\x16\x92\x01Q\x16\x91a\x13\xD2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x07(WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x07(W\x81\x84\x05\x14\x90\x15\x17\x15a\x07(WV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[Q\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x80\x84\x03\x84\x81\x11a\x07(Wa\x14\0\x81a\x14\x05\x84a\x14\0a\x14\x0B\x96a\x14\x11\x99a EV[a\x1C\xC8V[\x94a EV[\x90a\x13\x99V[\x05\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x07(WV[\x90`\0\x82\x82\x03\x92\x12\x81\x83\x12\x81\x16\x91\x83\x13\x90\x15\x16\x17a\x07(W`\0\x13a\x14\\W`\x01\x90V[`\0\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x02\x02WV[\x90\x81`\x80\x91\x03\x12a\x01\x9DW`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99Wa\x14\xD5\x91``\x91`@R\x80Qa\x14\xA9\x81a\x02JV[\x84Ra\x14\xB7` \x82\x01a\x14aV[` \x85\x01R`@\x81\x01Qa\x14\xCA\x81a\x02JV[`@\x85\x01R\x01a\x14aV[``\x82\x01R\x90V[\x90\x81` \x91\x03\x12a\x01\x9DWQ\x90V[\x90`\x12\x03`\x12\x81\x11a\x07(Wa\x15\x01\x90a\x15\x05V[\x02\x90V[`M\x81\x11a\x07(W`\n\n\x90V[`\x12\x03`\x12\x81\x11a\x07(Wa\x0C\xFA\x90a\x15\x05V[\x90`\x12\x03`\x12\x81\x11a\x07(Wa\x15<\x90a\x15\x05V[\x90\x04\x90V[\x90\x91`\x80\x90a\x15\xCEa\x15\xB4a\x0C\xFA\x97\x96a\x15[\x87\x87a\x13YV[P`\x01`\x01`\x80\x1B\x03\x97\x88\x87Q\x16\x90` \x88\x01\x99a\x15\x80\x8BQ`\x01`\x01`\x80\x1B\x03\x16\x90V[`\xC0\x8A\x01Q\x91\x16\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x03a\x16cWa\xFF\xFFa\x15\xAC`\xA0\x8A\x01Qa\xFF\xFF\x16\x90V[\x16\x91\x86a\x16\xF5V[\x80\x96\x93P\x81a\x15\xC6\x92\x96\x93P\x89a\x13\xD2V[P\x01Q\x15\x15\x90V[\x91\x82\x15a\x16[WP\x90[\x15a\x16*W\x91a\x16\x19a\x16\x1F\x92a\x16\x13a\x16$\x95a\x16\ra\x0FEa\x10+a\x0FE` \x89Q\x99\x01Q\x96Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x97a \x88V[\x92a EV[\x90a\x1C\xC8V[a\x14\x15V[\x90a gV[\x91a\x16\x19a\x16\x1F\x92\x94a\x16\x13a\x16$\x95a\x16\ra\x0FEa\x10+a\x0FE` \x89\x01Q\x98Q\x96Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90P\x90a\x15\xD8V[a\xFF\xFFa\x16t\x88\x8A\x01Qa\xFF\xFF\x16\x90V[a\x15\xACV[a\x16\xCEa\x16\xDB\x93\x94`\x80\x92a\x16\x8E\x85\x82a\x13YV[\x97a\xFF\xFF\x80`\x01`\x01`\x80\x1B\x03\x93\x84\x81Q\x16\x94` \x82\x01Q\x16\x93`\x01\x80`\xA0\x1B\x03\x80`\xC0\x84\x01Q\x16\x91\x16\x14`\0\x14a\x16\xECW`\xA0\x01Q\x16[\x16\x91\x88a\x16\xF5V[\x94\x81\x96\x93P\x85\x92Pa\x13\xD2V[\x93\x01Q\x15a\x16\xE7WP\x92V[\x90P\x92V[\x87\x01Q\x16a\x16\xC6V[\x91\x94\x92\x94`\0a\x17\x10`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16a \xB8V[\x95\x80\x15\x80\x15a\x18IW[PP\x80\x95\x80\x97`\x80\x86\x01\x92a\x17fa\x172\x85Q\x15\x15\x90V[\x93\x84\x15a\x18BW\x87\x94[\x15a\x188Wa\x17a\x87\x95[a\x17[a\x0FE\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x18jV[a\x13\x8CV[\x90\x81\x81\x11a\x18&Wa\x17{` \x91\x85\x93a\x13\x8CV[\x97\x01\x91a\x17\x8F\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11a\x18\x14Wa\x17\xBD\x91a\x17\xB0a\x0FEa\x17\xB6\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x13\x8CV[\x91Q\x15\x15\x90V[\x90\x81\x15a\x18\x0BW\x84\x85\x92[\x15a\x18\x03WP\x92[\x14a\x17\xF1W\x81\x14a\x17\xDFW\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92a\x17\xD0V[\x80\x94\x85\x92a\x17\xC8V[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[a\x17a\x88\x95a\x17GV[\x86\x94a\x17<V[\x90\x91Pa\x18eW\x85\x04\x94\x85\x81\x03\x90\x81\x11a\x07(W\x948\x80a\x17\x1AV[a\x13\xBCV[\x91\x90\x82\x01\x80\x92\x11a\x07(WV[\x90`\x01`\x01`\x80\x1B\x03\x91a\x18\x8E\x82\x84\x83Q\x16a EV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x07(Wa\x0C\xFA\x93` a\x16$\x93\x01Q\x16\x90a EV[`@Q\x90a\x18\xC2\x82a\x07\xAFV[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[\x90`\x01`\x01`\x80\x1B\x03\x80\x92\x16\x82\x03\x91\x82\x11a\x07(WV[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[``\x81\x01\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93\x92\x90`\x01`\x01`@\x1B\x03\x16\x91\x84;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x93\x90\x93\x16`\x04\x84\x01Ra\x01\0\x90\x81\x84`$\x81\x89Z\xFA\x93\x84\x15a\x04\x82W`\0\x94a\x1C9W[Pa\x1A\0a\x19\xF5a\x19\xE9\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[` \x1Cb\xFF\xFF\xFF\x16\x90V[\x90\x86;\x15a\x04\x87W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`\x80\x80\x83`$\x81\x8BZ\xFA\x93\x84\x15a\x04\x82Wa\x0F\x90`\xA0a\x1B5\x93a\x1BB\x97a\t\x93\x97`\0\x91a\x1C\x1CW[Pa\x1AUa\x18\xF7V[\x94a\x1Ab\x83\x8C\x01Q\x15\x15\x90V[\x15a\x1B\xB9W`@\x82a\x1A\x8Ba\x1A\xC5\x93a\x1A\x82` a\x1A\xD5\x97\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x8A\x01RV[a\x1A\xA7a\x1A\x9C``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x8A\x01RV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x89\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x86\x01RV[a\x1A\xE9a\x0FE\x8AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81\x85\x01Ra\x1B.a\x1B!a\x1B\x18` \x8C\x01\x96a\x1B\x0Fa\x0FE\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x82\x01Ra\x1CXV[\x92\x83\x01Qa!FV[`\x01`\x01`\x80\x1B\x03\x16\x8ARV[\x01Qa!FV[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x84;\x15a\x04\x87W` `\x04\x95`@Q\x96\x87\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x92\x83\x15a\x04\x82Wa\x1B\x85\x95`\0\x94a\x1B\x95W[Pa\x1B\x7F\x90a\x11\xF7V[\x90a\x16yV[\x90\x91Pa\x1B\x92\x81\x83a\x148V[\x92V[a\x1B\x7F\x91\x94Pa\x1B\xB2\x90` =\x81\x11a\x04{Wa\x04l\x81\x83a\x07\xE5V[\x93\x90a\x1BuV[\x81a\x1B\xDBa\x1A\xC5\x92a\x1B\xD2``a\x1C\x17\x96\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x89\x01RV[a\x1B\xF7a\x1B\xEC` \x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x89\x01RV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01RQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x1A\xD5V[a\x1C3\x91P\x82=\x84\x11a\x04\xC8Wa\x04\xB9\x81\x83a\x07\xE5V[8a\x1ALV[a\x1CQ\x91\x94P\x82=\x84\x11a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x928a\x19\xD1V[a\x1C`a\x18\xF7V[P`@\x81\x01\x80Q\x90a\x1C|a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90a\x14\xECV[\x90R``\x82\x01a\x1C\x92\x81Q`\xFF\x84Q\x16\x90a\x14\xECV[\x90Ra\x1C\xA9`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90a\x14\xECV[\x90R`\xA0\x81\x01a\x1C\xC3\x81Q`\xFFa\x01 \x85\x01Q\x16\x90a\x14\xECV[\x90R\x90V[a\x1Eua\x0C\xFA\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x1E\x83\x93a\x1C\xFE`\0\x82\x13a \rV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x1D\x1A\x82a \xDEV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x13\x99V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a \x07Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1F\xD3We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[\x15a \x14WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\x02W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\x02Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\x02W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\x02W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a \xE9\x81\x15\x15a \rV[\x80`\x01`\x01`\x80\x1B\x03\x10`\x07\x1B\x81\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\x80\x1B\x81\x10\x15a\x02\x02W`\x01`\x01`\x80\x1B\x03\x16\x90V\xFE\xA2dipfsX\"\x12 \0~o\xDDv\xF2@n\x89\xE9V\xF8\xEBL\\\x92\x11\x94\xC4\x87a\xDF\xA6\x02\xF0\xADQ`h\x17\x9C\x1EdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEANSTRATEGY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\x10\x8FW`\x005`\xE0\x1C\x80c\x14\xA0\x11\xFC\x14a\0\xDCW\x80c\x16\xED\xE0\x16\x14a\0\xD7W\x80c\x19\x05x\x07\x14a\0\xD2W\x80c4\xDB\xC7;\x14a\0\xCDW\x80c9CMZ\x14a\0\xC8W\x80c\x7FI\x125\x14a\0\xC3W\x80c\x80\xAF\x9Dv\x14a\0\xBEW\x80c\xA4G\x89\x19\x14a\0\xB9W\x80c\xE0hx\x7F\x14a\0\xB4W\x80c\xE31\xBA4\x14a\0\xAFW\x80c\xE6\x04{\x19\x14a\0\xAAW\x80c\xECshT\x14a\0\xA5Wc\xF0{\x87\x9E\x03a\x10\x8FWa\r\xD2V[a\x0C\xFDV[a\x0C\x9CV[a\x0B\xC1V[a\nKV[a\x08\xC7V[a\x08$V[a\x06\x96V[a\x05QV[a\x04\xFEV[a\x02jV[a\x01\xA7V[a\x01\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FABI decoding: tuple data too sho`D\x82\x01Ra\x1C\x9D`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xA2W`\x006`\x03\x19\x01\x12a\x01\x9DW` `@Q`\0\x81R\xF3[a\x011V[a\0\xE1V[4a\x01\xA2W`\x006`\x03\x19\x01\x12a\x01\x9DW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02\x02WV[`\0\x80\xFD[`d5\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x02\x02WV[`$5\x90\x81\x15\x15\x82\x03a\x02\x02WV[`D5\x90\x81\x15\x15\x82\x03a\x02\x02WV[`\x845\x90\x81\x15\x15\x82\x03a\x02\x02WV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02\x02WV[`\xC45\x90a\x02h\x82a\x02JV[V[4a\x01\xA2W`\x806`\x03\x19\x01\x12a\x01\x9DWa\x02\x83a\x01\xECV[a\x02\x8Ba\x02\x1DV[`d5a\x02\x97\x81a\x02JV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x85\x16`\x04\x82\x01Ra\x01\0\x92\x90\x83\x81`$\x81\x85Z\xFA\x93\x84\x15a\x04\x82W`\0\x94a\x04\xCFW[PP\x80;\x15a\x04\x87W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x86\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x91`\x80\x83`$\x81\x84Z\xFA\x92\x83\x15a\x04\x82W`\0\x93a\x04\x9FW[P\x85\x15a\x04\x8CWa\x03_`\xFFa\x03Y\x84\x86\x01Q`\xFF\x16\x90V[\x16a\x15\x13V[\x93a\x03\xC5a\x03\x8Da\x03\x83\x8A`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x96`D5\x02a!FV[\x98a\x03\xA8a\x03\x99a\x08\x06V[`\x01`\x01`\x80\x1B\x03\x90\x9B\x16\x8BRV[`\x01\x8A\x86\x01R`\0`@\x8B\x01R`\x01`\x01`@\x1B\x03\x16``\x8A\x01RV[\x86\x15\x15`\x80\x89\x01R\x81;\x15a\x04\x87W\x82`\x04\x92`@Q\x93\x84\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x80\x15a\x04\x82Wa\x04<\x98a\x04+\x97`\xFF\x97a\x04\x18\x95`\0\x94a\x04OW[Pa\x04\x12\x90a\x11\xF7V[\x90a\x15AV[\x94\x15a\x04@WP``\x01Q`\xFF\x16a\x03YV[`@Q\x91\x04\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x01Q`\xFF\x16a\x03YV[a\x03YV[a\x04\x12\x91\x94Pa\x04t\x90\x88=\x8A\x11a\x04{W[a\x04l\x81\x83a\x07\xE5V[\x81\x01\x90a\x14\xDDV[\x93\x90a\x04\x08V[P=a\x04bV[a\x13MV[a\x12\x15V[a\x03_`\xFFa\x04J``\x86\x01Q`\xFF\x16\x90V[a\x04\xC1\x91\x93P`\x80=\x81\x11a\x04\xC8W[a\x04\xB9\x81\x83a\x07\xE5V[\x81\x01\x90a\x14oV[\x918a\x03@V[P=a\x04\xAFV[a\x04\xEF\x92\x94P\x80=\x10a\x04\xF7W[a\x04\xE7\x81\x83a\x07\xE5V[\x81\x01\x90a\x12\x9EV[\x918\x80a\x03\x01V[P=a\x04\xDDV[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DW`\x01`\x01`@\x1B\x03a\x05\x1Fa\x01\xECV[\x16`\0R`\0` R`@`\0 `\x01\x81T\x91\x01T\x90a\x04<`@Q\x92\x83\x92\x83` \x90\x93\x92\x91\x93`@\x81\x01\x94\x81R\x01RV[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DWa\x05ja\x01\xECV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x93a\x06\t\x93`\0\x93a\x06\x19W[PPa\x05\xFEa\x06\x03\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\x11\xF7V[\x90a\x13YV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[a\x06\x03\x92\x93Pa\x05\xFE\x91\x81a\x069\x92\x90=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x92\x91a\x05\xDCV[\x90\x93\x92\x91\x93``\x82R\x80Q\x90\x81``\x84\x01R`\0[\x82\x81\x10a\x06\x80WPP\x90`\x80\x82`\0\x82`@\x95\x85\x01\x01R`\x1F\x80\x19\x91\x01\x16\x82\x01\x01\x94` \x82\x01R\x01RV[\x80` \x80\x92\x84\x01\x01Q`\x80\x82\x87\x01\x01R\x01a\x06UV[4a\x01\xA2W`\x806`\x03\x19\x01\x12a\x01\x9DW`\x045`d5\x90`@Qa\x06\xBA\x81a\x07~V[\x81\x81R` \x81\x01`$5\x81R`@Q\x92` \x84\x01RQ`@\x83\x01R`@\x82Ra\x06\xE2\x82a\x07\xCAV[Qg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x81\x03\x11a\x07(W\x81\x11a\x07(Wa\x07\x1Ca\x07\x16a\x07\x0E\x83a\x04<\x94a EV[`D5a gV[\x84a EV[`@Q\x93\x84\x93\x84a\x06@V[a\x13vV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`#`$\x82\x01R\x7FABI decoding: struct data too sh`D\x82\x01Rb\x1B\xDC\x9D`\xEA\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@RV[`@Q\x90a\x02h\x82a\x07\xAFV[`\x01`\x01`\x80\x1B\x03\x81\x16\x03a\x02\x02WV[4a\x01\xA2W6`\x03\x19\x01`\xE0\x81\x12a\x01\x9DW`\xA0\x13a\x08\xC2Wa\x04<a\x08\xA3`@Qa\x08O\x81a\x07\xAFV[`\x045a\x08[\x81a\x08\x13V[\x81R`$5a\x08i\x81a\x08\x13V[` \x82\x01Ra\x08va\x02,V[`@\x82\x01Ra\x08\x83a\x02\x07V[``\x82\x01Ra\x08\x90a\x02;V[`\x80\x82\x01Ra\x08\x9Da\x02[V[\x90a\x19TV[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[a\x07-V[4a\x01\xA2W`\x806`\x03\x19\x01\x12a\x01\x9DWa\x08\xE0a\x01\xECV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x83\x16`\x04\x82\x01Ra\x01\0\x92\x90\x91\x83\x90\x83\x90`$\x90\x82\x90Z\xFA\x91\x82\x15a\x04\x82Wa\t\xAC\x93`\0\x93a\t\xCBW[PPa\x05\xFEa\x06\x03\x91a\tua\th`D5a!FV[`\x01`\x01`\x80\x1B\x03\x16\x85RV[a\t\x93a\t\x83`d5a!FV[`\x01`\x01`\x80\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[a\t\xB8\x81`$5a\x148V[`@\x80Q\x91\x15\x15\x82R` \x82\x01\x92\x90\x92R\xF3[a\x06\x03\x92\x93Pa\x05\xFE\x91\x81a\t\xEB\x92\x90=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x92\x91a\tQV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rjrray stride`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xA2W`@6`\x03\x19\x01\x12a\x01\x9DWa\nda\x01\xECV[`$\x90\x815\x90`\x01`\x01`@\x1B\x03\x90\x81\x83\x11a\x0BrW6`#\x84\x01\x12\x15a\x0B\x1AW\x82`\x04\x015\x91\x82\x11a\n\xC2W6\x84\x83\x85\x01\x01\x11a\n\xBDWa\x04<\x93a\n\xAB\x93\x01\x90a\x10\xF2V[`@Q\x90\x15\x15\x81R\x90\x81\x90` \x82\x01\x90V[a\t\xF2V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+\x81\x86\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x0ENL/$\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+\x81\x86\x01R\x7FABI decoding: invalid calldata a`D\x82\x01Rj\x1C\x9C\x98^H\x1B\xD9\x99\x9C\xD9]`\xAA\x1B`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"\x81\x86\x01R\x7FABI decoding: invalid tuple offs`D\x82\x01Ra\x19]`\xF2\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DWa\x0B\xDAa\x01\xECV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x93a\x06\t\x93`\0\x93a\x0CuW[PPa\x05\xFEa\x0Cn\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[Q\x90a\x18wV[a\x0Cn\x92\x93Pa\x05\xFE\x91\x81a\x0C\x95\x92\x90=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x92\x91a\x0CLV[4a\x01\xA2W` 6`\x03\x19\x01\x12a\x01\x9DWa\x0C\xB5a\x01\xECV[P` `@Q`\x01\x81R\xF3[``\x90`\x03\x19\x01\x12a\x01\x9DW`\x045`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x02\x02W\x90`$5\x80\x15\x15\x81\x03a\x02\x02W\x90`D5a\x0C\xFA\x81a\x02JV[\x90V[4a\x01\xA2Wa\r\x0B6a\x0C\xC1V[PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x81\x90\x03a\r\xC0W\x80;\x15a\x04\x87W`@Q\x91\x82\x91c\"i|!`\xE2\x1B\x83R`\x01`\x01`@\x1B\x03\x82\x16`\x04\x84\x01R\x82`$a\x01\0\x94\x85\x93Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x93a\r\xA8\x93`\0\x93a\x06\x19WPPa\x05\xFEa\x06\x03\x91`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[`@Q\x91\x82\x91\x82\x91\x90` `@\x84\x01\x93`\x01\x81R\x01RV[`@Qc;\xAAq\x99`\xE2\x1B\x81R`\x04\x90\xFD[4a\x01\xA2Wa\r\xE06a\x0C\xC1V[Pa\r\xE9a\x18\xB5V[P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x84\x16`\x04\x82\x01Ra\x01\0\x93\x90\x84\x81`$\x81\x86Z\xFA\x94\x85\x15a\x04\x82W`\0\x95a\x10pW[PP\x81;\x15a\x04\x87W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81R` \x82\x81\x1Cb\xFF\xFF\xFF\x16`\x04\x83\x01R\x90\x94\x90\x92`\x80\x90\x86\x90\x81\x80`$\x81\x01\x03\x91Z\xFA\x90\x81\x15a\x04\x82Wa\x04<\x95`\0\x92a\x10PW[Pa\x0E\xBFa\x05\xFE\x84`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[Pa\x0E\xC8a\x08\x06V[\x94`\0\x86Ra\x0E\xF1\x85\x87\x01\x94`\0\x86R`\0`@\x89\x01R``\x88\x01\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x15\x80\x15`\x80\x87\x01Ra\x0F\xF0W\x83a\x0Fpa\x0Fj``a\x0F\x90\x95a\x0F\x88a\x0F{a\x0Fv\x88a\x0Fpa\x0Fja\x0F`a\x0F\x9D\x9Fa\x0FE\x90a\x0FQa\x0Fv\x9Fa\x0F@a\x0FE\x91Q`\x01`\x01`\x80\x1B\x03\x16\x90V[a\x18\xE0V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x95\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x9A\x87\x01Q`\xFF\x16\x90V[`\xFF\x16\x90V[\x90a\x15'V[a!FV[`\x01`\x01`\x80\x1B\x03\x16\x8BRV[\x01Q`\xFF\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90RV[`@Q\x91\x82\x91\x82\x91\x90\x91`\x80\x80`\xA0\x83\x01\x94`\x01`\x01`\x80\x1B\x03\x80\x82Q\x16\x85R` \x82\x01Q\x16` \x85\x01R`@\x81\x01Q\x15\x15`@\x85\x01R`\x01`\x01`@\x1B\x03``\x82\x01Q\x16``\x85\x01R\x01Q\x15\x15\x91\x01RV[\x80a\x0Fpa\x0Fj\x86a\x0F\x90\x95a\x0F\x88a\x0F{a\x0Fva\x109a\x0FEa\x10+a\x0FEa\x10K\x9Fa\x0F@\x90a\x0Fv\x9E\x01Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x99Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x97a\x0Fpa\x0Fj``\x87\x01Q`\xFF\x16\x90V[a\x0F\x9DV[a\x10i\x91\x92P`\x80=\x81\x11a\x04\xC8Wa\x04\xB9\x81\x83a\x07\xE5V[\x908a\x0E\x9EV[a\x10\x87\x92\x95P\x80=\x10a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x928\x80a\x0ESV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FContract does not have fallback `D\x82\x01Rtnor receive functions`X\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x903\x83\x90\x03a\r\xC0W\x81`@\x91`\0` \x84Qa\x11:\x81a\x07~V[\x82\x81R\x01R\x81\x01\x03\x12a\x01\x9DW`@Qa\x11\x90\x91a\x11W\x82a\x07~V[\x805\x82R` \x80\x83\x01\x91\x015\x81Ra\x11\x82\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x91Q\x90Q\x91`\x01\x91\x81U\x01UV[\x7FP\xA3\xE9\x9Ea\xA2\xAC\xDE\xB3\x9B\xB5/\x9ECg>\xE5\xE8\x0C\xFF\x1D\xDB\x90I\xFC\xB6y}\x88g\xD1\xCC`\x01`\x01`@\x1B\x03a\x11\xD9a\x05\xFE\x85`\x01`\x01`@\x1B\x03\x16`\0R`\0` R`@`\0 \x90V[\x80Q` \x91\x82\x01Q`@\x80Q\x92\x83R\x92\x82\x01R\x94\x90\x91\x16\x93\xA3`\x01\x90V[\x90`@Qa\x12\x04\x81a\x07~V[` `\x01\x82\x94\x80T\x84R\x01T\x91\x01RV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`%`$\x82\x01R\x7FTarget contract does not contain`D\x82\x01Rd code`\xD8\x1B`d\x82\x01R`\x84\x90\xFD[Q\x90a\x02h\x82a\x08\x13V[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x02WV[Q\x90a\xFF\xFF\x82\x16\x82\x03a\x02\x02WV[Q\x90a\x02h\x82a\x02JV[\x80\x91a\x01\0\x92\x83\x91\x03\x12a\x01\x9DW`@Q\x91\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99Wa\x13E\x91`\xE0\x91`@Ra\x12\xD7\x81a\x12hV[\x84Ra\x12\xE5` \x82\x01a\x12hV[` \x85\x01Ra\x12\xF6`@\x82\x01a\x12hV[`@\x85\x01Ra\x13\x07``\x82\x01a\x12sV[``\x85\x01Ra\x13\x18`\x80\x82\x01a\x12\x84V[`\x80\x85\x01Ra\x13)`\xA0\x82\x01a\x12\x84V[`\xA0\x85\x01Ra\x13:`\xC0\x82\x01a\x12\x93V[`\xC0\x85\x01R\x01a\x12\x93V[`\xE0\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[\x90a\x0C\xFA\x91`\x01`\x01`\x80\x1B\x03` \x81\x83Q\x16\x92\x01Q\x16\x91a\x13\xD2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x07(WV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x07(W\x81\x84\x05\x14\x90\x15\x17\x15a\x07(WV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[Q\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x80\x84\x03\x84\x81\x11a\x07(Wa\x14\0\x81a\x14\x05\x84a\x14\0a\x14\x0B\x96a\x14\x11\x99a EV[a\x1C\xC8V[\x94a EV[\x90a\x13\x99V[\x05\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x07(WV[\x90`\0\x82\x82\x03\x92\x12\x81\x83\x12\x81\x16\x91\x83\x13\x90\x15\x16\x17a\x07(W`\0\x13a\x14\\W`\x01\x90V[`\0\x90V[Q\x90`\xFF\x82\x16\x82\x03a\x02\x02WV[\x90\x81`\x80\x91\x03\x12a\x01\x9DW`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99Wa\x14\xD5\x91``\x91`@R\x80Qa\x14\xA9\x81a\x02JV[\x84Ra\x14\xB7` \x82\x01a\x14aV[` \x85\x01R`@\x81\x01Qa\x14\xCA\x81a\x02JV[`@\x85\x01R\x01a\x14aV[``\x82\x01R\x90V[\x90\x81` \x91\x03\x12a\x01\x9DWQ\x90V[\x90`\x12\x03`\x12\x81\x11a\x07(Wa\x15\x01\x90a\x15\x05V[\x02\x90V[`M\x81\x11a\x07(W`\n\n\x90V[`\x12\x03`\x12\x81\x11a\x07(Wa\x0C\xFA\x90a\x15\x05V[\x90`\x12\x03`\x12\x81\x11a\x07(Wa\x15<\x90a\x15\x05V[\x90\x04\x90V[\x90\x91`\x80\x90a\x15\xCEa\x15\xB4a\x0C\xFA\x97\x96a\x15[\x87\x87a\x13YV[P`\x01`\x01`\x80\x1B\x03\x97\x88\x87Q\x16\x90` \x88\x01\x99a\x15\x80\x8BQ`\x01`\x01`\x80\x1B\x03\x16\x90V[`\xC0\x8A\x01Q\x91\x16\x91`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x16\x03a\x16cWa\xFF\xFFa\x15\xAC`\xA0\x8A\x01Qa\xFF\xFF\x16\x90V[\x16\x91\x86a\x16\xF5V[\x80\x96\x93P\x81a\x15\xC6\x92\x96\x93P\x89a\x13\xD2V[P\x01Q\x15\x15\x90V[\x91\x82\x15a\x16[WP\x90[\x15a\x16*W\x91a\x16\x19a\x16\x1F\x92a\x16\x13a\x16$\x95a\x16\ra\x0FEa\x10+a\x0FE` \x89Q\x99\x01Q\x96Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x97a \x88V[\x92a EV[\x90a\x1C\xC8V[a\x14\x15V[\x90a gV[\x91a\x16\x19a\x16\x1F\x92\x94a\x16\x13a\x16$\x95a\x16\ra\x0FEa\x10+a\x0FE` \x89\x01Q\x98Q\x96Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90P\x90a\x15\xD8V[a\xFF\xFFa\x16t\x88\x8A\x01Qa\xFF\xFF\x16\x90V[a\x15\xACV[a\x16\xCEa\x16\xDB\x93\x94`\x80\x92a\x16\x8E\x85\x82a\x13YV[\x97a\xFF\xFF\x80`\x01`\x01`\x80\x1B\x03\x93\x84\x81Q\x16\x94` \x82\x01Q\x16\x93`\x01\x80`\xA0\x1B\x03\x80`\xC0\x84\x01Q\x16\x91\x16\x14`\0\x14a\x16\xECW`\xA0\x01Q\x16[\x16\x91\x88a\x16\xF5V[\x94\x81\x96\x93P\x85\x92Pa\x13\xD2V[\x93\x01Q\x15a\x16\xE7WP\x92V[\x90P\x92V[\x87\x01Q\x16a\x16\xC6V[\x91\x94\x92\x94`\0a\x17\x10`\x01`\x01`\x80\x1B\x03\x97\x88\x86Q\x16a \xB8V[\x95\x80\x15\x80\x15a\x18IW[PP\x80\x95\x80\x97`\x80\x86\x01\x92a\x17fa\x172\x85Q\x15\x15\x90V[\x93\x84\x15a\x18BW\x87\x94[\x15a\x188Wa\x17a\x87\x95[a\x17[a\x0FE\x8CQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x18jV[a\x13\x8CV[\x90\x81\x81\x11a\x18&Wa\x17{` \x91\x85\x93a\x13\x8CV[\x97\x01\x91a\x17\x8F\x83Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x16\x11a\x18\x14Wa\x17\xBD\x91a\x17\xB0a\x0FEa\x17\xB6\x93Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x90a\x13\x8CV[\x91Q\x15\x15\x90V[\x90\x81\x15a\x18\x0BW\x84\x85\x92[\x15a\x18\x03WP\x92[\x14a\x17\xF1W\x81\x14a\x17\xDFW\x90\x91V[`@Qc\x1F\xB0\xB7\xDD`\xE0\x1B\x81R`\x04\x90\xFD[`@Qc9;xE`\xE1\x1B\x81R`\x04\x90\xFD[\x90P\x92a\x17\xD0V[\x80\x94\x85\x92a\x17\xC8V[`@Qc\x86j\x03+`\xE0\x1B\x81R`\x04\x90\xFD[`@QcvG\x0F\xE7`\xE1\x1B\x81R`\x04\x90\xFD[a\x17a\x88\x95a\x17GV[\x86\x94a\x17<V[\x90\x91Pa\x18eW\x85\x04\x94\x85\x81\x03\x90\x81\x11a\x07(W\x948\x80a\x17\x1AV[a\x13\xBCV[\x91\x90\x82\x01\x80\x92\x11a\x07(WV[\x90`\x01`\x01`\x80\x1B\x03\x91a\x18\x8E\x82\x84\x83Q\x16a EV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\x07(Wa\x0C\xFA\x93` a\x16$\x93\x01Q\x16\x90a EV[`@Q\x90a\x18\xC2\x82a\x07\xAFV[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[\x90`\x01`\x01`\x80\x1B\x03\x80\x92\x16\x82\x03\x91\x82\x11a\x07(WV[`@Q\x90a\x01@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x07\x99W`@R\x81a\x01 `\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x82`\xC0\x82\x01R\x82`\xE0\x82\x01R\x82a\x01\0\x82\x01R\x01RV[``\x81\x01\x80Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93\x92\x90`\x01`\x01`@\x1B\x03\x16\x91\x84;\x15a\x04\x87W`@Qc\"i|!`\xE2\x1B\x81R`\x01`\x01`@\x1B\x03\x93\x90\x93\x16`\x04\x84\x01Ra\x01\0\x90\x81\x84`$\x81\x89Z\xFA\x93\x84\x15a\x04\x82W`\0\x94a\x1C9W[Pa\x1A\0a\x19\xF5a\x19\xE9\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[` \x1Cb\xFF\xFF\xFF\x16\x90V[\x90\x86;\x15a\x04\x87W`@Qc\x17\x91\xD9\x8F`\xE2\x1B\x81Rb\xFF\xFF\xFF\x92\x90\x92\x16`\x04\x83\x01R`\x80\x80\x83`$\x81\x8BZ\xFA\x93\x84\x15a\x04\x82Wa\x0F\x90`\xA0a\x1B5\x93a\x1BB\x97a\t\x93\x97`\0\x91a\x1C\x1CW[Pa\x1AUa\x18\xF7V[\x94a\x1Ab\x83\x8C\x01Q\x15\x15\x90V[\x15a\x1B\xB9W`@\x82a\x1A\x8Ba\x1A\xC5\x93a\x1A\x82` a\x1A\xD5\x97\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x8A\x01RV[a\x1A\xA7a\x1A\x9C``\x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x8A\x01RV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x89\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x86\x01RV[a\x1A\xE9a\x0FE\x8AQ`\x01`\x01`\x80\x1B\x03\x16\x90V[\x81\x85\x01Ra\x1B.a\x1B!a\x1B\x18` \x8C\x01\x96a\x1B\x0Fa\x0FE\x89Q`\x01`\x01`\x80\x1B\x03\x16\x90V[\x86\x82\x01Ra\x1CXV[\x92\x83\x01Qa!FV[`\x01`\x01`\x80\x1B\x03\x16\x8ARV[\x01Qa!FV[Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x84;\x15a\x04\x87W` `\x04\x95`@Q\x96\x87\x80\x92cXq\x0FE`\xE1\x1B\x82RZ\xFA\x92\x83\x15a\x04\x82Wa\x1B\x85\x95`\0\x94a\x1B\x95W[Pa\x1B\x7F\x90a\x11\xF7V[\x90a\x16yV[\x90\x91Pa\x1B\x92\x81\x83a\x148V[\x92V[a\x1B\x7F\x91\x94Pa\x1B\xB2\x90` =\x81\x11a\x04{Wa\x04l\x81\x83a\x07\xE5V[\x93\x90a\x1BuV[\x81a\x1B\xDBa\x1A\xC5\x92a\x1B\xD2``a\x1C\x17\x96\x01Q`\xFF\x16\x90V[`\xFF\x16\x90\x89\x01RV[a\x1B\xF7a\x1B\xEC` \x83\x01Q`\xFF\x16\x90V[`\xFF\x16a\x01 \x89\x01RV[`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x88\x01RQ`\x01`\x01`\xA0\x1B\x03\x16\x90V[a\x1A\xD5V[a\x1C3\x91P\x82=\x84\x11a\x04\xC8Wa\x04\xB9\x81\x83a\x07\xE5V[8a\x1ALV[a\x1CQ\x91\x94P\x82=\x84\x11a\x04\xF7Wa\x04\xE7\x81\x83a\x07\xE5V[\x928a\x19\xD1V[a\x1C`a\x18\xF7V[P`@\x81\x01\x80Q\x90a\x1C|a\x01\0\x84\x01\x92`\xFF\x84Q\x16\x90a\x14\xECV[\x90R``\x82\x01a\x1C\x92\x81Q`\xFF\x84Q\x16\x90a\x14\xECV[\x90Ra\x1C\xA9`\x80\x83\x01\x91`\xFF\x83Q\x91Q\x16\x90a\x14\xECV[\x90R`\xA0\x81\x01a\x1C\xC3\x81Q`\xFFa\x01 \x85\x01Q\x16\x90a\x14\xECV[\x90R\x90V[a\x1Eua\x0C\xFA\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x1E\x83\x93a\x1C\xFE`\0\x82\x13a \rV[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x1D\x1A\x82a \xDEV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x13\x99V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a \x07Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1F\xD3We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[\x15a \x14WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02\x02W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\x02Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02\x02W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02\x02W`\x01a'\x10`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a \xE9\x81\x15\x15a \rV[\x80`\x01`\x01`\x80\x1B\x03\x10`\x07\x1B\x81\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\x80\x1B\x81\x10\x15a\x02\x02W`\x01`\x01`\x80\x1B\x03\x16\x90V\xFE\xA2dipfsX\"\x12 \0~o\xDDv\xF2@n\x89\xE9V\xF8\xEBL\\\x92\x11\x94\xC4\x87a\xDF\xA6\x02\xF0\xADQ`h\x17\x9C\x1EdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static GEOMETRICMEANSTRATEGY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GeometricMeanStrategy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeometricMeanStrategy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeometricMeanStrategy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeometricMeanStrategy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeometricMeanStrategy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeometricMeanStrategy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeometricMeanStrategy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GEOMETRICMEANSTRATEGY_ABI.clone(),
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
                GEOMETRICMEANSTRATEGY_ABI.clone(),
                GEOMETRICMEANSTRATEGY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `MINIMUM_INVARIANT_DELTA` (0x14a011fc) function
        pub fn minimum_invariant_delta(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([20, 160, 17, 252], ())
                .expect("method not found (this should never happen)")
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
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
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
        ///Calls the contract's `getStrategyData` (0x7f491235) function
        pub fn get_strategy_data(
            &self,
            asset_weight_wad: ::ethers::core::types::U256,
            quote_weight_wad: ::ethers::core::types::U256,
            price_wad: ::ethers::core::types::U256,
            asset_in_wad: ::ethers::core::types::U256,
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
                    [127, 73, 18, 53],
                    (asset_weight_wad, quote_weight_wad, price_wad, asset_in_wad),
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
            GeometricMeanStrategyEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GeometricMeanStrategy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `GeometricMeanStrategy_NotPortfolio` with signature `GeometricMeanStrategy_NotPortfolio()` and selector `0xeea9c664`
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
        name = "GeometricMeanStrategy_NotPortfolio",
        abi = "GeometricMeanStrategy_NotPortfolio()"
    )]
    pub struct GeometricMeanStrategy_NotPortfolio;
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
    pub enum GeometricMeanStrategyErrors {
        GeometricMeanStrategy_NotPortfolio(GeometricMeanStrategy_NotPortfolio),
        SwapLib_OutputExceedsReserves(SwapLib_OutputExceedsReserves),
        SwapLib_ProtocolFeeTooHigh(SwapLib_ProtocolFeeTooHigh),
        SwapLib_ZeroXAdjustment(SwapLib_ZeroXAdjustment),
        SwapLib_ZeroYAdjustment(SwapLib_ZeroYAdjustment),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanStrategyErrors {
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
                = <GeometricMeanStrategy_NotPortfolio as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GeometricMeanStrategy_NotPortfolio(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanStrategyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::GeometricMeanStrategy_NotPortfolio(element) => {
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
    impl ::ethers::contract::ContractRevert for GeometricMeanStrategyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <GeometricMeanStrategy_NotPortfolio as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GeometricMeanStrategyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GeometricMeanStrategy_NotPortfolio(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<::std::string::String> for GeometricMeanStrategyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<GeometricMeanStrategy_NotPortfolio>
    for GeometricMeanStrategyErrors {
        fn from(value: GeometricMeanStrategy_NotPortfolio) -> Self {
            Self::GeometricMeanStrategy_NotPortfolio(value)
        }
    }
    impl ::core::convert::From<SwapLib_OutputExceedsReserves>
    for GeometricMeanStrategyErrors {
        fn from(value: SwapLib_OutputExceedsReserves) -> Self {
            Self::SwapLib_OutputExceedsReserves(value)
        }
    }
    impl ::core::convert::From<SwapLib_ProtocolFeeTooHigh>
    for GeometricMeanStrategyErrors {
        fn from(value: SwapLib_ProtocolFeeTooHigh) -> Self {
            Self::SwapLib_ProtocolFeeTooHigh(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroXAdjustment> for GeometricMeanStrategyErrors {
        fn from(value: SwapLib_ZeroXAdjustment) -> Self {
            Self::SwapLib_ZeroXAdjustment(value)
        }
    }
    impl ::core::convert::From<SwapLib_ZeroYAdjustment> for GeometricMeanStrategyErrors {
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
        abi = "AfterCreate(address,uint64,uint256,uint256)"
    )]
    pub struct AfterCreateFilter {
        #[ethevent(indexed)]
        pub portfolio: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_id: u64,
        pub asset_weight_wad: ::ethers::core::types::U256,
        pub quote_weight_wad: ::ethers::core::types::U256,
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
        pub portfolio: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum GeometricMeanStrategyEvents {
        AfterCreateFilter(AfterCreateFilter),
        GenesisFilter(GenesisFilter),
    }
    impl ::ethers::contract::EthLogDecode for GeometricMeanStrategyEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AfterCreateFilter::decode_log(log) {
                return Ok(GeometricMeanStrategyEvents::AfterCreateFilter(decoded));
            }
            if let Ok(decoded) = GenesisFilter::decode_log(log) {
                return Ok(GeometricMeanStrategyEvents::GenesisFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GeometricMeanStrategyEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AfterCreateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GenesisFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AfterCreateFilter> for GeometricMeanStrategyEvents {
        fn from(value: AfterCreateFilter) -> Self {
            Self::AfterCreateFilter(value)
        }
    }
    impl ::core::convert::From<GenesisFilter> for GeometricMeanStrategyEvents {
        fn from(value: GenesisFilter) -> Self {
            Self::GenesisFilter(value)
        }
    }
    ///Container type for all input parameters for the `MINIMUM_INVARIANT_DELTA` function with signature `MINIMUM_INVARIANT_DELTA()` and selector `0x14a011fc`
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
    #[ethcall(name = "MINIMUM_INVARIANT_DELTA", abi = "MINIMUM_INVARIANT_DELTA()")]
    pub struct MinimumInvariantDeltaCall;
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
    ///Container type for all input parameters for the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,uint256)` and selector `0x7f491235`
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
        abi = "getStrategyData(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetStrategyDataCall {
        pub asset_weight_wad: ::ethers::core::types::U256,
        pub quote_weight_wad: ::ethers::core::types::U256,
        pub price_wad: ::ethers::core::types::U256,
        pub asset_in_wad: ::ethers::core::types::U256,
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
    pub enum GeometricMeanStrategyCalls {
        MinimumInvariantDelta(MinimumInvariantDeltaCall),
        AfterCreate(AfterCreateCall),
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
    impl ::ethers::core::abi::AbiDecode for GeometricMeanStrategyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <MinimumInvariantDeltaCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MinimumInvariantDelta(decoded));
            }
            if let Ok(decoded)
                = <AfterCreateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AfterCreate(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanStrategyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::MinimumInvariantDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AfterCreate(element) => {
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
    impl ::core::fmt::Display for GeometricMeanStrategyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::MinimumInvariantDelta(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AfterCreate(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<MinimumInvariantDeltaCall>
    for GeometricMeanStrategyCalls {
        fn from(value: MinimumInvariantDeltaCall) -> Self {
            Self::MinimumInvariantDelta(value)
        }
    }
    impl ::core::convert::From<AfterCreateCall> for GeometricMeanStrategyCalls {
        fn from(value: AfterCreateCall) -> Self {
            Self::AfterCreate(value)
        }
    }
    impl ::core::convert::From<BeforeSwapCall> for GeometricMeanStrategyCalls {
        fn from(value: BeforeSwapCall) -> Self {
            Self::BeforeSwap(value)
        }
    }
    impl ::core::convert::From<ConfigsCall> for GeometricMeanStrategyCalls {
        fn from(value: ConfigsCall) -> Self {
            Self::Configs(value)
        }
    }
    impl ::core::convert::From<GetAmountOutCall> for GeometricMeanStrategyCalls {
        fn from(value: GetAmountOutCall) -> Self {
            Self::GetAmountOut(value)
        }
    }
    impl ::core::convert::From<GetInvariantCall> for GeometricMeanStrategyCalls {
        fn from(value: GetInvariantCall) -> Self {
            Self::GetInvariant(value)
        }
    }
    impl ::core::convert::From<GetMaxOrderCall> for GeometricMeanStrategyCalls {
        fn from(value: GetMaxOrderCall) -> Self {
            Self::GetMaxOrder(value)
        }
    }
    impl ::core::convert::From<GetSpotPriceCall> for GeometricMeanStrategyCalls {
        fn from(value: GetSpotPriceCall) -> Self {
            Self::GetSpotPrice(value)
        }
    }
    impl ::core::convert::From<GetStrategyDataCall> for GeometricMeanStrategyCalls {
        fn from(value: GetStrategyDataCall) -> Self {
            Self::GetStrategyData(value)
        }
    }
    impl ::core::convert::From<PortfolioCall> for GeometricMeanStrategyCalls {
        fn from(value: PortfolioCall) -> Self {
            Self::Portfolio(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for GeometricMeanStrategyCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<ValidatePoolCall> for GeometricMeanStrategyCalls {
        fn from(value: ValidatePoolCall) -> Self {
            Self::ValidatePool(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for GeometricMeanStrategyCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    ///Container type for all return fields from the `MINIMUM_INVARIANT_DELTA` function with signature `MINIMUM_INVARIANT_DELTA()` and selector `0x14a011fc`
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
    pub struct MinimumInvariantDeltaReturn(pub ::ethers::core::types::I256);
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
        pub asset_weight_wad: ::ethers::core::types::U256,
        pub quote_weight_wad: ::ethers::core::types::U256,
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
    ///Container type for all return fields from the `getStrategyData` function with signature `getStrategyData(uint256,uint256,uint256,uint256)` and selector `0x7f491235`
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
