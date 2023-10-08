use cairo_platinum_prover::Felt252;
use hex_wrapper::Hex64;

mod radixsort;
use radixsort::radixsort;

fn main() {
    let mut addresses1: Vec<Felt252> = Vec::new();
    let mut values1: Vec<Felt252> = Vec::new();

    for _i in (0..5).step_by(1) {
        addresses1.push(Felt252::from_hex_unchecked(
            &format!(
                "{}{}{}{}",
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string()
            )[0..],
        ));
        values1.push(Felt252::from_hex_unchecked(
            &format!(
                "{}{}{}{}",
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string(),
                Hex64::rand().to_string()
            )[0..],
        ));
    }

    let mut tuples1: Vec<_> = addresses1.into_iter().zip(values1).collect();
    let tuples2 = tuples1.clone();

    tuples1.sort_by(|(x, _), (y, _)| x.representative().cmp(&y.representative()));

    radixsort(&tuples2);
}
