# ARCH
# CARGO
# OS
# PACKAGE
# RELEASE
# STRIP
# TARGET
# VERSION

ARCH ?= x86_64
CARGO ?= cargo
OS ?= macos
PACKAGE ?= fip_api
# RELEASE ?=
RUSTUP ?= rustup
STRIP ?= strips
TARGET ?= $(shell $(RUSTUP) show |sed -n 's/^Default host: \(.*\)/\1/p')
VERSION ?= 0.1.0

TARGET_DIR = target/$(TARGET)/debug
ifdef RELEASE
	RELEASE = --release
	TARGET_DIR = target/$(TARGET)/release
endif
BIN = $(TARGET_DIR)/$(PACKAGE)

# PACKAGE_ROOT = $(__TARGET__)/package
BIN_NAME = $(PACKAGE)-$(VERSION)-$(ARCH)
# PACKAGE_BASE = $(PACKAGE_ROOT)/$(BIN_NAME)

SHASUM = shasum -a 256

CARGO_BENCH = $(CARGO) bench --all-features --frozen --no-default-features --package $(PACKAGE) --target $(TARGET)
CARGO_BUILD = $(CARGO) build --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_CHECK = $(CARGO) check --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_CLEAN = $(CARGO) clean --frozen --package $(PACKAGE) $(RELEASE) --target $(TARGET) #--target-dir $(TARGET_DIR)
CARGO_DOC = $(CARGO) doc --all-features --frozen --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_FETCH = $(CARGO) fetch --locked --target $(TARGET)
CARGO_TEST = $(CARGO) test --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_VENDOR = $(CARGO) vendor

CARGO_AUDIT = $(CARGO) audit --target-arch $(ARCH) --target-os $(OS)
CARGO_CLIPPY = $(CARGO) clippy --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET) -- -D warnings
CARGO_DENY = $(CARGO) deny --all-features --no-default-features --workspace
CARGO_FMT = $(CARGO) fmt --package $(PACKAGE)

$(BIN): add-fmt add-target fetch
	$(CARGO_BUILD)

.PHONY: add-audit
add-audit:
	$(CARGO) install cargo-audit

.PHONY: add-clippy
add-clippy:
	$(RUSTUP) component add clippy

.PHONY: add-deny
add-deny:
	$(CARGO) install cargo-deny

.PHONY: add-fmt
add-fmt:
	$(RUSTUP) component add rustfmt

.PHONY: add-target
add-target:
	$(RUSTUP) target add $(TARGET)

.PHONY: all
all: fmt-check check

.PHONY: audit
audit: add-audit
	$(CARGO_AUDIT)

.PHONY: bench
bench: add-fmt add-target fetch
	$(CARGO_BENCH)

.PHONY: build
build: $(BIN)

.PHONY: check
check: add-fmt add-target fetch
	$(CARGO_CHECK)

.PHONY: clean
clean: add-target
	$(CARGO_CLEAN)

.PHONY: clean-doc
clean-doc: add-target
	$(CARGO_CLEAN) --doc

.PHONY: clippy
clippy: add-clippy add-fmt fetch
	$(CARGO_CLIPPY)

.PHONY: deny-check
deny-check: add-deny fetch
	$(CARGO_DENY) check

.PHONY: deny-fetch
deny-fetch: add-deny fetch
	$(CARGO_DENY) fetch

.PHONY: deny-fix
deny-fix: add-deny fetch
	$(CARGO_DENY) fix

.PHONY: doc
doc: add-fmt add-target fetch
	$(CARGO_DOC)

.PHONY: fetch
fetch: Cargo.lock
	$(CARGO_FETCH)

.PHONY: fmt
fmt: add-fmt
	$(CARGO_FMT)

.PHONY: fmt-check
fmt-check: add-fmt
	$(CARGO_FMT) -- --check

.PHONY: release
release: $(BIN)
	@mkdir -p release
	cp $(BIN) release/$(BIN_NAME)
	$(STRIP) release/$(BIN_NAME)
	$(SHASUM) release/$(BIN_NAME) > release/$(BIN_NAME).shasum

.PHONY: test
test: add-fmt add-target fetch
	$(CARGO_TEST)
# ARCH
# CARGO
# OS
# PACKAGE
# RELEASE
# STRIP
# TARGET
# VERSION

ARCH ?= x86_64
CARGO ?= cargo
OS ?= macos
PACKAGE ?= fip_api
# RELEASE ?=
RUSTUP ?= rustup
STRIP ?= strips
TARGET ?= $(shell $(RUSTUP) show |sed -n 's/^Default host: \(.*\)/\1/p')
VERSION ?= 0.1.0

TARGET_DIR = target/$(TARGET)/debug
ifdef RELEASE
	RELEASE = --release
	TARGET_DIR = target/$(TARGET)/release
endif
BIN = $(TARGET_DIR)/$(PACKAGE)

# PACKAGE_ROOT = $(__TARGET__)/package
BIN_NAME = $(PACKAGE)-$(VERSION)-$(ARCH)
# PACKAGE_BASE = $(PACKAGE_ROOT)/$(BIN_NAME)

SHASUM = shasum -a 256

CARGO_BENCH = $(CARGO) bench --all-features --frozen --no-default-features --package $(PACKAGE) --target $(TARGET)
CARGO_BUILD = $(CARGO) build --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_CHECK = $(CARGO) check --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_CLEAN = $(CARGO) clean --frozen --package $(PACKAGE) $(RELEASE) --target $(TARGET) #--target-dir $(TARGET_DIR)
CARGO_DOC = $(CARGO) doc --all-features --frozen --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_FETCH = $(CARGO) fetch --locked --target $(TARGET)
CARGO_TEST = $(CARGO) test --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET)
CARGO_VENDOR = $(CARGO) vendor

CARGO_AUDIT = $(CARGO) audit --target-arch $(ARCH) --target-os $(OS)
CARGO_CLIPPY = $(CARGO) clippy --all-features --frozen --no-default-features --package $(PACKAGE) $(RELEASE) --target $(TARGET) -- -D warnings
CARGO_DENY = $(CARGO) deny --all-features --no-default-features --workspace
CARGO_FMT = $(CARGO) fmt --package $(PACKAGE)

$(BIN): add-fmt add-target fetch
	$(CARGO_BUILD)

.PHONY: add-audit
add-audit:
	$(CARGO) install cargo-audit

.PHONY: add-clippy
add-clippy:
	$(RUSTUP) component add clippy

.PHONY: add-deny
add-deny:
	$(CARGO) install cargo-deny

.PHONY: add-fmt
add-fmt:
	$(RUSTUP) component add rustfmt

.PHONY: add-target
add-target:
	$(RUSTUP) target add $(TARGET)

.PHONY: all
all: fmt-check check

.PHONY: audit
audit: add-audit
	$(CARGO_AUDIT)

.PHONY: bench
bench: add-fmt add-target fetch
	$(CARGO_BENCH)

.PHONY: build
build: $(BIN)

.PHONY: check
check: add-fmt add-target fetch
	$(CARGO_CHECK)

.PHONY: clean
clean: add-target
	$(CARGO_CLEAN)

.PHONY: clean-doc
clean-doc: add-target
	$(CARGO_CLEAN) --doc

.PHONY: clippy
clippy: add-clippy add-fmt fetch
	$(CARGO_CLIPPY)

.PHONY: deny-check
deny-check: add-deny fetch
	$(CARGO_DENY) check

.PHONY: deny-fetch
deny-fetch: add-deny fetch
	$(CARGO_DENY) fetch

.PHONY: deny-fix
deny-fix: add-deny fetch
	$(CARGO_DENY) fix

.PHONY: doc
doc: add-fmt add-target fetch
	$(CARGO_DOC)

.PHONY: fetch
fetch: Cargo.lock
	$(CARGO_FETCH)

.PHONY: fmt
fmt: add-fmt
	$(CARGO_FMT)

.PHONY: fmt-check
fmt-check: add-fmt
	$(CARGO_FMT) -- --check

.PHONY: release
release: $(BIN)
	@mkdir -p release
	cp $(BIN) release/$(BIN_NAME)
	$(STRIP) release/$(BIN_NAME)
	$(SHASUM) release/$(BIN_NAME) > release/$(BIN_NAME).shasum

.PHONY: test
test: add-fmt add-target fetch
	$(CARGO_TEST)
