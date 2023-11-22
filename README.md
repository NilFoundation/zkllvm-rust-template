# zkLLVM Rust template

This repository goes step-by-step through the process of using zkLLVM toolchain to generate zk-proof
for algorithms implemented in Rust.

Table of contents:

- [Prerequisites](#prerequisites)
- [Install zkLLVM toolchain](#install-zkllvm-toolchain)
- [Building circuit](#building-rust-code)

## Prerequisites

Supported platforms:

- Linux x86-64

Dependencies:

- Python 3.7+

## Install zkLLVM toolchain

1. Install `zkllvm` deb-package as described [here][zkllvm-deb-package].
2. Install zkLLVM Rust as described [here][rslang-binary-installation].
3. Install proof-generator: **TODO**

To check the installation, print versions:

```bash
rustc +zkllvm --version && assigner --version && transpiler --version
```

You must see something like this:

```plain
TODO
```

Now you are ready to build the circuit.

## Building Rust code

As you can see, this repository has a simple Rust project structure. It has a single binary crate
`zkllvm-rust-template` with all the code contained in `src/main.rs`.

The function `validate_path`, defining the circuit is marked with `#[circuit]` attribute.

To build the code, use this command:

```bash
cargo +zkllvm build --release --target assigner-unknown-unknown --ignore-rust-version --features=zkllvm
```

Pay attention to a couple of things:

- `+zkllvm` — tells Cargo to use installed `zkllvm` toolchain instead of original Rust toolchain.
- `--target assigner-unknown-unknown` — tells the compiler to compile the code into a special
intermediate representation (called LLVM assembly), which later will be translated into a circuit.
- `--ignore-rust-version` — this is a dirty hack, which will be removed soon.
- `--features=zkllvm` — this feature is used to enable `arkworks` zkLLVM mode, which will result in
generating more efficient circuits.

As a result you will get a file `target/assigner-unknown-unknown/release/zkllvm-rust-template.ll`.
This is LLVM assembly file, which we now will use to produce circuit and assignment table.

## Generating circuit

Previously you used Rust compiler to generate intermediate representation of your code. Now it's
time to produce a circuit itself from it. For this purpose `assigner` tool is used. `assigner`
requires the following files for input:

- generated `.ll` file
- input file in JSON format, which contains input values for your circuit function

We have an example input file `inputs/example.inp`. As a result you will have a circuit file `.crct` and an assignment file `.tbl`.

To generate circuit, use command:

```bash
assigner -b target/assigner-unknown-unknown/release/zkllvm-rust-template.ll -i inputs/example.inp -t assignment.tbl -c circuit.crct -e pallas
```

## Generate and verify a proof locally

TODO

[zkllvm-deb-package]: https://docs.nil.foundation/zkllvm/starting-first-project/installation#binary-installation
[rslang-binary-installation]: https://github.com/NilFoundation/zkLLVM#rust-toolchain
