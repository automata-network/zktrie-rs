#![feature(test)]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "tstd")]
#[macro_use]
extern crate sgxlib as std;


#[cfg(test)]
extern crate test;

pub use poseidon_rs::{Field, Fr, FrRepr, PrimeField, PrimeFieldDecodingError};

mod util;
pub use util::*;

mod hash;
pub use hash::*;
#[cfg(test)]
mod hash_test;

mod byte32;
pub use byte32::*;
#[cfg(test)]
mod byte32_test;

mod database;
pub use database::*;

mod proof;
pub use proof::*;

mod node;
pub use node::*;
#[cfg(test)]
mod node_test;

mod zktrie;
pub use crate::zktrie::*;
#[cfg(test)]
mod zktrie_test;
#[cfg(test)]
mod zktrie_bench;

mod zktrie_deletionproof;
pub use crate::zktrie_deletionproof::*;