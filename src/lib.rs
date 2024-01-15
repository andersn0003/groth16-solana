//! # groth16-solana
//!
//! Groth16 zero-knowledge proof verification with Solana altbn254 syscalls.
//!
//! Verification takes less than 200,000 compute units.
//!
//! The syscalls are contained in Solana releases 1.15.0 onwards and yet to be activated on a public network.
//!
//! Inputs need to be in u8 arrays in big endian.
//!
//! See functional test as an example how to use this library.
//!
//! This crate is compatible with Groth16 proofs of circom circuits.
//!
//! The verifier file can be generated with the java script script from a verifyingkey.json file generated by snarkjs.
//!
//! ## Usage:
//!
//! ```rust,ignore
//! let mut public_inputs_vec = Vec::new();
//! for input in PUBLIC_INPUTS.chunks(32) {
//!     public_inputs_vec.push(input);
//! }
//!
//! let proof_a: G1 =
//!     <G1 as FromBytes>::read(&*[&change_endianness(&PROOF[0..64])[..], &[0u8][..]].concat())
//!         .unwrap();
//! let mut proof_a_neg = [0u8; 65];
//! <G1 as ToBytes>::write(&proof_a.neg(), &mut proof_a_neg[..]).unwrap();
//!
//! let proof_a = change_endianness(&proof_a_neg[..64]).try_into().unwrap();
//! let proof_b = PROOF[64..192].try_into().unwrap();
//! let proof_c = PROOF[192..256].try_into().unwrap();
//!
//! let mut verifier = Groth16Verifier::new(
//!     &proof_a,
//!     &proof_b,
//!     &proof_c,
//!     public_inputs_vec.as_slice(),
//!     &VERIFYING_KEY,
//! )
//! .unwrap();
//! verifier.verify().unwrap();
//! ```
//!
//! See functional test for a running example how to use this library.
//!
pub mod decompression;
pub mod errors;
pub mod groth16;
pub mod syscalls;
