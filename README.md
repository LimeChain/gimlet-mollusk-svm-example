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
- `make`

Verify:

```sh
solana --version
cargo build-sbf --version
```

## Build

```sh
make build
```

Produces:
- `target/deploy/simple_anchor_app.so`
- `target/deploy/cpi_target.so`

## Run the test

```sh
make test
```

This runs the mollusk CPI test and prints the program logs. Expect to see `simple_anchor_app invoke [1]` → `cpi_target invoke [2]` → `Ping received by cpi_target!` and both programs returning `success`.

## Debug a program

```sh
make debug
```

`make debug` runs the same test but with `SBF_DEBUG_PORT=1212` set. The sBPF VM **opens TCP port `1212` and blocks waiting for a debugger to attach** before stepping into the program.

## Clean

```sh
make clean
```

Wipes `target/` and any stray nested `target/` left by prior runs.

## Layout

```
.
├── Cargo.toml                                  # workspace
├── Makefile                                    # build / test / debug
└── programs/
    ├── cpi_target/                             # callee program
    │   ├── Cargo.toml
    │   └── src/lib.rs
    └── simple_anchor_app/                      # caller program (does the CPI)
        ├── Cargo.toml
        ├── src/lib.rs
        └── tests/cpi.rs                        # mollusk-svm test
```
