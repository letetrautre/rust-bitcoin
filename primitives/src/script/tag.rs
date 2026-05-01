// SPDX-License-Identifier: CC0-1.0

//! Script tags.
//!
//! Tags are used to differentiate the different kinds of scripts that appear
//! in Bitcoin transactions.

/// Sealed trait representing a type of script.
pub trait Tag {}

/// A P2SH redeem script.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum RedeemScriptTag {}
impl Tag for RedeemScriptTag {}

/// A script signature (scriptSig).
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ScriptSigTag {}
impl Tag for ScriptSigTag {}

/// A script public key (scriptPubKey).
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ScriptPubKeyTag {}
impl Tag for ScriptPubKeyTag {}

/// A signet block challenge script.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum SignetBlockScriptTag {}
impl Tag for SignetBlockScriptTag {}

/// A Segwit v1 Taproot script.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum TapScriptTag {}
impl Tag for TapScriptTag {}

/// A Segwit v0 witness script.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum WitnessScriptTag {}
impl Tag for WitnessScriptTag {}
