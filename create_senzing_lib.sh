#!/bin/bash
# Create Senzing-style library name (libSzConfigTool) from Rust output
# Usage: ./create_senzing_lib.sh [release|debug]

set -e

PROFILE="${1:-release}"
TARGET_DIR="target/${PROFILE}"

echo "Creating Senzing-style library names in ${TARGET_DIR}..."

# Detect OS
case "$(uname -s)" in
    Darwin)
        # macOS
        SRC="${TARGET_DIR}/libsz_configtool_lib.dylib"
        DST="${TARGET_DIR}/libSzConfigTool.dylib"
        if [ -f "$SRC" ]; then
            echo "  $SRC -> $DST"
            ln -sf "$(basename $SRC)" "$DST"
        else
            echo "  ✗ Source not found: $SRC"
            exit 1
        fi
        ;;
    Linux)
        # Linux
        SRC="${TARGET_DIR}/libsz_configtool_lib.so"
        DST="${TARGET_DIR}/libSzConfigTool.so"
        if [ -f "$SRC" ]; then
            echo "  $SRC -> $DST"
            ln -sf "$(basename $SRC)" "$DST"
        else
            echo "  ✗ Source not found: $SRC"
            exit 1
        fi
        ;;
    *)
        echo "  ✗ Unsupported OS: $(uname -s)"
        exit 1
        ;;
esac

echo "✓ Created Senzing-style library: $DST"
