# Contributing

## Setup

```sh
rustup default stable
cargo run
```

## Build Check

```sh
cargo check
```

## Project Conventions

- Keep game-state changes inside `MathQuizApp`.
- Move reusable logic into modules if the file grows.
- Keep problem generation deterministic enough to test by injecting or controlling randomness.
- Prefer keyboard-friendly interactions because the game is speed-based.

## Good First Improvements

- Add unit tests for `generate_problem`.
- Split UI, game state, and problem generation into separate files.
- Add a high-score file.
- Add configurable game duration.
- Add mode selection for arithmetic categories.

