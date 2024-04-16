use ff::PrimeField;
use poseidon_rs::Fr;
use poseidon_rs::FrRepr;
fn main() {
    let (prove_poseidon, verify_poseidon) = guest::build_poseidon();

    let input = [2u64, 0, 0, 0];
    let (output, proof) = prove_poseidon(input);
    let is_valid = verify_poseidon(proof);

    println!(
        "output: {}",
        Fr::from_repr(FrRepr(output)).unwrap().to_string()
    );
    println!("valid: {}", is_valid);
}
