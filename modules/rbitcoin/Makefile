all: module.a

CXXFLAGS += -Wall -Wextra -O2 -std=c++20 -I ../../include

# Upstream tree (clone or symlink). Override for a local checkout.
RBITCOIN_DIR ?= $(abspath ../../external/rbitcoin)
RBITCOIN_URL ?= https://github.com/rbitcoin-org/rbitcoin.git
RUSTUP_TOOLCHAIN ?= 1.95.0
# Host target without forcing nightly sanitizers onto this crate yet —
# ASan on the C++ side still covers the driver; Rust lib is release-clean for PoC.
RUST_TARGET ?= $(shell rustc +$(RUSTUP_TOOLCHAIN) -vV | sed -n 's/^host: //p')
RUST_LIB_PATH := ./rbitcoin_lib/target/$(RUST_TARGET)/release/librbitcoin_lib.a

SYMBOL_PREFIX ?= rbitcoin

# Prefer PATH objcopy; fall back to Homebrew LLVM on macOS.
OBJCOPY ?= $(shell command -v objcopy 2>/dev/null || command -v llvm-objcopy 2>/dev/null || ls /opt/homebrew/opt/llvm/bin/llvm-objcopy 2>/dev/null)

ensure-rbitcoin:
	@if [ ! -f "$(RBITCOIN_DIR)/crates/rbitcoin-consensus/Cargo.toml" ]; then \
		echo "Fetching rbitcoin into $(RBITCOIN_DIR)..."; \
		mkdir -p "$(dir $(RBITCOIN_DIR))"; \
		git clone --depth 1 "$(RBITCOIN_URL)" "$(RBITCOIN_DIR)"; \
	fi
	@mkdir -p ../../external
	@# Cargo.toml path is fixed at external/rbitcoin; link override checkouts there.
	@if [ "$(abspath $(RBITCOIN_DIR))" != "$(abspath ../../external/rbitcoin)" ]; then \
		ln -sfn "$(abspath $(RBITCOIN_DIR))" ../../external/rbitcoin; \
	fi

$(RUST_LIB_PATH): ensure-rbitcoin
	cd rbitcoin_lib && \
	CARGO_TARGET_DIR="$(abspath rbitcoin_lib/target)" \
	cargo +$(RUSTUP_TOOLCHAIN) build --release --target $(RUST_TARGET)

cargo: $(RUST_LIB_PATH)

module.a: module.o $(RUST_LIB_PATH)
	bash ../../scripts/merge.sh module.a $(RUST_LIB_PATH) module.o
	@if [ -n "$(OBJCOPY)" ] && [ -f ../../scripts/rename_rust_secp_symbols.sh ]; then \
		OBJCOPY="$(OBJCOPY)" bash ../../scripts/rename_rust_secp_symbols.sh module.a $(SYMBOL_PREFIX); \
	else \
		echo "Skipping rust secp rename (objcopy not found)"; \
	fi
	ranlib module.a

module.o: module.cpp module.h
	$(CXX) $(CXXFLAGS) -I . -fPIC -c module.cpp -o module.o

format:
	clang-format -i ./module.cpp ./module.h ./rbitcoin_lib/rbitcoin_lib.h
	cd rbitcoin_lib && cargo +$(RUSTUP_TOOLCHAIN) fmt

check-format:
	clang-format -Werror --fail-on-incomplete-format -n ./module.cpp ./module.h ./rbitcoin_lib/rbitcoin_lib.h
	cd rbitcoin_lib && cargo +$(RUSTUP_TOOLCHAIN) fmt --check

# Standalone smoke (no Bitcoin Core / libFuzzer). Proves FFI + oracle.
BASEMODULE_O := ../../include/bitcoinfuzz/basemodule.o

$(BASEMODULE_O):
	$(MAKE) -C ../.. include/bitcoinfuzz/basemodule.o

poc-smoke: module.a poc_smoke.cpp $(BASEMODULE_O)
	$(CXX) $(CXXFLAGS) -I . poc_smoke.cpp module.a $(BASEMODULE_O) -o poc_smoke \
		-framework Security -framework SystemConfiguration
	./poc_smoke

clean:
	rm -rf *.o module.a poc_smoke
	cd rbitcoin_lib && cargo +$(RUSTUP_TOOLCHAIN) clean

.PHONY: all cargo clean format check-format ensure-rbitcoin poc-smoke $(RUST_LIB_PATH)
