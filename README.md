## Radixsort
[Radixsort](https://en.wikipedia.org/wiki/Radix_sort) algorithm in Rust for large integers. For handing large integers, we use [Lambdaworks](https://github.com/lambdaclass/lambdaworks) for sorting numbers from the Stark prime field.

## Results

Below we present a benchmark comparing `radixsort` against the method `sort_by` in from `std::slice`.  Clearly, `sort_by` beats our `radixsort` by a wide margin, where the execution times of `radixsort` is in the order of miliseconds and the `sort_by` is in the order to microseconds. See [here](https://mdvillagra.github.io/radixsort/) for more detailed results.

![50_to_500](https://mdvillagra.github.io/radixsort/50_to_500/Radixsort/report/lines.svg)

## Potential Improvements

There are still room for improvements in our naive `radixsort` implementation.

1. Replace the places where we `clone` the values with a more efficient approach.
2. Use an in-place version of radixsort.
3. Use binary search for updating the buckets.