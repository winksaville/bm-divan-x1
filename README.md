# bm-divan-x1

```plain
WWWWWW
||||||
((((((
```

A simple lib, app and benchmark example using divan [[1]].

## Example

```plain
WWWWWW
||||||
((((((

wink@fwlaptop 26-03-27T21:39:13.970Z:~/data/prgs/rust/benchmark-divan-x1 (main)
$ cargo bench
   Compiling benchmark-divan-x1 v0.1.0 (/home/wink/data/prgs/rust/benchmark-divan-x1)
    Finished `bench` profile [optimized] target(s) in 5.36s
     Running unittests src/lib.rs (target/release/deps/benchmark_divan_x1-5d94dab880745c0f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/release/deps/benchmark_divan_x1-c7b11dc8e9ede157)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/bm_loops.rs (target/release/deps/bm_loops-adef6b91318bd6b3)
Timer precision: 12 ns
bm_loops        fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ loops_bench                │               │               │               │         │
   ├─ 1000      275.2 ns      │ 279.7 ns      │ 276.1 ns      │ 276.2 ns      │ 100     │ 800
   ╰─ 1000000   238.6 µs      │ 290.1 µs      │ 263.7 µs      │ 260.1 µs      │ 100     │ 100

     Running benches/fib.rs (target/release/deps/fib-e7c4f10844a38880)
Timer precision: 13 ns
fib           fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ fibonacci                │               │               │               │         │
   ├─ 1       1.048 ns      │ 1.703 ns      │ 1.053 ns      │ 1.065 ns      │ 100     │ 102400
   ├─ 2       1.975 ns      │ 6.574 ns      │ 2.245 ns      │ 2.267 ns      │ 100     │ 102400
   ├─ 4       5.747 ns      │ 11.56 ns      │ 5.901 ns      │ 6.22 ns       │ 100     │ 25600
   ├─ 8       42.18 ns      │ 86.75 ns      │ 46.86 ns      │ 45.59 ns      │ 100     │ 3200
   ├─ 16      2.09 µs       │ 3.726 µs      │ 2.122 µs      │ 2.224 µs      │ 100     │ 100
   ╰─ 32      4.684 ms      │ 5.933 ms      │ 4.702 ms      │ 4.761 ms      │ 100     │ 100

wink@fwlaptop 26-03-28T01:31:39.547Z:~/data/prgs/rust/benchmark-divan-x1 (main)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[1]: https://github.com/nvzqz/divan
