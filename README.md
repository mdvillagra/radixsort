## Radixsort
[Radixsort](https://en.wikipedia.org/wiki/Radix_sort) algorithm in Rust for large integers. For handling large integers, we use [Lambdaworks](https://github.com/lambdaclass/lambdaworks) for sorting numbers from the Stark prime field.

## Results

Below we present a benchmark comparing `radixsort` against the method `sort_by` from `std::slice`. All benchmarks were run on an Intel Xeon CPU  of 2.40GHz. Clearly, `sort_by` beats our `radixsort` by a wide margin, where the execution times of `radixsort` is in the order of miliseconds and the `sort_by` method is in the order of microseconds. We start with a vector of 100,000 random numbers each of size 256 bits and in each iteration we add 100,000 more numbers upt to 1,000,000 numbers in total.

See [here](https://mdvillagra.github.io/radixsort/) for more detailed results.

![100000_to_1000000](https://mdvillagra.github.io/radixsort/criterion/Radixsort-vs-sortby-100-to-1000/report/lines.svg)

![sort_by only](https://mdvillagra.github.io/radixsort/criterion/Radixsort-vs-sortby-100-to-1000/sort_by/report/lines.svg)

![radixsort only](https://mdvillagra.github.io/radixsort/criterion/Radixsort-vs-sortby-100-to-1000/radixsort/report/lines.svg)

## Potential Improvements

There are still room for improvements in our naive `radixsort` implementation.

1. Replace the places where we `clone` the values with a more efficient approach.
2. Use an in-place version of radixsort.
3. Use binary search for updating the buckets.