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

### Size Categories

- **0-999**: `(small)` in cyan
- **1,000-999,999**: `(medium)` in green
- **1,000,000-999,999,999**: `(pretty big)` in yellow
- **1,000,000,000+**: `(extremely big)` in bold red

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

## Error Handling

Invalid input will display an error message:

```bash
$ nn "not a number"
Error: Please enter a valid number

$ echo "invalid" | nn
Error: Please enter a valid number
```
