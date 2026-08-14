#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
FIXTURES_DIR="$PROJECT_DIR/tests/fixtures/brains"
GOLDEN="$FIXTURES_DIR/valid_golden.anr"
CORRUPT_DIR="$FIXTURES_DIR/corrupt"

mkdir -p "$CORRUPT_DIR"

export PATH="$HOME/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:$PATH"

# Generate golden brain using the Rust binary
cargo run --bin anr -- brain init "$GOLDEN"

if [ ! -f "$GOLDEN" ]; then
    echo "Failed to generate golden brain"
    exit 1
fi

echo "Golden brain generated: $GOLDEN"
ls -la "$GOLDEN"

# Corruption classes per SD-03 §3.7
# Header field offsets:
#   magic:           0  (4 bytes)
#   format_version:  4  (4 bytes)
#   header_size:     8  (4 bytes)
#   flags:          12  (4 bytes)
#   total_size:     16  (8 bytes)
#   generation:     24  (8 bytes)
#   cortex_offset:  32  (8 bytes)
#   cortex_size:    40  (8 bytes)
#   cerebellum_offset: 48 (8 bytes)
#   cerebellum_size:   56 (8 bytes)
#   section_table_count: 136 (4 bytes)

# 1. Invalid magic
cp "$GOLDEN" "$CORRUPT_DIR/invalid_magic.anr"
printf '\xff\xff\xff\xff' | dd of="$CORRUPT_DIR/invalid_magic.anr" bs=1 count=4 conv=notrunc 2>/dev/null

# 2. Wrong version
cp "$GOLDEN" "$CORRUPT_DIR/wrong_version.anr"
printf '\x02\x00\x00\x00' | dd of="$CORRUPT_DIR/wrong_version.anr" bs=1 count=4 seek=4 conv=notrunc 2>/dev/null

# 3. Wrong header size
cp "$GOLDEN" "$CORRUPT_DIR/wrong_header_size.anr"
printf '\x10\x00\x00\x00' | dd of="$CORRUPT_DIR/wrong_header_size.anr" bs=1 count=4 seek=8 conv=notrunc 2>/dev/null

# 4. Unaligned section offset (cortex offset = 4097, not 4096-aligned)
cp "$GOLDEN" "$CORRUPT_DIR/unaligned_offset.anr"
printf '\x01\x10\x00\x00\x00\x00\x00\x00' | dd of="$CORRUPT_DIR/unaligned_offset.anr" bs=1 count=8 seek=32 conv=notrunc 2>/dev/null

# 5. Section overlap (cortex and cerebellum both at offset 4096)
cp "$GOLDEN" "$CORRUPT_DIR/section_overlap.anr"
printf '\x00\x10\x00\x00\x00\x00\x00\x00' | dd of="$CORRUPT_DIR/section_overlap.anr" bs=1 count=8 seek=32 conv=notrunc 2>/dev/null
printf '\x00\x20\x00\x00\x00\x00\x00\x00' | dd of="$CORRUPT_DIR/section_overlap.anr" bs=1 count=8 seek=40 conv=notrunc 2>/dev/null
printf '\x00\x10\x00\x00\x00\x00\x00\x00' | dd of="$CORRUPT_DIR/section_overlap.anr" bs=1 count=8 seek=48 conv=notrunc 2>/dev/null

# 6. Generation zero
cp "$GOLDEN" "$CORRUPT_DIR/generation_zero.anr"
printf '\x00\x00\x00\x00\x00\x00\x00\x00' | dd of="$CORRUPT_DIR/generation_zero.anr" bs=1 count=8 seek=24 conv=notrunc 2>/dev/null

# 7. Checksum mismatch (corrupt the 32-byte checksum at offset 256)
cp "$GOLDEN" "$CORRUPT_DIR/checksum_mismatch.anr"
printf '\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff' | dd of="$CORRUPT_DIR/checksum_mismatch.anr" bs=1 count=32 seek=256 conv=notrunc 2>/dev/null

# 8. Total size too small (total_size = 100, less than header_size 288)
cp "$GOLDEN" "$CORRUPT_DIR/size_too_small.anr"
python3 -c "import struct; f=open('$CORRUPT_DIR/size_too_small.anr','r+b'); f.seek(16); f.write(struct.pack('<Q', 100)); f.close()"

# 9. Invalid section table count (must be 3, set to 5)
cp "$GOLDEN" "$CORRUPT_DIR/invalid_section_count.anr"
printf '\x05\x00\x00\x00' | dd of="$CORRUPT_DIR/invalid_section_count.anr" bs=1 count=4 seek=136 conv=notrunc 2>/dev/null

# 10. Truncated file (cut at 128 bytes)
cp "$GOLDEN" "$CORRUPT_DIR/truncated.anr"
truncate -s 128 "$CORRUPT_DIR/truncated.anr"

echo "Corrupt fixtures generated in $CORRUPT_DIR"
ls -la "$CORRUPT_DIR"
