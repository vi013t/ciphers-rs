# `ciphers-rs`

A massive collection of classic cryptographic tools for Rust.

## General Purpose tools

There is [a utilities crate](https://github.com/vi013t/ciphers-rs/tree/main/crates/cipher-utils) that provides various general classical cryptographic analysis utilities, such as index of coincidence, frequency analysis, etc. 

Additionally, there is [a master "cipher-cracker" crate](https://github.com/vi013t/ciphers-rs/tree/main/crates/cipher-cracker) that identifies cipher types and delegates the decryption to a determined cipher type.

## Ciphers

The following describes the supported and planned cipher types:

- [ ] A1Z26
- [ ] ADFGX
- [ ] ADFGVX
- [ ] Affine
- [ ] Atbash
- [ ] Baconian
- [ ] Base 64
- [ ] Beaufort
- [ ] Bifid
- [ ] Caeser
- [ ] Columnar Transposition
- [x] Enigma M3
- [ ] Enigma M4
- [ ] Fractionated Morse
- [x] Gronsfeld
- [ ] Hex
- [ ] Hill
- [x] Morse Code
- [ ] Navajo Code Talker
- [ ] Octal
- [ ] One-Time Pad
- [ ] Playfair
- [ ] Polybius Square
- [ ] Porta
- [ ] Purple
- [ ] Rail Fence
- [ ] Rot-13
- [ ] Running Key 
- [ ] Scytale
- [ ] Simplified Lorenz
- [ ] Straddling Checkerboard
- [ ] Trifid
- [ ] Trithemius
- [ ] Ubchi
- [ ] Vigenere

Additionally, there is [a utilities crate](https://github.com/vi013t/ciphers-rs/tree/main/crates/cipher-utils) that provides various general classical cryptographic analysis utilities, such as index of coincidence, frequency analysis, etc. 