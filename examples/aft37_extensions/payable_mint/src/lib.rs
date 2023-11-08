#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(Ownable)]
#[allfeat_contracts::implementation(AFT37, AFT37PayableMint, AFT37Metadata, AFT37Enumerable)]
#[allfeat_contracts::contract]
pub mod my_aft37_payable_mint {
    use ink::prelude::string::String as PreludeString;
    use openbrush::traits::{Storage, String};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft37: aft37::Data,
        #[storage_field]
        payable_mint: aft37::extensions::payable_mint::Data,
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
        pub fn new(name: String, symbol: String, base_uri: String) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            ownable::InternalImpl::_init_with_owner(&mut instance, caller);

            let account_id = Self::env().account_id();
            let collection_id = Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec());

            metadata::InternalImpl::_set_attribute(
                &mut instance,
                &collection_id,
                &String::from("name"),
                &name,
            )
            .unwrap();

            metadata::InternalImpl::_set_attribute(
                &mut instance,
                &collection_id,
                &String::from("symbol"),
                &symbol,
            )
            .unwrap();

            metadata::InternalImpl::_set_attribute(
                &mut instance,
                &collection_id,
                &String::from("baseUri"),
                &base_uri,
            )
            .unwrap();

            instance
        }
    }

    #[cfg(test)]
    pub mod tests {
        use allfeat_contracts::aft37::{
            aft37_external::AFT37,
            extensions::payable_mint::aft37payablemint_external::AFT37PayableMint, AFT37Error::*,
        };
        use ink::env::test;

        #[rustfmt::skip]
        use super::*;
        #[cfg(all(test, feature = "e2e-tests"))]
        #[rustfmt::skip]
        use ink_e2e::build_message;

        #[cfg(all(test, feature = "e2e-tests"))]
        use test_helpers::{address_of, balance_of_37};

        #[cfg(all(test, feature = "e2e-tests"))]
        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        const BASE_URI: &str = "ipfs://myIpfsUri/";
        const PRICE: u128 = 100_000_000_000_000_000;

        #[ink::test]
        fn mint_multiple_works() {
            let mut aft37 = Contract::default();
            let id = Id::U32(0);

            let accounts = test::default_accounts::<Environment>();
            ownable::InternalImpl::_init_with_owner(&mut aft37, accounts.bob);
            set_sender(accounts.bob);
            let num_of_mints: u64 = 5;

            aft37::extensions::payable_mint::AFT37PayableMintImpl::set_price(
                &mut aft37,
                id.clone(),
                PRICE,
            )
            .unwrap();
            aft37::extensions::payable_mint::AFT37PayableMintImpl::set_max_supply(
                &mut aft37,
                id.clone(),
                10,
            )
            .unwrap();

            assert_eq!(AFT37Impl::total_supply(&aft37, Some(id.clone())), 0);
            test::set_value_transferred::<ink::env::DefaultEnvironment>(
                PRICE * num_of_mints as u128,
            );

            assert!(aft37::extensions::payable_mint::AFT37PayableMintImpl::mint(
                &mut aft37,
                accounts.bob,
                vec![(id.clone(), num_of_mints.into())]
            )
            .is_ok());
            assert_eq!(
                AFT37Impl::total_supply(&aft37, Some(id.clone())),
                num_of_mints as u128
            );
            assert_eq!(
                AFT37Impl::balance_of(&aft37, accounts.bob, Some(id.clone())),
                5
            );
            assert_eq!(
                AFT37EnumerableImpl::owners_token_by_index(&aft37, accounts.bob, 0),
                Some(id)
            );
            assert_eq!(
                AFT37EnumerableImpl::owners_token_by_index(&aft37, accounts.bob, 5),
                None
            );
        }

        fn set_sender(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }

        #[cfg(all(test, feature = "e2e-tests"))]
        #[ink_e2e::test]
        async fn mint_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(
                String::from("Allfea37"),
                String::from("AFT37"),
                String::from(BASE_URI),
            );
            let id = Id::U32(0);

            let address = client
                .instantiate(
                    "my_aft37_payable_mint",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let set_max_supply_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_max_supply(id.clone(), 10));

                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set_max_supply failed")
            }
            .return_value();

            assert_eq!(set_max_supply_1, Ok(()));

            let set_price_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_price(id.clone(), 1));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set_price failed")
            }
            .return_value();

            assert_eq!(set_price_1, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, Some(id.clone())), 0);
            assert_eq!(balance_of_37!(client, address, bob, Some(id.clone())), 0);

            let mint_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), vec![(id.clone(), 1)]));
                client
                    .call(&ink_e2e::alice(), _msg, 1, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_1, Ok(()));

            let mint_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), vec![(id.clone(), 2)]));
                client
                    .call(&ink_e2e::alice(), _msg, 2, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_2, Ok(()));

            assert_eq!(balance_of_37!(client, address, alice, Some(id.clone())), 1);
            assert_eq!(balance_of_37!(client, address, bob, Some(id.clone())), 2);

            Ok(())
        }
    }
}
