# nice-number

A Rust command-line tool that formats numbers with thousand separators.

It adds colorful descriptions based on number size.

You can pass numbers as command-line arguments or via stdin.

It supports integers, decimals, and scientific notation. It can also process arbitrary text, formatting only the embedded numbers while preserving the rest unchanged.

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
- A colorful description of the number's size (for pure numbers)
- Optional binary unit format (KiB, MiB, GiB, etc.) with `--bytes` flag
- **Text processing mode**: Can process arbitrary text with embedded numbers, formatting only the numbers while preserving the rest unchanged

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

**Text processing mode (formats numbers within text):**
```bash
$ echo "The file is 1024 bytes" | nn
The file is 1,024 bytes

$ echo "I have 5000 apples and 2500 oranges" | nn
I have 5,000 apples and 2,500 oranges

$ echo "Price is 1234.56 dollars" | nn
Price is 1,234.56 dollars

$ echo "Temperature: -25.5 degrees" | nn
Temperature: -25.50 degrees

$ echo "Science: 1.23e5 particles" | nn
Science: 123,000 particles

$ echo "Results: 10 samples, 1000 iterations, 0.05 error rate" | nn
Results: 10 samples, 1,000 iterations, 0.05 error rate

$ echo "Hello world without numbers" | nn
Hello world without numbers
```

## Similar Projects

Several tools offer number formatting capabilities. Here's how `nice-number` compares:

### numfmt (GNU Coreutils)

[`numfmt`](https://www.gnu.org/software/coreutils/manual/html_node/numfmt-invocation.html) is a general-purpose number formatting utility from GNU coreutils.

**Comparison:**
- **numfmt strengths**: More formatting options (SI units, IEC units, field selection), locale support, padding/alignment, arbitrary precision
- **numfmt limitations**: No colorful size descriptions, more complex syntax, primarily designed for data processing rather than interactive use
- **nice-number advantages**: Simpler syntax, colorful visual feedback, text processing mode (automatically finds and formats numbers in text), more user-friendly for interactive terminal use

**Example:**
```bash
# numfmt: requires explicit format specification
$ echo 1234567 | numfmt --grouping
1,234,567

# nice-number: automatic formatting with visual feedback
$ echo 1234567 | nn
1,234,567 (pretty big)
```

### humansize / humanize

Python libraries like [`humanize`](https://github.com/python-humanize/humanize) and Rust crates like [`humansize`](https://github.com/LeopoldArkham/humansize) focus on human-readable file sizes.

**Comparison:**
- **humansize/humanize strengths**: Primarily library APIs, extensive size formatting options, language-specific integration
- **humansize/humanize limitations**: Not standalone CLI tools (require scripting), focused mainly on byte sizes rather than general numbers
- **nice-number advantages**: Standalone CLI ready to use, handles all number types (not just bytes), colorful descriptions, text processing mode

### bc / calc / qalc

Command-line calculators like [`bc`](https://www.gnu.org/software/bc/), [`calc`](http://www.isthe.com/chongo/tech/comp/calc/), and [`qalc`](https://qalculate.github.io/).

**Comparison:**
- **bc/calc/qalc strengths**: Powerful calculation engines, scripting capabilities, scientific/mathematical functions
- **bc/calc/qalc limitations**: Focused on computation rather than formatting, no automatic thousand separators, no visual size descriptions
- **nice-number advantages**: Specialized for formatting and readability, automatic thousand separators, colorful size context, simpler for formatting-only tasks

### thousands (Rust crate)

The [`thousands`](https://github.com/Thomasdezeeuw/thousands) Rust crate provides number formatting with separators.

**Comparison:**
- **thousands strengths**: Library for Rust programs, customizable separators
- **thousands limitations**: Library only (not a CLI tool), no visual descriptions or interactive features
- **nice-number advantages**: Complete CLI tool, colorful output, text processing mode, binary unit support, size descriptions

### When to use nice-number

Choose `nice-number` when you want:
- **Quick visual feedback** on number magnitudes with colorful descriptions
- **Simple, interactive formatting** without complex options or syntax
- **Text processing** that automatically finds and formats embedded numbers
- **User-friendly terminal experience** for everyday number formatting tasks
- **Binary unit conversion** for file sizes and byte counts

Choose alternatives when you need:
- Complex field-based data processing (`numfmt`)
- Scientific calculations and math operations (`bc`, `qalc`)
- Library integration in your code (`humansize`, `thousands`)
- Locale-specific formatting or precise control over output format
