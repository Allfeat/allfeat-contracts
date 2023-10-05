// Copyright (c) 2022-2023 Allfeat labs
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use ink::prelude::string::String;
use openbrush::contracts::traits::errors::{
    AccessControlError, OwnableError, PausableError, ReentrancyGuardError,
};

/// The AFT34 error type. Contract will throw one of this errors.
/// This extend the PSP34 error enum in case some would be added for the AFT34 in the future.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AFT34Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if owner approves self
    SelfApprove,
    /// Returned if the caller doesn't have allowance for transferring.
    NotApproved,
    /// Returned if the owner already own the token.
    TokenExists,
    /// Returned if  the token doesn't exist
    TokenNotExists,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for AFT34Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                AFT34Error::Custom(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::NewOwnerIsNotSet => {
                AFT34Error::Custom(String::from("O::NewOwnerIsNotSet"))
            }
        }
    }
}

impl From<AccessControlError> for AFT34Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => AFT34Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => {
                AFT34Error::Custom(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                AFT34Error::Custom(String::from("AC::InvalidCaller"))
            }
        }
    }
}

impl From<PausableError> for AFT34Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => AFT34Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => AFT34Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for AFT34Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => {
                AFT34Error::Custom(String::from("RG::ReentrantCall"))
            }
        }
    }
}

/// The AFT34Receiver error types.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AFT34ReceiverError {
    /// Returned if transfer is rejected.
    TransferRejected(String),
}

impl From<AFT34ReceiverError> for AFT34Error {
    fn from(error: AFT34ReceiverError) -> Self {
        match error {
            AFT34ReceiverError::TransferRejected(message) => {
                AFT34Error::SafeTransferCheckFailed(message)
            }
        }
    }
}
