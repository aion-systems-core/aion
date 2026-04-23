ROOT := $(abspath ../../..)
LIBDIR ?= $(ROOT)/target/debug
INCDIR := $(ROOT)/include

CFLAGS := -Wall -Wextra -O2 -I$(INCDIR)

UNAME_S := $(shell uname -s 2>/dev/null || echo unknown)
ifeq ($(UNAME_S),Darwin)
	LIBNAME := libaion_engine.dylib
	LDFLAGS := -L$(LIBDIR) -laion_engine -rpath $(LIBDIR)
else
	LIBNAME := libaion_engine.so
	ifeq ($(OS),Windows_NT)
		LIBNAME := aion_engine.dll
		LDFLAGS := -L$(LIBDIR) -laion_engine
	else
		LDFLAGS := -L$(LIBDIR) -laion_engine -Wl,-rpath,$(LIBDIR)
	endif
endif

.PHONY: all clean

all: showcase_c

showcase_c: main.c
	@if [ ! -f "$(LIBDIR)/$(LIBNAME)" ] && [ ! -f "$(LIBDIR)/aion_engine.dll" ]; then echo "missing engine library under $(LIBDIR)"; exit 1; fi
	$(CC) $(CFLAGS) main.c $(LDFLAGS) -o showcase_c

clean:
	rm -f showcase_c
