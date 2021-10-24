//! Runs a fibonacci computation. Exists for the purpose of a being disassembled.
use bench_vtables::fibonacci;

fn main() {
    let table = fibonacci::make_fibonacci_packed_sieve::<_, 3, 4>();
    let ptr = bench_vtables::packed_sieve_table::PackedSievePtr::new(&table);
    let fib = fibonacci::fibonacci(ptr, 20);
    println!("fib={}", fib);
}
