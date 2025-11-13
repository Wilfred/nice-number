# nice-number

A Rust command-line tool that formats integers with thousand separators and adds colorful, whimsical descriptions based on their size.

## Installation

```bash
cargo build --release
```

## Usage

```bash
echo "1234567" | cargo run --quiet
```

Or use the compiled binary:

```bash
echo "1234567" | ./target/release/nice-number
```

## Output

The tool reads an integer from stdin and outputs:
- The number formatted with commas as thousand separators
- A colorful description of the number's size

### Size Categories

- **0-999**: `(tiny)` in cyan
- **1,000-999,999**: `(medium)` in green
- **1,000,000-999,999,999**: `(pretty big)` in yellow
- **1,000,000,000+**: `(extremely big)` in bold red

## Examples

```bash
$ echo "42" | cargo run --quiet
42 (tiny)

$ echo "5000" | cargo run --quiet
5,000 (medium)

$ echo "2000000" | cargo run --quiet
2,000,000 (pretty big)

$ echo "9876543210" | cargo run --quiet
9,876,543,210 (extremely big)
```

## Error Handling

Invalid input will display an error message:

```bash
$ echo "not a number" | cargo run --quiet
Error: Please enter a valid integer
```
