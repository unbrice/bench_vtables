# SieveTable: v-tables for sum traits

This repo is a playground to experiment with various representations of V-Tables for multiple Rust traits.

I this repo in the context of [rust-lang #2035](https://github.com/rust-lang/rfcs/issues/2035#issuecomment-1062770650). I wrote a blog post [presenting the issue](https://vleu.net/posts/2022/sievetables/#issue) as well as [a proposed solution](https://vleu.net/posts/2022/sievetables/#proposal). This repo is mostly so that I could benchmark that propsal. It could be useful to benchmark other approaches.

To give an overview, it proposes a representation that:
- Allows turning a sum trait to a subset of the sum (`Box<A+…+B+…+C>` → `Box<A+B+C>`) without heap allocations.
- Keeps pointers to a sum-trait type on a regular two words [fat-pointer](https://stackoverflow.com/questions/57754901/what-is-a-fat-pointer).
- Performs better than the often discussed "extra fat" pointers, at least on my x86_64 machine according to my naive [benchmarks](#bench-results).
- Does not require full program optimisation (individual crates can remain compiled separately).
- Offers multiple variants and tuning opportunities for varying architectures.

More details in [the blog post](https://vleu.net/posts/2022/sievetables/).
Bencmarking is an art I am far from mastering, contributions are welcome.

## Bench results

[`src/fibonacci.rs`](src/fibonacci.rs) implements  an inefficient recursive Fibonacci based on virtual calls. [`benches/fibonacci.rs`](benches/fibonacci.rs) benches it for various number of traits.

On 2021-10-24, I ran on a 2019 Mac `cargo criterion` and observed the following:

|               | Size      | 1 trait | 2 traits | 3 traits | 4 traits | 5 traits |
| -------------:|:---------:|:--------:|:--------:|:--------:|:--------:|:--------:|
| `VPtr       ` | 2 words   | 402ns    | -        | -        | -        | -        |
| `PackedSieve` | 2 words   | -        | 464 ns   | 488 ns   | 482 ns   | 495 ns   |
| `InlineSieve` | 3 words   | -        | 510 ns   | 510 ns   | 510 ns   | 510 ns   |
| `MultiVPtr`<br>(Extra-fat pointers)  | N+1 words | -        | 472 ns   | 753 ns   | 764 ns   | 831 ns   |

### Interpretation

- `PackedSieve` performed as well as `MultiVPtr` for 2 traits, and better for all other trait numbers, while also being smaller. 
- `InlineSieve` has the advantage of stable performance, as it does not depend on the number of traits. Nevertheless, it is a bit larger and significantly more complex.

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
