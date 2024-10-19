# Rust gems

This repository offers some Rust snippets that might be useful when studying the language.

- [vec_any](src/vec_any.rs) Collect trait objects in a vector and use downcast_ref to access the concrete type instances of the items
- [mutate_in_closure](src/mutate_in_closure.rs) Mutate a value in a closure without copy and clone
- [from_str](src/from_str.rs) Thou shall not implement From\<str'> but instead implement the FromStr trait
- [graphemes](src/graphemes.rs) Trim an unicode string to a maximum length with

## Run the snippets

Dependencies:

- Install the command runner [just](https://github.com/casey/just?tab=readme-ov-file#installation).
- Install the test runner [cargo-nexttest](https://nexte.st/).

Run the snippets in debug mode:

```shell
just test-all
```

Run the snippets in release mode:

```shell
just test-all --release
```
