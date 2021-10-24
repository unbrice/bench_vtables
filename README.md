# SieveTable: experimenting with v-tables

This repo is not production-quality code. Instead it is intended as a playground to experiment with various representation of V-Tables for multiple Rust traits.

I created it in the context of [rust-lang #2035](https://github.com/rust-lang/rfcs/issues/2035#issuecomment-422060294). That is a request to be able to do `Box<A+…+B+…+C>` → `Box<A+B+C>`.

I first wrote a [proposal](https://internals.rust-lang.org/t/sieve-tables-for-multiple-traits-objects-box-a-b-c-to-box-a-c-2035/15397) of a new solution to that problem, this repo is mostly so that I can benchmark the propsal. It could be useful to benchmark  other approaches.

Bencmarking is an art I am far from mastering, contributions are welcome. I can add some doc if people are interested.

# Bench results

[`src/fibonacci.rs`](src/fibonacci.rs) implements  an inefficient recursive Fibonacci based on virtual calls. [`benches/fibonacci.rs`](benches/fibonacci.rs) benches it for various number of traits.

On 2021-10-24, I ran on a 2019 Mac `cargo criterion` and observed the following:

|               | Size      | 2 traits | 3 traits | 4 traits | 5 traits |
| -------------:|:---------:|:--------:|:--------:|:--------:|:--------:|
| `PackedSieve` | 2 words   | 464 ns   | 488 ns   | 482 ns   | 495 ns   |
| `InlineSieve` | 3 words   | 510 ns   | 510 ns   | 510 ns   | 510 ns   |
| `MultiVPtr`   | N+1 words | 472 ns   | 753 ns   | 764 ns   | 831 ns   |

With one trait, `VPtr`, a simulation of Rust current's approach took 402 ns. `MultiVPtr`, which is a simulation of the proposed "extra-fat pointers" took 421 ns.

## Interpretation

- `PackedSieve` performed almost as well as `MultiVPtr` for 2 traits, and better for all other trait numbers, while also being smaller. 

- `InlineSieve` has the advantage of stable performance, as it does not depend on the number of traits. Nevertheless, it is a bit larger and significantly more complex.

- `MultiVPptr` behaving the same as `VPtr` with a single trait is encouraging.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

```

```
