# Rust gems

This repository offers some Rust snippets that can be useful when studying the language.

- [vec_any](src/vec_any.rs) Collect trait objects in a vector and use downcast_ref to access the concrete type instances of the items
- [mutate_in_closure](src/mutate_in_closure.rs) Mutate a value in a closure without copy and clone by using Rc
- [async_higher_order_fn](src/async_higher_order_fn.rs) Implement an async higher order function that accept an async closures and returns an async result
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

Run clippy validations and tests in parallel:

```shell
just clippy-release & just test-all
```
