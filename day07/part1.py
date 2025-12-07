#!/usr/bin/env python3

from pathlib import Path

input = Path(__file__).parent.joinpath("input")

with open(input) as f:
    data = [line.strip() for line in f.readlines()]

max = len(data)


def process(x: int, y: int, cache: dict) -> int:
    if y == max:
        return 0
    if (
        result := cache.get(
            (
                x,
                y,
            )
        )
        != None
    ):
        return 0

    result = 0
    match data[y][x]:
        case ".":
            result = process(x, y + 1, cache)
        case "^":
            result = 1 + process(x - 1, y + 1, cache) + process(x + 1, y + 1, cache)
    cache[
        (
            x,
            y,
        )
    ] = result
    return result


print(process(data[0].find("S"), 1, dict()))
