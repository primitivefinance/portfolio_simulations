pub use normal_strategy_lib::*;
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
pub mod normal_strategy_lib {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::std::collections::BTreeMap::new(),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static NORMALSTRATEGYLIB_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4`\x1BW`@Q`\x94\x90\x81a\0i\x8290\x81PP\xF3[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7FEther sent to non-payable functi`D\x82\x01Ra7\xB7`\xF1\x1B`d\x82\x01R`\x84\x90\xFD\xFE`\x80`@\x81\x90RbF\x1B\xCD`\xE5\x1B\x81R` `\x84\x90\x81R`5`\xA4R\x7FContract does not have fallback `\xC4Rtnor receive functions`X\x1B`\xE4R\x90\xFD\xFE\xA2dipfsX\"\x12 \xDE5\x8Ex\xE8}8vc\x1B\xC0\xF6U\xF7\xA5no1\xD5\x958GY\xB4/a\x90\xD8\xA8\x99\"\x1BdsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static NORMALSTRATEGYLIB_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x90RbF\x1B\xCD`\xE5\x1B\x81R` `\x84\x90\x81R`5`\xA4R\x7FContract does not have fallback `\xC4Rtnor receive functions`X\x1B`\xE4R\x90\xFD\xFE\xA2dipfsX\"\x12 \xDE5\x8Ex\xE8}8vc\x1B\xC0\xF6U\xF7\xA5no1\xD5\x958GY\xB4/a\x90\xD8\xA8\x99\"\x1BdsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static NORMALSTRATEGYLIB_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct NormalStrategyLib<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NormalStrategyLib<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NormalStrategyLib<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NormalStrategyLib<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NormalStrategyLib<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NormalStrategyLib))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NormalStrategyLib<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    NORMALSTRATEGYLIB_ABI.clone(),
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
                NORMALSTRATEGYLIB_ABI.clone(),
                NORMALSTRATEGYLIB_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for NormalStrategyLib<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
}
