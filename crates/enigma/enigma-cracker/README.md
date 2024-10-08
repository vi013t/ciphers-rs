# `enigma-cracker`

A start-from-nothing Enigma cipher decryption library for Rust.

`enigma-cracker` finds the most likely rotor settings, ring settings, and plugboard by brute forcing them one at a time and performing various cryptographic analysis techniques on the results.

Naturally, the crate can't perfectly identify the "correct" plaintext, so it relies on statistics like index of coincidence; Thus, it'll be more accurate with longer ciphertexts.

## Usage

```bash
cargo add enigma-cracker
```

```rust
use enigma_cracker::crack_enigma;

fn main() -> EnigmaResult<()> {

	let ciphertext = include_str!("cipher_file.txt");
	let plaintext = crack_enigma(ciphertext);

	Ok(())
}
```

```bash
cargo run --release
```

## Performance

The performance of this crate varies wildly by ciphertext length; Since Enigma machines decrypt character by character, the decryption process is `O(n)`. This crate needs to perform several million decryptions, so longer ciphertexts can drastically increase runtime.

**Make sure you run in release mode; The difference between debug and release mode can be over 10x in speed.**

I also recommend using [cargo-wizard](https://github.com/Kobzol/cargo-wizard.git) to optimize your release profile for maximum runtime performance.