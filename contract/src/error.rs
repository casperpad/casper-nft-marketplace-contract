//! Error handling on the casper platform.
use casper_types::ApiError;

/// Errors which can be returned by the library.
///
/// When an `Error` is returned from a smart contract, it is converted to an [`ApiError::User`].
///
/// Where a smart contract consuming this library needs to define further error variants, it can
/// return those via the [`Error::User`] variant or equivalently via the [`ApiError::User`]
/// variant.
///
/// Such a user error should be in the range `[0..(u16::MAX - 4)]` (i.e. [0, 65532]) to avoid
/// conflicting with the other `Error` variants.
#[repr(u16)]
#[derive(Debug)]
pub enum Error {
    InsufficientBalance = 0,
    InsufficientAllowance = 1,

    // User Error
    PermissionDenied = 41,
    NotApproved = 42,
    NotOwner = 43,
    NotOrderMaker = 44,
    NotValidAmount = 45,
    BidExist = 46,
    OrderExist = 47,
    OrderNotExist = 48,
    // Contract Error
    InvalidContext = 90,
    KeyAlreadyExists = 91,
    KeyMismatch = 92,
    Overflow = 93,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        match error {
            Error::PermissionDenied => ApiError::PermissionDenied,
            Error::InsufficientBalance
            | Error::InsufficientAllowance
            | Error::NotApproved
            | Error::NotOwner
            | Error::NotOrderMaker
            | Error::NotValidAmount
            | Error::BidExist
            | Error::InvalidContext
            | Error::KeyAlreadyExists
            | Error::KeyMismatch
            | Error::Overflow
            | Error::OrderExist
            | Error::OrderNotExist => ApiError::User(error as u16),
        }
    }
}
