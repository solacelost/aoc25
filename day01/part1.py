#!/usr/bin/env python3

from pathlib import Path

dial = 50
solve = 0
with open(Path(__file__).parent.joinpath("input")) as f:
    for line in f.readlines():
        direction = line[0]
        count = int(line.strip()[1:])
        if direction == "L":
            count = -count
        dial = (dial + count) % 100
        if dial == 0:
            solve += 1
print(solve)
