SHELL = /bin/bash

# Will strip strings from distribution binary.
# 0=disable
# 1=enable
STRIP_DIST_BIN:=0

# Will compress distribution binary.
# 0=disable
# 1=enable
COMPRESS_DIST_BIN:=0

# Create and copy the debug build into distribution folder.
# 0=disable
# 1=enable
BUILD_DEBUG:=1

APP_NAME=$(shell grep -E '^\s*name\s*=' Cargo.toml | grep -o '".*"' | tr -d '"')
APP_DEBUG_NAME=$(APP_NAME)d
APP_VERSION=$(shell grep -E '^\s*version\s*=' Cargo.toml | grep -o '".*"' | tr -d '"')

PACKAGE_SRC_DIR:=./build
PACKAGE_DEST_DIR:=./dist
PACKAGE_PATH:=$(PACKAGE_DEST_DIR)/$(APP_NAME)_$(APP_VERSION).tar.gz
IMG_SRC_DIR:=./img

UPX_FLAGS:=--best
TEST_FLAGS:=
CLIPPY_FLAGS:=-D warnings
CLIPPY_PEDANTIC_FLAGS:=-W clippy::pedantic
BUILD_RELEASE_FLAGS:=
BUILD_DEBUG_FLAGS:=
# DOC_FLAGS:=--open
DOC_FLAGS:=

.DEFAULT_GOAL := default

default: all

all: check test clippy debug release doc dist

check:
	cargo fmt -- --check

test:
	cargo test -- $(TEST_FLAGS)

clippy:
	cargo clippy -- $(CLIPPY_FLAGS)

pedantic:
	cargo clippy -- $(CLIPPY_PEDANTIC_FLAGS)

debug:
	if [ $(BUILD_DEBUG) -eq 1 ]; then \
		cargo build $(BUILD_DEBUG_FLAGS); \
	fi

release:
	cargo build --release $(BUILD_RELEASE_FLAGS)

doc:
	cargo doc $(DOC_FLAGS)

clean:
	cargo clean

dist: distclean readme
	mkdir -p "$(PACKAGE_DEST_DIR)/img"
	cp "$(PACKAGE_SRC_DIR)/"* "$(PACKAGE_DEST_DIR)/"
	cp "./target/release/$(APP_NAME)" "$(PACKAGE_DEST_DIR)/"
	cp "./LICENSE" "$(PACKAGE_DEST_DIR)/"
	cp "$(IMG_SRC_DIR)/enjoy_logo.svg" "$(PACKAGE_DEST_DIR)/img"
	if [ $(STRIP_DIST_BIN) -eq 1 ]; then \
		strip "$(PACKAGE_DEST_DIR)/$(APP_NAME)"; \
	fi
	if [ $(COMPRESS_DIST_BIN) -eq 1 ]; then \
		upx $(UPX_FLAGS) "$(PACKAGE_DEST_DIR)/$(APP_NAME)"; \
	fi
	tar -cf "$(PACKAGE_PATH)" "$(PACKAGE_DEST_DIR)/"*
	if [ $(BUILD_DEBUG) -eq 1 ]; then \
		cp "./target/debug/$(APP_NAME)" "$(PACKAGE_DEST_DIR)/$(APP_DEBUG_NAME)"; \
		gzip -f "$(PACKAGE_DEST_DIR)/$(APP_DEBUG_NAME)"; \
	fi

readme:
	mkdir -p "$(PACKAGE_DEST_DIR)"
	pandoc "./README.md" -o "$(PACKAGE_DEST_DIR)/README.html"

distclean:
	-cd "$(PACKAGE_DEST_DIR)/img" && rm -f *
	-rm -d -f "$(PACKAGE_DEST_DIR)/img"
	-cd "$(PACKAGE_DEST_DIR)" && rm -f *
	-rm -d -f "$(PACKAGE_DEST_DIR)"

install:
	cd "$(PACKAGE_DEST_DIR)" && "./install.sh"

uninstall:
	cd "$(PACKAGE_SRC_DIR)" && "./uninstall.sh"

