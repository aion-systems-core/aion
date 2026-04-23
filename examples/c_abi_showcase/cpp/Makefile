ROOT := $(abspath ../../..)
LIBDIR ?= $(ROOT)/target/debug
INCDIR := $(ROOT)/include

CXXFLAGS := -Wall -Wextra -O2 -std=c++17 -I$(INCDIR)
LDFLAGS := -L$(LIBDIR) -laion_engine -Wl,-rpath,$(LIBDIR)

.PHONY: all clean

all: showcase_cpp

showcase_cpp: main.cpp
	$(CXX) $(CXXFLAGS) main.cpp $(LDFLAGS) -o showcase_cpp

clean:
	rm -f showcase_cpp
