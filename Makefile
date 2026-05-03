all:

clean:
	@cargo clean
	@rm -rf $(CURDIR)/programs/simple_anchor_app/target

build:
	@cargo-build-sbf --tools-version v1.54 --debug --arch v1

test:
	@SBF_OUT_DIR=$(CURDIR)/target/deploy/debug \
	SBF_TRACE_DIR=$(CURDIR)/target/sbf/trace \
	cargo test -p simple_anchor_app --features no-entrypoint --test cpi -- --nocapture

debug:
	@SBF_OUT_DIR=$(CURDIR)/target/deploy/debug \
	SBF_DEBUG_PORT=1212 \
	SBF_TRACE_DIR=$(CURDIR)/target/sbf/trace \
	cargo test -p simple_anchor_app --features no-entrypoint --test cpi -- --nocapture
