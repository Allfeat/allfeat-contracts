#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT34, AFT34Burnable)]
#[allfeat_contracts::contract]
pub mod my_aft34_burnable {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft34: aft34::Data,
    }

    impl Contract {
        /// The constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut instance = Self::default();

            aft34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(0u8))
                .expect("Should mint token with id 0");
            aft34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(1u8))
                .expect("Should mint token with id 1");
            aft34::Internal::_mint_to(&mut instance, Self::env().caller(), Id::U8(2u8))
                .expect("Should mint token with id 2");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use allfeat_contracts::aft34::{
            aft34_external::AFT34, extensions::burnable::aft34burnable_external::AFT34Burnable,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use test_helpers::{address_of, balance_of};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn burn_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft34_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 3);

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), Id::U8(0u8)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert_eq!(result.return_value(), Ok(()));
            assert_eq!(balance_of!(client, address, alice), 2);

            Ok(())
        }

        #[ink_e2e::test]
        async fn burn_from_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft34_burnable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert_eq!(balance_of!(client, address, alice), 3);

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.burn(address_of!(alice), Id::U8(0u8)));
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("call failed")
            };

            assert_eq!(result.return_value(), Ok(()));
            assert_eq!(balance_of!(client, address, alice), 2);

            Ok(())
        }
    }
}
