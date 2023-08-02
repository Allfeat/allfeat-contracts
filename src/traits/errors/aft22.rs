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

use openbrush::contracts::traits::access_control::AccessControlError;
use openbrush::contracts::traits::errors::ReentrancyGuardError;
use openbrush::contracts::traits::flashloan::FlashLenderError;
use openbrush::contracts::traits::ownable::OwnableError;
use openbrush::contracts::traits::pausable::PausableError;
use openbrush::traits::String;

/// The AFT22 error type. Contract will throw one of this errors.
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AFT22Error {
    /// Custom error type for cases if writer of traits added own restrictions
    Custom(String),
    /// Returned if not enough balance to fulfill a request is available.
    InsufficientBalance,
    /// Returned if not enough allowance to fulfill a request is available.
    InsufficientAllowance,
    /// Returned if recipient's address is zero.
    ZeroRecipientAddress,
    /// Returned if sender's address is zero.
    ZeroSenderAddress,
    /// Returned if safe transfer check fails
    SafeTransferCheckFailed(String),
}

impl From<OwnableError> for AFT22Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => {
                AFT22Error::Custom(String::from("O::CallerIsNotOwner"))
            }
            OwnableError::NewOwnerIsZero => AFT22Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for AFT22Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => AFT22Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => {
                AFT22Error::Custom(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                AFT22Error::Custom(String::from("AC::InvalidCaller"))
            }
        }
    }
}

impl From<PausableError> for AFT22Error {
    fn from(pausable: PausableError) -> Self {
        match pausable {
            PausableError::Paused => AFT22Error::Custom(String::from("P::Paused")),
            PausableError::NotPaused => AFT22Error::Custom(String::from("P::NotPaused")),
        }
    }
}

impl From<ReentrancyGuardError> for AFT22Error {
    fn from(guard: ReentrancyGuardError) -> Self {
        match guard {
            ReentrancyGuardError::ReentrantCall => {
                AFT22Error::Custom(String::from("RG::ReentrantCall"))
            }
        }
    }
}

impl From<AFT22Error> for FlashLenderError {
    fn from(error: AFT22Error) -> Self {
        match error {
            AFT22Error::Custom(message) => FlashLenderError::Custom(message),
            AFT22Error::InsufficientBalance => {
                FlashLenderError::Custom(String::from("PSP22: Insufficient Balance"))
            }
            AFT22Error::InsufficientAllowance => {
                FlashLenderError::Custom(String::from("PSP22: Insufficient Allowance"))
            }
            AFT22Error::ZeroRecipientAddress => {
                FlashLenderError::Custom(String::from("PSP22: Zero Recipient Address"))
            }
            AFT22Error::ZeroSenderAddress => {
                FlashLenderError::Custom(String::from("PSP22: Zero Sender Address"))
            }
            AFT22Error::SafeTransferCheckFailed(message) => FlashLenderError::Custom(message),
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AFT22ReceiverError {
    TransferRejected(String),
}

impl From<AFT22ReceiverError> for AFT22Error {
    fn from(error: AFT22ReceiverError) -> Self {
        match error {
            AFT22ReceiverError::TransferRejected(message) => {
                AFT22Error::SafeTransferCheckFailed(message)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum AFT22TokenTimelockError {
    AFT22Error(AFT22Error),
    /// Returned if the owner wants to withdraw the tokens before the release time
    CurrentTimeIsBeforeReleaseTime,
    /// Returned if there are no tokens to be released
    NoTokensToRelease,
    /// Returned if the timestamp provided is before the current time
    ReleaseTimeIsBeforeCurrentTime,
    /// Returned if the token is not initialized
    TokenZeroAddress,
    /// Returned if the beneficiary is not initialized
    BeneficiaryZeroAddress,
}

impl From<AFT22Error> for AFT22TokenTimelockError {
    fn from(error: AFT22Error) -> Self {
        match error {
            AFT22Error::Custom(message) => {
                AFT22TokenTimelockError::AFT22Error(AFT22Error::Custom(message))
            }
            AFT22Error::InsufficientBalance => {
                AFT22TokenTimelockError::AFT22Error(AFT22Error::InsufficientBalance)
            }
            AFT22Error::InsufficientAllowance => {
                AFT22TokenTimelockError::AFT22Error(AFT22Error::InsufficientAllowance)
            }
            AFT22Error::ZeroRecipientAddress => {
                AFT22TokenTimelockError::AFT22Error(AFT22Error::ZeroRecipientAddress)
            }
            AFT22Error::ZeroSenderAddress => {
                AFT22TokenTimelockError::AFT22Error(AFT22Error::ZeroSenderAddress)
            }
            AFT22Error::SafeTransferCheckFailed(message) => {
                AFT22TokenTimelockError::AFT22Error(AFT22Error::SafeTransferCheckFailed(message))
            }
        }
    }
}

impl From<OwnableError> for AFT22TokenTimelockError {
    fn from(ownable: OwnableError) -> Self {
        AFT22TokenTimelockError::AFT22Error(ownable.into())
    }
}

impl From<AccessControlError> for AFT22TokenTimelockError {
    fn from(access: AccessControlError) -> Self {
        AFT22TokenTimelockError::AFT22Error(access.into())
    }
}

impl From<PausableError> for AFT22TokenTimelockError {
    fn from(pausable: PausableError) -> Self {
        AFT22TokenTimelockError::AFT22Error(pausable.into())
    }
}

impl From<ReentrancyGuardError> for AFT22TokenTimelockError {
    fn from(guard: ReentrancyGuardError) -> Self {
        AFT22TokenTimelockError::AFT22Error(guard.into())
    }
}
