use cairo_platinum_prover::Felt252;
use hex_wrapper::Hex64;

fn main() {
    let mut addresses1: Vec<Felt252> = Vec::new();
    let mut values1: Vec<Felt252> = Vec::new();
    let mut addresses2: Vec<Felt252> = Vec::new();
    let mut values2: Vec<Felt252> = Vec::new();

    for _i in (0..1000).step_by(50) {
        addresses1.push(Felt252::from_hex_unchecked(&Hex64::rand().to_string()[0..]));
        addresses2.push(Felt252::from_hex_unchecked(&Hex64::rand().to_string()[0..]));
        values1.push(Felt252::from_hex_unchecked(&Hex64::rand().to_string()[0..]));
        values2.push(Felt252::from_hex_unchecked(&Hex64::rand().to_string()[0..]));
    }

    let mut tuples1: Vec<_> = addresses1.into_iter().zip(values1).collect();
    let mut tuples2: Vec<_> = addresses2.into_iter().zip(values2).collect();

    tuples1.sort_by(|(x, _), (y, _)| x.representative().cmp(&y.representative()));

    println!("{:?}", Hex64::rand());
}
