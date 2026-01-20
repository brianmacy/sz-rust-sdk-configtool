# Makefile for libSzConfigTool C tests
#
# Usage:
#   make        - Build test executable
#   make run    - Build and run test
#   make clean  - Remove built files

# Detect OS
UNAME_S := $(shell uname -s)

# Library paths (relative to this Makefile)
LIB_DIR = ../../target/release
INCLUDE_DIR = ../../include

# Platform-specific settings
ifeq ($(UNAME_S),Darwin)
    # macOS
    LIB_EXT = dylib
    DYLD_VAR = DYLD_LIBRARY_PATH
else ifeq ($(UNAME_S),Linux)
    # Linux
    LIB_EXT = so
    DYLD_VAR = LD_LIBRARY_PATH
else
    # Windows (not yet supported)
    LIB_EXT = dll
    DYLD_VAR = PATH
endif

# Compiler settings
CC = cc
CFLAGS = -Wall -Wextra -I$(INCLUDE_DIR)
LDFLAGS = -L$(LIB_DIR) -lSzConfigTool

# Targets
TEST_EXEC = test_basic
TEST_SRC = test_basic.c

.PHONY: all run clean

all: $(TEST_EXEC)

$(TEST_EXEC): $(TEST_SRC)
	@echo "Building C test: $(TEST_EXEC)"
	$(CC) $(CFLAGS) -o $(TEST_EXEC) $(TEST_SRC) $(LDFLAGS)
	@echo "✓ Built: $(TEST_EXEC)"

run: $(TEST_EXEC)
	@echo "Running C test..."
	@$(DYLD_VAR)=$(LIB_DIR) ./$(TEST_EXEC)

clean:
	rm -f $(TEST_EXEC)
	@echo "✓ Cleaned"

# Show build info
info:
	@echo "OS: $(UNAME_S)"
	@echo "Library extension: $(LIB_EXT)"
	@echo "Library path: $(LIB_DIR)"
	@echo "Include path: $(INCLUDE_DIR)"
	@echo "Dynamic loader: $(DYLD_VAR)"
