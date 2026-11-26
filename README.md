# First Rust Program 🦀

A beginner-level Rust program exploring basic programming concepts through a biological sequence.

## Overview

This project is my first standalone Rust program. It uses a nucleotide sequence obtained from an NCBI Reference Sequence (RefSeq) record to demonstrate basic string handling and output in Rust.

## Biological Reference

| Field | Information |
|---|---|
| Organism | *Homo sapiens* |
| Gene | LOC127903862 |
| Accession | NM_001414722.1 |
| Molecule | mRNA |
| Source | NCBI RefSeq |

**Sequence database:** [NCBI Nucleotide – NM_001414722.1](https://www.ncbi.nlm.nih.gov/nuccore/NM_001414722.1)

## Features

- Stores a nucleotide sequence as a string.
- Calculates sequence length.
- Displays the sequence and its length.
- Demonstrates basic Rust syntax and string methods.

## Technologies

- **Language:** Rust
- **Compiler:** rustc
- **Editor:** Sublime Text
- **Version Control:** Git

## How to Run

Clone the repository:

```bash
git clone https://github.com/Shreyas-the-1st/first-rust-program.git
cd first-rust-program
```

Compile the program:

```bash
rustc first-rust-program.rs
```

Execute:

```bash
./first-rust-program
```

## Concepts Practiced

- `fn main()` — program entry point.
- `println!()` — formatted output.
- `let` — variable declaration.
- `&str` — string slices.
- `.len()` — string length calculation.

## Learning Objective

This project marks the beginning of my Rust programming journey, with a particular interest in computational biology and bioinformatics.

Future improvements may include nucleotide frequency analysis, GC content calculation, and sequence validation.

---

**Author:** [Shreyas-the-1st](https://github.com/Shreyas-the-1st)

*An ongoing journey into Rust and bioinformatics.*
