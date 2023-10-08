use cairo_platinum_prover::Felt252;

// countingsort over the i-th digits
// i takes values from 0 to 79
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
pub fn radixsort(tuples: &Vec<(Felt252, Felt252)>) -> Vec<(Felt252, Felt252)> {
    let mut new_tuples = tuples.clone();
    for i in 0..80 {
        let output = countingsort_limbs(&new_tuples, i);
        new_tuples = output;
    }
    new_tuples
}