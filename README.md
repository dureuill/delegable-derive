# delegable-derive

Experimental proc macro to delegate the implementation of some traits to a field or function.

Requires nightly.

# Usage

See the [companion test repository](https://github.com/dureuill/delegate_test).

# Examples

See the [examples directory](examples/), that contains the following examples:

* `my_into`: a delegable implementation of an `Into`-like trait. Demonstrates
  that the macro works with names that are overloaded like `into()` (also,
  generic parameter).
* `vec_facade: a delegable trait that implements the inherent methods of `Vec`.
  It is easy to have a "vector-like" struct by implementing the delegate trait
  for the `VecFacade` trait.
