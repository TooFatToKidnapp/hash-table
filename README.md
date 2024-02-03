# Hash Table Implementation in [Rust](https://rust-lang.org/)

This repo was purely for educational purposes.

the benchmarks show how slow this vertion of [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) is.

For resolving collisions we use [Open Addressing](https://en.wikipedia.org/wiki/Hash_table#Open_addressing). For the hash function we use [djb2](http://www.cse.yorku.ca/~oz/hash.html)

## Quick Start

```console
$ cargo run --release
```

## Run tests

```console
$ cargo test
```
