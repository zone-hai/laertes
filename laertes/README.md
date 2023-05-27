# Laertes -- A Polonius-based tool to tighten lifetime annotations

## Building

We depend on `rustc` as a library to access ASTs so Laertes has to use
the nightly Rust toolchain. We use a specific version of nightly Rust
that supports `rls` (you need to check for breaking APIs for future
versions): `nightly-2020-10-15`. You can install it by running

```sh
rustup install nightly-2020-10-15
```

You can either run the Rust-related commands (like `cargo`, `rustc`)
with `+nightly-2020-10-15` every single time, or mark the project
directory to always use the nightly toolchain by using the following
command in the project directory:

```sh
rustup override set nightly-2020-10-15
```

For the further commands, if you haven't overridden the toolchain, you
need to add `+nightly-2020-10-15` argument.

Install `rustc` as a library:

```sh
rustup component add rustc-dev
```

The following components are optional, and they are useful for using
RLS to have a nicer development environment:

```sh
rustup component add rls rust-analysis rust-src
```

## Binaries

Laertes consists of several binaries (each with its own main file
under `src/bin` per rust packaging convention):

- `remove-raw-ptrs`: This binary takes a bunch of Rust files, removes
  all non-void raw pointers and converts them to immutable references
  (`&`) and introduces lifetimes in type and function declarations.
- `fix-borrow-errors`: This binary takes the output from Polonius, and
  corresponding Rust source code. It then proposes some fixes to
  reduce ownership violations in the Rust code.

## Workflow

1. Run `c2rust` to get the initial unsafe Rust code
2. Run `remove-raw-ptrs` to make the Rust code overly optimistic,
   using immutable references everywhere. This code is likely not
   going to pass the borrow checker. In the next step, we are going to
   iteratively relax this program until it passes the borrow checker.
3. Until reaching a fixpoint,
   1. Run `cargo rustc -- -Znll-facts` to get non-lexical lifetime
   constraints to feed to Polonius
   2. Run Polonius, with naive deduction rules, on the lifetime facts
      by running this command on Polonius directory: `cargo +nightly run --release -- --show-tuples -v -a Naive <path-to-lifetime-constraints>` and save the output to a file using `tee`.
   3. Run `remove-raw-ptrs` on Polonius output and the Rust program,
      it will apply the fixes in-place.
   4. Repeat step 3
