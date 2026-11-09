# rbitcoin module

Upstream: [rbitcoin](https://github.com/rbitcoin-org/rbitcoin) (`rbitcoin-consensus`)

Differential oracles for `script_eval` and `verify_script` against Bitcoin Core
and other modules.

Note: Bitcoin Core's harness uses a fuzzed signature checker (CHECKSIG always
passes). This module runs real ECDSA verification, so scripts that hit CHECKSIG
may disagree with `BITCOIN_CORE` until a shared mock checker exists.

## Dependencies

- Rust toolchain `1.95.0` (`rustup toolchain install 1.95.0`)
- Git (to fetch upstream if `external/rbitcoin` is missing)

Override the tree with `RBITCOIN_DIR=/path/to/rbitcoin`.

## Build

```bash
cd modules/rbitcoin
# optional: export RBITCOIN_DIR=/path/to/rbitcoin
make
export CXXFLAGS="$CXXFLAGS -DRBITCOIN"
```

## Smoke (no Core / libFuzzer)

```bash
cd modules/rbitcoin
make poc-smoke
```

## Run differential (Linux / Docker)

Build Bitcoin Core + rbitcoin modules, then:

```bash
# from repo root
export CXXFLAGS="-DBITCOIN_CORE -DRBITCOIN"
# build modules/bitcoin, modules/rbitcoin, then:
make
MODULES="BITCOIN_CORE,RBITCOIN" FUZZ=script_eval ./bitcoinfuzz
MODULES="BITCOIN_CORE,RBITCOIN" FUZZ=verify_script ./bitcoinfuzz
```

Or via compose services `script_eval` / `verify_script` (include `-DRBITCOIN`).
