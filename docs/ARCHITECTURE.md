# Architecture

## Purpose

Rapid Math is a Rust desktop arithmetic practice app. It uses `eframe` and `egui` for a native UI and generates timed math problems with increasing difficulty.

## Runtime Structure

```text
main()
  -> eframe native app
  -> MathQuizApp state
  -> egui update loop
  -> problem generation
  -> scoring and timer updates
```

## Core State

| Field | Purpose |
|---|---|
| `question` | Current prompt displayed to the user. |
| `answer` | Correct integer answer for the current problem. |
| `user_input` | Current text-entry value. |
| `score` | Total score. |
| `correct_answers` | Count of correct submissions. |
| `wrong_answers` | Count of incorrect or invalid submissions. |
| `remaining_time` | Countdown timer. |
| `start_time` | Timer reference point. |
| `feedback` | User-facing correctness message. |
| `game_over` | Controls game-over screen. |
| `is_pemdas` | Tracks whether current question is a complex expression. |

## UI Flow

1. App starts with an initial generated problem and a prompt to press Start.
2. Start begins the timer.
3. User types an answer and presses Enter.
4. The app validates numeric input.
5. Correct answers increase score and add time.
6. Incorrect or invalid answers subtract time.
7. Difficulty increases as score rises.
8. When time reaches zero, the game-over screen shows final stats.

## Extension Points

- Extract problem generation into a module.
- Add deterministic tests for generated expressions.
- Add difficulty presets.
- Persist high scores locally.
- Add keyboard-first restart and start controls.
- Add optional game modes for multiplication, division, or PEMDAS-only drills.

