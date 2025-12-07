#!/usr/bin/env python3

from collections import namedtuple
from pathlib import Path

Point = namedtuple("Point", ["x", "y"])

with open(Path(__file__).parent.joinpath("input")) as f:
    data: list[str] = [line.strip() for line in f.readlines()]


def process(beam: Point) -> int:
    if beam.y == len(data):  # we are at the end, no splits left
        return 0
    if (result := cache.get(beam)) != None:  # skip double-counting splits
        return 0
    match data[beam.y][beam.x]:
        case ".":
            next = Point(beam.x, beam.y + 1)
            result = process(next)  # run the beam further down
        case "^":
            left, right = Point(beam.x - 1, beam.y), Point(beam.x + 1, beam.y)
            result = 1 + process(left) + process(right)  # split the beam, count the split
        case _:
            raise RuntimeError("Can't get here")
    cache[beam] = result  # memoize our non-terminating results
    return result


cache: dict[Point, int] = dict()
start = Point(data[0].find("S"), 1)  # start just below the S at the top
print(process(start))  # recurse into every path the beam could take, print the number of splits
