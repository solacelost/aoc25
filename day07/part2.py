#!/usr/bin/env python3

from collections import namedtuple
from pathlib import Path

Point = namedtuple("Point", ["x", "y"])

with open(Path(__file__).parent.joinpath("input")) as f:
    data: list[str] = [line.strip() for line in f.readlines()]


def process(beam: Point) -> int:
    if beam.y == len(data):
        return 1
    if (result := cache.get(beam)) != None:
        return result
    match data[beam.y][beam.x]:
        case ".":
            next = Point(beam.x, beam.y + 1)
            result = process(next)
        case "^":
            left, right = Point(beam.x - 1, beam.y), Point(beam.x + 1, beam.y)
            result = process(left) + process(right)
        case _:
            raise RuntimeError("Can't get here")
    cache[beam] = result
    return result


cache: dict[Point, int] = dict()
start = Point(data[0].find("S"), 1)
result = process(start)
print(result)
