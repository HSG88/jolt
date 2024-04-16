#![cfg_attr(feature = "guest", no_std)]
#![no_main]
use ff::*;
use poseidon_rs::{Fr, Poseidon};
use poseidon_rs::FrRepr;

#[jolt::provable]
fn poseidon(input: [u64;4]) -> [u64;4] {
    let preimage = Fr::from_repr(FrRepr(input)).unwrap();
    // let preimage = Fr::from_str(input).unwrap();
    let poseidon = Poseidon::new();
    let result = poseidon.hash(vec![preimage]).unwrap();
    result.into_repr().0
}
