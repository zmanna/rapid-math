# Rapid Math

A Rust command-line math practice tool focused on fast arithmetic drills.

## Overview

This repository is a compact Rust project. It is useful for understanding how a Cargo application is structured and how terminal-based practice tools can be built.

## What This Project Demonstrates

- Using Cargo to manage a Rust binary project.
- Keeping application logic in `src/main.rs` for a small command-line tool.
- Building and running a fast local utility from source.

## Project Structure

| Path | Purpose |
|---|---|
| `Cargo.toml` | Rust package metadata and dependency configuration. |
| `Cargo.lock` | Resolved dependency versions for reproducible builds. |
| `src/main.rs` | Application entry point and core command-line behavior. |
| `.DS_Store` | macOS metadata currently present in the repository; future cleanup should remove it from version control. |

## How to Run or Review

```sh
cargo run
cargo build --release
```

## How to Read This Repository

Read `src/main.rs` first. If the project grows, the next documentation step is to split input handling, scoring, and drill generation into separate modules.

## Documentation Roadmap

This README is the first cleanup pass. The next documentation pass should add:

- A fuller problem statement or assignment prompt.
- Setup notes for required tools, environment variables, or services.
- Example input and output.
- Screenshots, diagrams, or architecture notes where they clarify the project.
- Testing instructions and known limitations.

## Repository Status

This repository has been renamed and labeled as part of a GitHub organization cleanup. The goal is to make the project understandable to future readers, reviewers, and collaborators.
