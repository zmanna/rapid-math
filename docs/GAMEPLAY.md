# Gameplay Rules

## Objective

Answer as many arithmetic questions as possible before the timer reaches zero.

## Timer

- Starting time: 30 seconds.
- Correct answer: adds 1 second.
- Incorrect answer: subtracts 2 seconds.
- Invalid input: subtracts 2 seconds.

## Scoring

- Simple arithmetic problem: 1 point.
- PEMDAS-style problem: 2 points.

## Difficulty Progression

| Score Range | Number Range | Complex Problems |
|---|---:|---|
| `< 5` | 1-10 | disabled |
| `5-9` | 1-20 | enabled |
| `>= 10` | 1-50 | enabled |

Complex problems are generated with a probability of 30% once enabled.

## Problem Types

Simple problems:

- addition
- subtraction
- multiplication
- division with integer-safe construction

Complex problems:

- multiplication applied to grouped addition
- grouped subtraction followed by division

## Current Limitations

- Some generated division expressions use integer division.
- There is no high-score persistence.
- There is no test suite for problem generation.
- The UI is functional but minimal.

