#!/usr/bin/env python3
"""Compare criterion benchmark results (bencher output format) of a base and a head build.

Usage: compare_benches.py <base_dir> <head_dir>

Each directory holds one or more `run_*.txt` files with `--output-format bencher` output.
For every benchmark, the fastest (minimum) result over all runs is used, which is the
estimate least affected by noise from other processes on the runner.

Prints a markdown table to stdout.
"""

import re
import sys
from pathlib import Path

LINE_RE = re.compile(r"^test (\S+) \.\.\. bench:\s+([\d,]+) ns/iter \(\+/- ([\d,]+)\)")

# Changes within this band are treated as noise
NOISE_THRESHOLD = 0.03


def parse(directory: Path) -> dict[str, int]:
    results: dict[str, int] = {}
    for f in sorted(directory.glob("run_*.txt")):
        for line in f.read_text().splitlines():
            m = LINE_RE.match(line.strip())
            if m:
                name, ns = m.group(1), int(m.group(2).replace(",", ""))
                results[name] = min(ns, results.get(name, ns))
    return results


def fmt_ns(ns: int) -> str:
    for unit, div in (("s", 1e9), ("ms", 1e6), ("µs", 1e3)):
        if ns >= div:
            return f"{ns / div:.2f} {unit}"
    return f"{ns} ns"


def main() -> None:
    base = parse(Path(sys.argv[1]))
    head = parse(Path(sys.argv[2]))

    print("| Benchmark | Base | PR | Change |")
    print("|---|---:|---:|---:|")
    for name in sorted(base.keys() | head.keys()):
        b, h = base.get(name), head.get(name)
        if b is None or h is None:
            print(f"| `{name}` | {fmt_ns(b) if b else '–'} | {fmt_ns(h) if h else '–'} | n/a |")
            continue
        change = (h - b) / b
        if change > NOISE_THRESHOLD:
            icon = "🔴"
        elif change < -NOISE_THRESHOLD:
            icon = "🟢"
        else:
            icon = "⚪"
        print(f"| `{name}` | {fmt_ns(b)} | {fmt_ns(h)} | {icon} {change:+.1%} |")
    print()
    print(
        f"Lower is better. Both builds ran on the same runner, interleaved; "
        f"the fastest of all runs per benchmark is shown. "
        f"Changes within ±{NOISE_THRESHOLD:.0%} are marked ⚪ (likely noise)."
    )


if __name__ == "__main__":
    main()
