use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion,
};

use cairo_platinum_prover::Felt252;
use hex_wrapper::Hex64;

#[path="../src/radixsort.rs"]
mod radixsort;
use radixsort::radixsort;

fn radixsort_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Radixsort");

    let mut addresses1: Vec<Felt252> = Vec::new();
    let mut values1: Vec<Felt252> = Vec::new();

    let n_rounds = 1;//number of rounds to test
    let n_inputs = 50;//size increase in each round

    println!("=== Benchmarking {} inputs ===", n_rounds);
    println!("=== Each round the input size increases by {} numbers ===", n_inputs);

    for rounds in 0..n_rounds {
        for _i in 0..n_inputs {
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