#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT22, AFT22Wrapper)]
#[allfeat_contracts::contract]
pub mod my_aft22_wrapper {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft22: aft22::Data,
        #[storage_field]
        wrapper: wrapper::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(token_address: AccountId) -> Self {
            let mut instance = Self::default();

            Internal::_init(&mut instance, token_address);

            instance
        }

        /// Exposes the `_recover` function for message caller
        #[ink(message)]
        pub fn recover(&mut self) -> Result<Balance, AFT22Error> {
            Internal::_recover(self, Self::env().caller())
        }
    }
}
