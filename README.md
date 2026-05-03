# gimlet-mollusk-svm-example

A minimal two-program Solana workspace for exercising and debugging sBPF programs via [mollusk-svm](https://github.com/anza-xyz/mollusk) and [Gimlet](https://marketplace.visualstudio.com/items?itemName=Limechain.gimlet).

`simple_anchor_app` makes a CPI into `cpi_target`. Both are written with [pinocchio](https://github.com/anza-xyz/pinocchio).

```
simple_anchor_app  ─── CPI ───▶  cpi_target
  5UDda9Uq56F75…                   HtH3m4682j9Dq9bG…
```

## Prerequisites

- Rust (stable, with `cargo`)
- Solana CLI / Agave toolchain (provides `cargo build-sbf`)

Verify:

```sh
solana --version
cargo build-sbf --version
```

## Build

```sh
cargo-build-sbf --tools-version v1.54 --debug --arch v1
```

> **Why this?** `cargo build-sbf` step builds with SBPFv1 (`--arch v1`), debug symbols, and no optimizations — required for accurate coverage results.

> **Note:** At the time of writing, best coverage results are achieved with SBPFv1 (dynamic stack frames), which is why we use `--arch v1`. Only with dynamic stack frames can we safely disable optimizations (`opt-level = 0`) without hitting stack size limits. The `--tools-version` can be v1.51 or higher, and `--debug` is required for coverage to work. Starting with `cargo-build-sbf` 4.0.0, the `--debug` flag outputs artifacts to `target/deploy/debug` instead of `target/deploy`. If you are using an older version of `cargo-build-sbf`, replace `target/deploy/debug` with `target/deploy` in all the steps.

## Run the test

```sh
SBF_OUT_DIR=$(pwd)/target/deploy/debug \
SBF_TRACE_DIR=$(pwd)/target/sbf/trace \
cargo test -p simple_anchor_app --features no-entrypoint --test cpi -- --nocapture
```

- `SBF_OUT_DIR` tells mollusk where to find the built `.so` files.
- `SBF_TRACE_DIR` is where the sBPF VM writes execution traces consumed by Gimlet.

This runs the mollusk CPI test and prints the program logs. Expect to see `simple_anchor_app invoke [1]` → `cpi_target invoke [2]` → `Ping received by cpi_target!` and both programs returning `success`.

## Debug a program

```sh
SBF_OUT_DIR=$(pwd)/target/deploy/debug \
SBF_DEBUG_PORT=1212 \
SBF_TRACE_DIR=$(pwd)/target/sbf/trace \
cargo test -p simple_anchor_app --features no-entrypoint --test cpi -- --nocapture
```

`SBF_DEBUG_PORT=1212` makes the sBPF VM **open TCP port `1212` and block waiting for a debugger to attach** before stepping into the program.

## Clean

```sh
cargo clean
rm -rf programs/simple_anchor_app/target
```

Wipes `target/` and any stray nested `target/` left by prior runs, if any.

## Layout

```
.
├── Cargo.toml                                  # workspace
└── programs/
    ├── cpi_target/                             # callee program
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── simple_anchor_app/                      # caller program (does the CPI)
        ├── Cargo.toml
        ├── src/lib.rs
        └── tests/cpi.rs                        # mollusk-svm test
```
