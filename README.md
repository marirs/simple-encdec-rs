# Simple Encrypt / Decrypt
[![Linux Arm7](https://github.com/marirs/simple-encdec-rs/actions/workflows/linux_arm7.yml/badge.svg)](https://github.com/marirs/simple-encdec-rs/actions/workflows/linux_arm7.yml)
[![Linux x86_64](https://github.com/marirs/simple-encdec-rs/actions/workflows/linux_x86-64.yml/badge.svg)](https://github.com/marirs/simple-encdec-rs/actions/workflows/linux_x86-64.yml)
[![macOS](https://github.com/marirs/simple-encdec-rs/actions/workflows/macos.yml/badge.svg)](https://github.com/marirs/simple-encdec-rs/actions/workflows/macos.yml)
[![Windows](https://github.com/marirs/simple-encdec-rs/actions/workflows/windows.yml/badge.svg)](https://github.com/marirs/simple-encdec-rs/actions/workflows/windows.yml)

A very simple rust library to encrypt and decrypt strings.

### Requirements
- Rust 1.56+ (edition 2021)

### Compile
```bash
cargo b --release
```

### Example

- Encrypt
```bash
use simple_encdec::encrypt;

let x = encrypt("hello world");
assert!(x.is_some());
assert_eq!(x.unwrap(), "a=GQVGsbbyG982gd");
```

- Decrypt
```bash
use simple_encdec::decrypt;

let x = decrypt("a=GQVGsbbyG982gd");
assert!(x.is_some());
assert_eq!(x.unwrap(), "hello world");
```

---
License: MIT