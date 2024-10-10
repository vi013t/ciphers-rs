# `enigma-simulator`

An absurdly fast and highly flexible Enigma machine simulation, encryption, and decryption library for Rust.

The library features a safe and unsafe API. The unsafe API does not do correctness checks on the user input nor during the encryption/decryption process, but is marginally faster than the safe API.

## Usage

```bash
cargo add enigma-simulator
```

Example Usage:

```rust
use enigma_simulator::{EnigmaMachine, EnigmaResult, EnigmaBuilder as _};

pub fn main() -> EnigmaResult<()> {
    let machine = EnigmaMachine::new()
        .reflector("B")
        .plugboard("BY EW FZ GI QM RV UX")
        .rotors(1, 2, 3)
        .ring_settings(10, 12, 14)
        .ring_positions(5, 22, 3)?;
    let plaintext = machine.decode("KDZVKMNTYQJPHFXI");
    println!("{plaintext}");

    Ok(())
}
```

```bash
cargo run --release
```

### Unsafe API

Additionally, you can use `::unchecked()` to opt into the API without safety checks. This means that the Engima machine won't necessarily error when you create it with invalid settings, and may either error during encryption/decryption or simply produce an invalid encryption/decryption, but the construction of the machine will be faster as it bypasses all correctness checks:

```rust
use enigma_simulator::{EnigmaMachine, EnigmaResult, EnigmaBuilder as _};

pub fn main() -> EnigmaResult<()> {

    let machine = EnigmaMachine::unchecked()
        .reflector("B")
        .plugboard("BY EW FZ GI QM RV UX")
        .rotors(1, 2, 3)
        .ring_settings(10, 12, 14)
        .ring_positions(5, 22, 3)
        .build();

    Ok(())
}
```

Note that using `unchecked()` means any errors that *do* occur immediately will occur as a `panic!` instead of returning a `Result`.

## Utilities

`enigma-simulator` comes with a number of utilities relating to Enigma encryption and decryption.

- `.debug()` - When constructing an Enigma machine with the safe API, use `.debug()` to show debug information during encryption. This will print out what each letter changes to as it goes through each step of the encryption process.
- `.clear_punctuation()` - When constructing an Enigma machine with the safe API, use `.clear_punctuation()` to make it so that punctuation is removed in the output, instead of retained like with the default options.
- `.clear_casing()` - When constructing an Enigma machine with the safe API, use `.clear_casing()` to output the result in all capitals, instead of retaining the casing of the original message like with the default settings.

These options are only available in the safe API because the unsafe API is designed for maxmimum performance, and it'd slow it down to perform these checks during decryption of each character. The unsafe API is designed for brute-force cracking, so these kinds of options wouldn't be super useful anyway.

## Performance

**Note: Remember to use `release` mode when compiling for maximum performance. The difference between debug mode release mode can be more than 10x in speed.**

I recommend also using [cargo-wizard](https://github.com/Kobzol/cargo-wizard) to optimize your release profile for maximum runtime performance.

The Enigma cipher encodes/decodes one character at a time, so the process of encryption/decryption is in `O(n)` for a ciphertext/plaintext of length `n`. The rotors, ring settings, ring positions, plugboard, and reflector do not affect the algorithmic runtime of the machine, and changing them will cause virtually zero measurable difference in runtime.

This crate uses a number of optimization techniques for max performance, including:

- Memoization of certain functions that are called whenever an Enigma machine is created
- String storage as `&[u8]` for rapid indexing
- Unsafe API for when maximum performance is needed at the expense of fast crashes and good error messages

On my personal machine, with rudimentary benchmarks, `enigma-simulator` can construct enigma machines and encrypt 100-character messages over 44,000 times per second. To test this benchmark on your machine, run:

```bash
cargo test --test benchmarks --release -- --nocapture
```

Or, if you have [cargo-cmd](https://github.com/danreeves/cargo-cmd) installed:

```bash
cargo cmd benchmark
```