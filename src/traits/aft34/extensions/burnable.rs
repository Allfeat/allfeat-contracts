/// Extension of [`AFT34`] that allows token holders to destroy their tokens
use crate::traits::aft34::AFT34Error;
use crate::traits::aft34::Id;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type AFT34BurnableRef = dyn AFT34Burnable;

#[openbrush::trait_definition]
pub trait AFT34Burnable {
    /// Destroys token with id equal to `id` from `account`
    ///
    /// Caller must be approved to transfer tokens from `account`
    /// or to transfer token with `id`
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), AFT34Error>;
}
