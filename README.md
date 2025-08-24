# poker-tx-holdem

A small Rust library that determines the winning hand between two Texas Hold'em players from the first 9 cards of a deck permutation (1–52). It returns the winner's **best 5-card hand** in the format `["<rank><SUIT>", ...]`, with suits `C/D/H/S` and numeric faces (`11=J`, `12=Q`, `13=K`, `1=A`).

> This follows the exact I/O required by the original assignment: `pub fn deal(perm: [u32; 9]) -> Vec<String>`.

## Quick start

```bash
cargo test
```

## Example

```rust
use poker_tx_holdem::deal;

let perm: [u32; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];
let winner = deal(perm);
assert_eq!(winner, vec!["2C","3C","4C","5C","6C"]);
```

## Assignment Background

This project comes from a **C/CPS506 programming assignment**, where students were asked to solve the same task in two of several studied languages (Rust was one option).

- See [ASSIGNMENT.md](ASSIGNMENT.md) for the original course instructions (general and Rust-specific requirements).

### General Description
The program must:
- Deal **two 2-card poker hands** plus a **5-card shared pool** (flop, turn, river).
- Evaluate the strength of each hand according to **Texas Hold’em** rankings.
- **Return the winning hand** as a list of 5 cards.

References provided to students for rules and hand rankings:
- [PartyPoker: How to Play Texas Hold’em](https://www.partypoker.com/en/how-to-play/texas-holdem)  
- [FG Bradley’s Poker Hand Rankings](https://www.fgbradleys.com/et_poker.asp)  

### Constraints
- No third-party libraries (only standard language features).
- Free to use helper functions and idiomatic structures.
- For Rust, the required interface was strictly:

```rust
pub fn deal(perm: [u32; 9]) -> Vec<String>;
```

Input is the first **9 cards of a shuffled 52-card deck** (`1..=52`).  
Output is the winner’s **best 5-card hand**, e.g. `["2C", "3C", "4C", "5C", "6C"]`.

## Development
- `cargo fmt` – formats the code
- `cargo clippy -- -D warnings` – lints (warnings are denied in CI)
- `cargo test` – runs unit tests from `tests/`

## License
MIT
