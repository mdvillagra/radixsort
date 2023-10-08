use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion,
    SamplingMode,
};

use std::slice;

use cairo_platinum_prover::Felt252;
use hex_wrapper::Hex64;

fn radixsort_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Radixsort");

    let mut addresses1: Vec<Felt252> = Vec::new();
    let mut values1: Vec<Felt252> = Vec::new();

    let n_rounds = 1;
    let n_inputs = 500000;

    for rounds in 0..n_rounds {
        for i in 0..n_inputs {
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

        let mut tuples1: Vec<_> = addresses1
            .clone()
            .into_iter()
            .zip(values1.clone())
            .collect();
        let tuples2 = tuples1.clone();

        //tuples1.sort_by(|(x, _), (y, _)| x.representative().cmp(&y.representative()));

        group.sample_size(10);

        group.bench_with_input(
            BenchmarkId::new("radixsort", rounds as u32),
            &tuples2,
            |b, tuples2| b.iter(|| black_box(radixsort(&tuples2))),
        );

        group.bench_function(BenchmarkId::new("sort_by", rounds as u32), |b| {
            b.iter(|| tuples1.sort_by(|(x, _), (y, _)| x.representative().cmp(&y.representative())))
        });
    }
}

criterion_group!(benches, radixsort_bench);
criterion_main!(benches);

// countingsort over the i-th digits
// i takes values from 0 to 255
fn countingsort_limbs(tuples: &Vec<(Felt252, Felt252)>, i: u16) -> Vec<(Felt252, Felt252)> {
    let mut buckets: Vec<usize> = vec![0; 10]; //buckets for digits 0 to 9
    let limb: usize = 3 - (i as u16 / 20u16) as usize; //limb position where the i-th digit is
    let d = (i as u32) % 20u32; //position of the i-th digit inside the limb

    for j in 0..tuples.len() {
        buckets[((tuples[j].0.representative().limbs[limb]) / (10u64.pow(d)) % 10u64) as usize] +=
            1;
    }

    for j in 1..10 {
        buckets[j] += buckets[j - 1];
    }

    let mut output: Vec<(Felt252, Felt252)> =
        vec![(Felt252::zero(), Felt252::zero()); tuples.len()];

    for j in (0..tuples.len()).rev() {
        output[buckets
            [((tuples[j].0.representative().limbs[limb]) / (10u64.pow(d)) % 10u64) as usize]
            - 1] = tuples[j].clone();
        buckets[((tuples[j].0.representative().limbs[limb]) / (10u64.pow(d)) % 10u64) as usize] -=
            1;
    }

    output
}

//radix sort algorithm
fn radixsort(tuples: &Vec<(Felt252, Felt252)>) {
    let mut new_tuples = tuples.clone();
    for i in 0..80 {
        let output = countingsort_limbs(&mut new_tuples, i);
        new_tuples = output;
    }
}
