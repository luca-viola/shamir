# Shamir Secret Sharing - Rust Implementation

This is a Rust implementation of Shamir Secret Sharing, converted from the original Python version.

## Overview

Shamir Secret Sharing is a cryptographic algorithm that allows you to split a secret (like a password) into multiple shares. A minimum threshold of shares is required to reconstruct the original secret. This implementation uses Mersenne primes for enhanced security.

## Features

- Split secrets into multiple shares with configurable threshold
- Reconstruct secrets from minimum required shares
- Base93 encoding for input (supports extended character set)
- Base62 encoding for output shares (URL/JSON safe)
- Clipboard integration for easy secret recovery
- Secure password input (hidden from terminal)
- Uses Mersenne prime 2^521-1 for cryptographic operations

## Building

```bash
cargo build --release
```

## Installation

After building, the binary will be available at `target/release/shamir`. You can copy it to your PATH:

```bash
cp target/release/shamir /usr/local/bin/
```

## Usage

### Generate shares (interactive mode)

```bash
shamir
```

This will prompt you for a password twice and generate 5 shares with a minimum threshold of 3.

### Generate shares with custom parameters

```bash
shamir -t 3 -s 5 -k "mypassword"
```

Options:
- `-t, --threshold <MINIMUM>`: Minimum number of shares needed to reconstruct the secret (default: 3)
- `-s, --shares <SHARES>`: Total number of shares to generate (default: 5)
- `-k, --key <KEY>`: The secret/password to split into shares

### Reconstruct secret from shares

```bash
shamir -r 3
```

This will prompt you to enter 3 shares (one per line) and will copy the reconstructed secret to your clipboard.

## Example

```bash
$ shamir -k "test123" -t 3 -s 5
Generating 5 shares with a minimum of 3 shares required
shares:
  1-Xr3WGQLPw466ELcw14kyxUGzmdhAy3IkHwrigR7qiRxPgfhCtXi8mIawzNIqYcGy5oAy3Drljjcng0hMTGslcaqO2TyoC9Lu4oqGgW
  2-ikLSPDUmNjYjzEHTxzbmF4XoP0LTQSPU4APt3IJld65iuBVUBGU0TfXBNWye3LxRtXLmGXZG4iTATLWZLNVL8bTc0yx0Qvem9ahmSW
  3-WfroQNS5L0NvGdxdqkYNqkoRr5wtNDKDKeeV6ZZkjwOveXQprAJb66ohAT1OUD3RPBWQdx6T0wX6O0TccJriY1tfvUuaiL4ngYOZcY
  4-l1r70gqdwulUYycRt8z2JAwA5zCpgY6b6KyclMvEBOx02AcZuQQSvunKM0oaaoM73twkCI6fSxX46LP9FXeNnZO80ZTigd7vo8Do4f
  5-cQ4pTN394QVbNqIrrNVV5h3hAbPtUEgyMHzC6aKlp3fxwdvOLrb3gn7EwKyjdX4IQXMtuiwaWDkaw0SWHbAqwWfQ1f2EhYIDNtsJuo
Minimum shares reconstruction test passed, generation complete.
```

## Dependencies

- `clap`: Command-line argument parsing
- `clipboard`: Clipboard operations
- `rpassword`: Secure password input
- `rand`: Random number generation
- `num-bigint`: Large integer arithmetic
- `num-traits`: Numeric traits for BigUint
- `regex`: Regular expression matching

## Technical Details

The implementation uses:
- Lagrange interpolation for secret reconstruction
- Extended Euclidean algorithm for modular division
- Mersenne prime 2^8191-1 (2^13th Mersenne prime) as the field prime
- Base93 encoding for input (0-9A-Za-z plus special characters)
- Base62 encoding for output shares (0-9A-Za-z only)

## License

See LICENSE file.

## Author

Original Python version by Luca Viola (V2019.1)
Rust conversion: 2026
