# nice-number

A simple Rust CLI for formatting numbers in a human-readable way.

## Examples

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

nn can also take input on stdin. A single number is treated the same
as above. Larger pieces of text adds commas, but doesn't modify
otherwise.

```bash
$ echo "2000000" | nn
2,000,000 (pretty big)

$ echo "I have 5000 apples and 2500 oranges" | nn
I have 5,000 apples and 2,500 oranges
```

nn also supports a `-b` or `--bytes` flag for calculating KiB, MiB etc
using powers-of-2 units.

```bash
$ nn -b 1073741824
1,073,741,824 (extremely big)
1 GiB
```

## Alternatives

coreutils has a `numfmt` command which does something similar.

```
$ echo 1000000 | numfmt --grouping
1,000,000
```
