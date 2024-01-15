# zkLLVM Rust template

[![Tutorial check](https://github.com/NilFoundation/zkllvm-rust-template/actions/workflows/main.yml/badge.svg)](https://github.com/NilFoundation/zkllvm-rust-template/actions/workflows/main.yml)

This repository goes step-by-step through the process of using zkLLVM toolchain to generate
zero-knowledge proof for Rust code.

## Prerequisites

Supported platforms:

- Linux x86-64

Dependencies:

- Python 3.10+
- Git
- Curl

### Table of contents

- [Introduction](#introduction)
- [Getting started](#getting-started)
- [Building code](#building-rust-code)
- [Generating circuit](#generating-circuit)

## Introduction

This turorial goes step-by-step through the process of circuit development.

Overall process includes:

- writing source code in Rust (you already have a simple piece of code in this repository)
- generating arithmetic circuit for this code
- generating assignment table (execution trace) for this circuit
- generating proof for this circuit locally
OR
- posting a request on Proof Marker

__TODO:__ fix this section

## Getting started

First you need to install zkLLVM compilers and tools.

### Install zkLLVM toolchain

1. Install `zkllvm` and `proof-produces` DEB-packages:

    ```bash
    bash -c "echo 'deb [trusted=yes]  http://deb.nil.foundation/ubuntu/ all main' >>/etc/apt/sources.list"
    apt update
    apt install -y zkllvm proof-producer
    ```

2. Install zkLLVM Rust toolchain:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://cdn.jsdelivr.net/gh/NilFoundation/zkllvm@master/rslang-installer.py | python - --channel nightly
    ```

Check the installation:

```bash
rustc +zkllvm --version && assigner --version && proof-generator-single-threaded --version
```

You must see something like this:

```plain
0.1.8-4
rustc 1.68.0-nightly (bd2e7bf46 2023-11-01) (zkLLVM 0.1.8)
```

### Clone repository

```bash
git clone https://github.com/NilFoundation/zkllvm-rust-template.git
cd zkllvm-rust-template
```

Now you are ready build something.

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

## Generate a proof locally

```bash
proof-generator-single-threaded --circuit circuit.crct --assignment assignment.tbl --proof proof.bin
```
