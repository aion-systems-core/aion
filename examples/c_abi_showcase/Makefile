# Build all native examples (POSIX). Requires: cargo, cc, make, go in PATH.

ROOT := $(abspath ../..)
LIBDIR ?= $(ROOT)/target/debug

.PHONY: all engine c cpp go python

all: engine c cpp go python

engine:
	cd $(ROOT) && cargo build -p aion-engine

c: engine
	$(MAKE) -C c LIBDIR=$(LIBDIR)

cpp: engine
	$(MAKE) -C cpp LIBDIR=$(LIBDIR)

go: engine
	cd go && AION_LIB_PATH=$(LIBDIR) go build -o showcase_go .

python: engine
	AION_LIB_PATH=$(LIBDIR) python3 python/demo.py
