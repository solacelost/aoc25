#!/usr/bin/env python3

from pathlib import Path

input = Path(__file__).parent.joinpath("input")

with open(input) as f:
    data = [line.strip() for line in f.readlines()]

max = len(data)


def process(x: int, y: int) -> int:
    global cache
    global data
    global max
    if y == max:
        return 1
    if (result := cache.get((x,y,))) != None:
        return result

    result = 0
    match data[y][x]:
        case ".":
            result = process(x, y + 1)
        case "^":
            result = process(x - 1, y) + process(x + 1, y)
    cache[(x,y,)] = result
    return result


cache = dict()
result = process(data[0].find("S"), 1)
print(result)
