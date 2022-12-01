# Advent of Code 2022

My solutions for Advent of Code 2022. Written in Rust 🦀.

- Clone the repository.
- Make sure you have a nightly version of Rust (written around December 2022).
- `cargo run --release` for all days, `cargo run --release -- NN` for a specific
  day.
- Want your own inputs?
    - **Auto-download:** Delete the `inputs` directory, then create a
      `session_key.txt` file containing your AoC website's session cookie value.
    - **Manually:** Replace the contents of a `inputs/NN.txt` file with your
      desired input.
- Benchmarks? 🚤
    - `cargo bench --features "criterion"`
    - optionally add `-- dayNN` at the end, to run a specific day!
