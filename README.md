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

On most of the modern Linux-based platforms you will already have these installed.

### Table of contents

- [Introduction](#introduction)
- [Getting started](#getting-started)
- [Building code](#building-rust-code)
- [Generating circuit](#generating-circuit)
- [Generating proof locally](#generating-proof-locally)
- [Uploading circuit statement to Proof Market](#uploading-circuit-statement-to-proof-market)
- [Verifying proof on EVM](#verifying-proof-on-evm)

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

1. Install `zkllvm` and `proof-producer` DEB-packages:

    ```bash
    bash -c "echo 'deb [trusted=yes]  http://deb.nil.foundation/ubuntu/ all main' >>/etc/apt/sources.list"
    apt update
    apt install -y zkllvm proof-producer
    ```

2. Install zkLLVM Rust toolchain:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://cdn.jsdelivr.net/gh/NilFoundation/zkllvm@master/rslang-installer.py | python - --channel nightly
    ```

3. Check the installation:

    ```bash
    rustc +zkllvm --version && assigner --version
    ```

    You must see something like this:

    ```plain
    rustc 1.73.0-nightly (a516bc539 2023-12-14) (zkLLVM 0.1.14)
    0.1.14
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

The function `field_arithmetic_example`, defining the circuit is marked with `#[circuit]` attribute.

To build the code, use this command:

```bash
cargo +zkllvm build --release --target assigner-unknown-unknown --features=zkllvm
```

Pay attention to a couple of things:

- `+zkllvm` — tells Cargo to use installed `zkllvm` toolchain instead of original Rust toolchain;
- `--target assigner-unknown-unknown` — tells the compiler to compile the code into a special
intermediate representation (called LLVM assembly), which later will be translated into a circuit;
- `--features=zkllvm` — this feature is used to enable `arkworks` zkLLVM mode, which will result in
generating more efficient circuits.

As a result you will get a file `target/assigner-unknown-unknown/release/zkllvm-rust-template.ll`.
This is LLVM assembly file, which we now will use to produce circuit and assignment table.

## Generating circuit

Previously you used Rust compiler to generate intermediate representation of your code. Now it's
time to produce a circuit itself from it. For this purpose `assigner` tool is used. `assigner`
requires the following files for input:

- generated `.ll` file;
- input file in JSON format, which contains input values for your circuit function.

We have an example input file `inputs/example.inp`. As a result you will have a circuit file
`circuit.crct` and an assignment file `assignment.tbl`.

To generate circuit, use command:

```bash
assigner -b target/assigner-unknown-unknown/release/zkllvm-rust-template.ll -i inputs/example.inp -t assignment.tbl -c circuit.crct -e pallas
```

## Generating proof locally

As soon as you get circuit and assignment table, you are ready to generate the proof. If the circuit
is not too large (like in our example code), you can do it locally on your machine. For this purpose
tool called `proof-generator` is used. We will use multi-threaded version of it:

```bash
proof-generator-multi-threaded --circuit circuit.crct --assignment assignment.tbl --proof proof.bin
```

You will see something like this:

```plain
[info]    Preprocessing public data...
[info]    Preprocessing private data...
[info]    Generating proof...
[info]    Proof Type = N3nil7crypto32zk5snark17placeholder_proofINS0_7algebra6fields17pallas_base_fieldENS2_18placeholder_paramsINS2_26placeholder_circuit_paramsIS6_NS2_28plonk_arithmetization_paramsILm15ELm1ELm35ELm36EEEEENS1_11commitments21lpc_commitment_schemeINSC_34batched_list_polynomial_commitmentIS6_NSC_33list_polynomial_commitment_paramsINS0_6hashes11keccak_1600ILm256EEESI_Lm9ELm2ELb0ENSC_13proof_of_workISI_jLj4294901760EEEEEEENS0_4math14polynomial_dfsINS5_6detail10element_fpINS5_6paramsIS6_EEEESaIST_EEEEEEEEE
[info]    Proof generated
[info]    Verifying proof...
[info]    Proof is verified
[info]    Writing proof to "proof.bin"
[info]    Proof written
```

As a result, you will have the file `proof.bin`, which is a serialized proof from Placeholder proof
system. As you can see, your proof got already verified just before the serialization.

## Uploading circuit statement to Proof Market

__TODO__: fill this section

## Verifying proof on EVM

__TODO__: fill this section
