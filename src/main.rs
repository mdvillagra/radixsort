use cairo_platinum_prover::Felt252;
use hex_wrapper::{Hex32, Hex64};
use std::{collections::HashMap, ops::AddAssign};

fn hex_to_dec(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' | 'A' => 10,
        'b' | 'B' => 11,
        'c' | 'C' => 12,
        'd' | 'D' => 13,
        'e' | 'E' => 14,
        _ => 15,
    }
}

// Countingsort procedure for vectors of pairs.
// It sorts using the i-th digit from the right in the hex representation of the left element.
fn countingsort(tuples: &Vec<(Felt252, Felt252)>, i: usize) {
    let mut c: Vec<u64> = vec![0; 16];

    //compute the buckets
    for j in 0..tuples.len() {
        if tuples[j].0.to_string().len() - 2 < i {
            c[0] += 1;
        } else {
            c[hex_to_dec(
                tuples[j]
                    .0
                    .to_string()
                    .chars()
                    .nth(tuples[j].0.to_string().len() - 2 - i - 1)
                    .unwrap(),
            )] += 1;
        }
    }

    //accumulates
    for j in 1..16 {
        c[j] = c[j] + c[j - 1];
    }
}

fn main() {
    let mut addresses1: Vec<Felt252> = Vec::new();
    let mut values1: Vec<Felt252> = Vec::new();
    let mut addresses2: Vec<Felt252> = Vec::new();
    let mut values2: Vec<Felt252> = Vec::new();

    for _i in (0..5).step_by(1) {
        addresses1.push(Felt252::from_hex_unchecked(&Hex64::rand().to_string()[0..]));
        addresses2.push(Felt252::from_hex_unchecked(&Hex32::rand().to_string()[0..]));
        values1.push(Felt252::from_hex_unchecked(&Hex64::rand().to_string()[0..]));
        values2.push(Felt252::from_hex_unchecked(&Hex32::rand().to_string()[0..]));
    }

    let mut tuples1: Vec<_> = addresses1.into_iter().zip(values1).collect();
    let mut tuples2: Vec<_> = addresses2.into_iter().zip(values2).collect();

    tuples1.sort_by(|(x, _), (y, _)| x.representative().cmp(&y.representative()));

    println!(
        "{} {}",
        tuples1[0].0.to_string().len(),
        tuples2[0].0.to_string().len()
    );
}
