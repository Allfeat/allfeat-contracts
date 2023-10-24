#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable)]
#[allfeat_contracts::implementation(AFT34, AFT34PayableMint, AFT34Metadata, AFT34Enumerable)]
#[allfeat_contracts::contract]
pub mod my_aft34_payable_mint {
    use ink::prelude::string::String as PreludeString;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft34: aft34::Data,
        #[storage_field]
        payable_mint: aft34::extensions::payable_mint::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        enumerable: enumerable::Data,
    }

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new(
            name: String,
            symbol: String,
            base_uri: String,
            max_supply: u64,
            price_per_mint: Balance,
        ) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            ownable::InternalImpl::_init_with_owner(&mut instance, caller);
            let collection_id = aft34::AFT34Impl::collection_id(&instance);
            metadata::InternalImpl::_set_attribute(
                &mut instance,
                collection_id.clone(),
                String::from("name"),
                name,
            );
            metadata::InternalImpl::_set_attribute(
                &mut instance,
                collection_id.clone(),
                String::from("symbol"),
                symbol,
            );
            metadata::InternalImpl::_set_attribute(
                &mut instance,
                collection_id,
                String::from("baseUri"),
                base_uri,
            );
            instance.payable_mint.max_supply = max_supply;
            instance.payable_mint.price_per_mint = price_per_mint;
            instance.payable_mint.last_token_id = 0;
            instance
        }
    }

    #[cfg(test)]
    pub mod tests {
        use allfeat_contracts::aft34::{
            aft34_external::AFT34,
            extensions::payable_mint::aft34payablemint_external::AFT34PayableMint, AFT34Error::*,
        };
        use ink::env::test;

        #[rustfmt::skip]
        use super::*;
        #[cfg(all(test, feature = "e2e-tests"))]
        #[rustfmt::skip]
        use ink_e2e::build_message;

        #[cfg(all(test, feature = "e2e-tests"))]
        use test_helpers::{address_of, balance_of};

        #[cfg(all(test, feature = "e2e-tests"))]
        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const PRICE: u128 = 100_000_000_000_000_000;
        #[ink::test]
        fn mint_multiple_works() {
            let mut aft34 = Contract::new(
                String::from("Allfeat34"),
                String::from("AFT34"),
                String::from(BASE_URI),
                10,
                PRICE,
            );
            let accounts = test::default_accounts::<Environment>();
            set_sender(accounts.bob);
            let num_of_mints: u64 = 5;

            assert_eq!(AFT34Impl::total_supply(&aft34), 0);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );
            assert!(aft34::extensions::payable_mint::AFT34PayableMintImpl::mint(
                &mut aft34,
                accounts.bob,
                num_of_mints
            )
            .is_ok());
            assert_eq!(AFT34Impl::total_supply(&aft34), num_of_mints as u128);
            assert_eq!(AFT34Impl::balance_of(&aft34, accounts.bob), 5);
            assert_eq!(
                AFT34EnumerableImpl::owners_token_by_index(&aft34, accounts.bob, 0),
                Ok(Id::U64(1))
            );
            assert_eq!(
                AFT34EnumerableImpl::owners_token_by_index(&aft34, accounts.bob, 1),
                Ok(Id::U64(2))
            );
            assert_eq!(
                AFT34EnumerableImpl::owners_token_by_index(&aft34, accounts.bob, 2),
                Ok(Id::U64(3))
            );
            assert_eq!(
                AFT34EnumerableImpl::owners_token_by_index(&aft34, accounts.bob, 3),
                Ok(Id::U64(4))
            );
            assert_eq!(
                AFT34EnumerableImpl::owners_token_by_index(&aft34, accounts.bob, 4),
                Ok(Id::U64(5))
            );
            assert_eq!(
                AFT34EnumerableImpl::owners_token_by_index(&aft34, accounts.bob, 5),
                Err(TokenNotExists)
            );
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }

        #[cfg(all(test, feature = "e2e-tests"))]
        #[ink_e2e::test]
        async fn mint_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            const MAX_SUPPLY: u64 = 10;
            let constructor = ContractRef::new(
                String::from("Allfeat34"),
                String::from("AFT34"),
                String::from(BASE_URI),
                MAX_SUPPLY,
                1,
            );
            let address = client
                .instantiate(
                    "my_aft34_payable_mint",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 0);
            assert_eq!(balance_of!(client, address, bob), 0);

            let mint_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), 1));
                client
                    .call(&ink_e2e::alice(), _msg, 1, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), 2));
                client
                    .call(&ink_e2e::alice(), _msg, 2, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_2, Ok(()));

            assert_eq!(balance_of!(client, address, alice), 1);
            assert_eq!(balance_of!(client, address, bob), 2);

            Ok(())
        }
    }
}
