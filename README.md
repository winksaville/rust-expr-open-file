# Experiment with openning a file

I wanted to have a `fn open_file` which would OpenOptions as a parameter
but was having trouble doing so and was getting errors that OpenOptions
was not living long enough.

The solution is to hoist it up to the next outer scope, I wonder if
there is an easier/better solution?

## Running

The first time its run `file.txt` is created and `Hello, World` written to it:
```
wink@3900x:~/prgs/rust/projects/expr-open-file (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/expr-open-file`
Hello, World
```
Each subsequent run `Hello, World` is appended, so the second time we see two `Hello, World`'s:
```
wink@3900x:~/prgs/rust/projects/expr-open-file (main)
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/expr-open-file`
Hello, World
Hello, World
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
