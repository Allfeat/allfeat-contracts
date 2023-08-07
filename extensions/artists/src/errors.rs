use scale::{Decode, Encode};

#[cfg(feature = "substrate")]
use sp_runtime::{DispatchError, ModuleError};

#[cfg(feature = "substrate")]
#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ArtistsError {
    /// Success
    Success = 0,

    // General errors:
    // ===============
    /// The caller doesn't have enough funds for the deposit
    NotEnoughFunds = 1,
    /// The given string is longer than `T::NameMaxLength`.
    NameTooLong = 2,

    // Candidate related errors:
    // =========================
    /// The account is already in the candidate list
    AlreadyACandidate = 3,
    /// The wanted candidate is not found in the Candidates Storage
    CandidateNotFound = 4,
    /// The caller isn't in the candidate list.
    NotACandidate = 5,

    // Artist related errors:
    // ======================
    /// This account already is a certificated artist account.
    AlreadyAnArtist = 6,
    /// The caller isn't a verified artist.
    NotAnArtist = 7,
    /// The wanted artist is not found in the Artists Storage
    ArtistNotFound = 8,

    /// Unknown error
    UnknownError = 99,
}

#[cfg(feature = "substrate")]
// TODO: macro to make the implement not verbose that much ?
impl TryFrom<DispatchError> for ArtistsError {
    type Error = DispatchError;

    fn try_from(value: DispatchError) -> Result<Self, Self::Error> {
        let error_text = match value {
            DispatchError::Module(ModuleError { message, .. }) => message,
            _ => Some("No module error Info"),
        };
        return match error_text {
            Some("NotEnoughFunds") => Ok(Self::NotEnoughFunds),
            Some("NameTooLong") => Ok(Self::NameTooLong),
            Some("AlreadyACandidate") => Ok(Self::AlreadyACandidate),
            Some("CandidateNotFound") => Ok(Self::CandidateNotFound),
            Some("NotACandidate") => Ok(Self::NotACandidate),
            Some("AlreadyAnArtist") => Ok(Self::AlreadyAnArtist),
            Some("NotAnArtist") => Ok(Self::NotAnArtist),
            Some("ArtistNotFound") => Ok(Self::ArtistNotFound),
            _ => Ok(ArtistsError::UnknownError),
        };
    }
}
