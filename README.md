# Simple Anchor App

A simple Anchor application for Solana with code coverage support using Surfpool.

## Prerequisites

- [Anchor](https://www.anchor-lang.com/)
- [Surfpool](https://github.com/solana-foundation/surfpool) (register-tracing is enabled by default)
- [sbpf-coverage](https://crates.io/crates/sbpf-coverage)
- [lcov](https://github.com/linux-test-project/lcov) - for generating HTML coverage reports (`brew install lcov` on macOS, `apt install lcov` on Ubuntu, or `dnf install lcov` on Fedora)

## Build

### Get Surfpool

Install Surfpool following the instructions at [github.com/solana-foundation/surfpool](https://github.com/solana-foundation/surfpool). Register-tracing is enabled by default, so no special feature flags are needed.

### Build the Anchor project

```bash
anchor build
RUSTFLAGS="-Copt-level=0 -C strip=none -C debuginfo=2" cargo build-sbf --tools-version v1.54 --arch v1 --debug
```

> **Why two build steps?** `anchor build` generates the IDL. The second `cargo build-sbf` step rebuilds with SBPFv1 (`--arch v1`), debug symbols, and no optimizations — required for accurate coverage results.

> **Note:** At the time of writing, best coverage results are achieved with SBPFv1 (dynamic stack frames), which is why we use `--arch v1`. Only with dynamic stack frames can we safely disable optimizations (`opt-level = 0`) without hitting stack size limits. The `--tools-version` can be v1.51 or higher, and `--debug` is required for coverage to work. Starting with `cargo-build-sbf` 4.0.0, the `--debug` flag outputs artifacts to `target/deploy/debug` instead of `target/deploy`. If you are using an older version of `cargo-build-sbf`, replace `target/deploy/debug` with `target/deploy` in all the steps.

## Run Tests

### Start Surfpool

Run this in the anchor project directory:

For code coverage only:

```bash
SBF_TRACE_DIR=$PWD/target/sbf_trace_dir surfpool start --artifacts-path ./target/deploy/debug --watch
```

For code coverage with trace disassembly output:

```bash
SBF_TRACE_DISASSEMBLE=true SBF_TRACE_DIR=$PWD/target/sbf_trace_dir surfpool start --artifacts-path ./target/deploy/debug --watch
```

> **Note:** Setting `SBF_TRACE_DIR` is what signals `LiteSVM` to enable register tracing dumps. Adding `SBF_TRACE_DISASSEMBLE=true` additionally produces trace disassembly output.

### Run Anchor tests

Use Surfpool instead of solana-test-validator:

```bash
anchor test --skip-local-validator --skip-build --skip-deploy
```

## Generate Coverage

Install sbpf-coverage if not already installed:

```bash
cargo install sbpf-coverage
```

Generate and view coverage report:

```bash
sbpf-coverage --src-path=$PWD/programs/simple_anchor_app/src --sbf-path=$PWD/target/deploy/debug --sbf-trace-dir=$PWD/target/sbf_trace_dir
genhtml --output-directory coverage target/sbf_trace_dir/*.lcov --rc branch_coverage=1 && open coverage/index.html
```

## Trace Disassembly

Install sbpf-coverage if not already installed:

```bash
cargo install sbpf-coverage
```

Generate and view trace disassembly:

```bash
sbpf-coverage --src-path=$PWD/programs/simple_anchor_app/src --sbf-path=$PWD/target/deploy/debug --sbf-trace-dir=$PWD/target/sbf_trace_dir --trace-disassemble
```

## License

[MIT](LICENSE) © [LimeChain](https://limechain.tech)
