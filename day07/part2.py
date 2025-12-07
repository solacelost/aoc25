#!/usr/bin/env python3

from collections import namedtuple
from pathlib import Path

Point = namedtuple("Point", ["x", "y"])

with open(Path(__file__).parent.joinpath("input")) as f:
    data = [line.strip() for line in f.readlines()]

max = len(data)


def process(beam: Point) -> int:
    if beam.y == max:
        return 1
    if (result := cache.get(beam)) != None:
        return result
    match data[beam.y][beam.x]:
        case ".":
            result = process(Point(beam.x, beam.y + 1))
        case "^":
            result = process(Point(beam.x - 1, beam.y)) + process(Point(beam.x + 1, beam.y))
        case _:
            raise RuntimeError("Can't get here")
    cache[beam] = result
    return result


cache = dict()
start = Point(data[0].find("S"), 1)
result = process(start)
print(result)
