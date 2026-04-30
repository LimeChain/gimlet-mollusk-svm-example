all:

clean:
	@cargo clean

prepare:
	@anchor keys sync
	@anchor build
	@cd programs/cpi_target && RUSTFLAGS="-Copt-level=0 -C strip=none -C debuginfo=2" cargo build-sbf --tools-version v1.54 --arch v1 --debug
	@cd programs/simple_anchor_app && RUSTFLAGS="-Copt-level=0 -C strip=none -C debuginfo=2" cargo build-sbf --tools-version v1.54 --arch v1 --debug

surfpool:
	@SBF_DEBUG_PORT=1212 SBF_TRACE_DIR=$(PWD)/target/sbf/trace surfpool-debugger start --artifacts-path ./target/deploy/debug --watch --no-tui --disable-instruction-profiling

debug:
	@anchor test --skip-local-validator --skip-build --skip-deploy
