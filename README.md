# nice-number

A Rust command-line tool that formats numbers with thousand separators and adds colorful descriptions based on their size. Accepts numbers as command-line arguments or via stdin, and supports integers, decimals, and scientific notation.

## Installation

```bash
cargo build --release
```

## Usage

You can pass numbers as command-line arguments or via stdin:

**Command-line argument:**
```bash
nn 1234567
```

**Via stdin:**
```bash
echo "1234567" | nn
```

## Output

The tool reads a number from the command line or stdin and outputs:
- The number formatted with commas as thousand separators
- Supports integers, decimals, and scientific notation
- Decimal numbers are rounded to 2 decimal places
- A note "(rounded)" is added if rounding occurred
- A colorful description of the number's size
- Optional binary unit format (KiB, MiB, GiB, etc.) with `--bytes` flag

### Size Categories

- **0-999**: `(small)` in cyan
- **1,000-999,999**: `(medium)` in green
- **1,000,000-999,999,999**: `(pretty big)` in yellow
- **1,000,000,000+**: `(extremely big)` in bold red

### Binary Units (--bytes flag)

When using the `--bytes` or `-b` flag, the tool also displays the number in binary units:
- **B** (bytes): Less than 1,024
- **KiB** (kibibyte): 1,024 bytes
- **MiB** (mebibyte): 1,024² bytes
- **GiB** (gibibyte): 1,024³ bytes
- **TiB** (tebibyte): 1,024⁴ bytes
- **PiB** (pebibyte): 1,024⁵ bytes
- **EiB** (exbibyte): 1,024⁶ bytes

## Examples

**Using command-line arguments:**
```bash
$ nn 42
42 (small)

$ nn 5000
5,000 (medium)

$ nn 42.123456
42.12 (rounded) (small)

$ nn 1234567.89
1,234,567.89 (pretty big)

$ nn 9876543210
9,876,543,210 (extremely big)

$ nn 1.23e5
123,000 (medium)

$ nn -- -5000
-5,000 (medium)
```

**Using stdin:**
```bash
$ echo "42.50" | nn
42.50 (small)

$ echo "2000000" | nn
2,000,000 (pretty big)

$ echo "9.87654321e9" | nn
9,876,543,210 (extremely big)
```

**Using the --bytes flag:**
```bash
$ nn 1024 --bytes
1,024 (medium)
1 KiB

$ nn 1048576 -b
1,048,576 (pretty big)
1 MiB

$ nn 1073741824 --bytes
1,073,741,824 (extremely big)
1 GiB

$ nn 2048.5 --bytes
2,048.50 (medium)
2.00 KiB

$ nn 512 --bytes
512 (small)
512 B

$ echo "5120" | nn --bytes
5,120 (medium)
5 KiB
```

## Error Handling

Invalid input will display an error message:

```bash
$ nn "not a number"
Error: Please enter a valid number

$ echo "invalid" | nn
Error: Please enter a valid number
```
