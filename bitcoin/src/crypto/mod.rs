// SPDX-License-Identifier: CC0-1.0

//! Cryptography
//!
//! Cryptography related functionality: keys and signatures.

pub mod key;
pub mod sighash;
// Contents re-exported in `bitcoin::taproot`.
pub(crate) mod taproot;

/// ECDSA Bitcoin signatures.
pub mod ecdsa {
    #[doc(no_inline)]
    pub use crypto::ecdsa::{DecodeError, ParseSignatureError};
    #[doc(inline)]
    pub use crypto::ecdsa::{SerializedSignature, Signature};
}
