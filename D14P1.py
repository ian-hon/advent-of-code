from __future__ import annotations

import sys
from typing import NamedTuple
from more_itertools import pairwise

raw_data_string = """
495,144 -> 499,144
492,40 -> 492,43 -> 484,43 -> 484,50 -> 499,50 -> 499,43 -> 496,43 -> 496,40
499,13 -> 499,17 -> 497,17 -> 497,25 -> 505,25 -> 505,17 -> 502,17 -> 502,13
504,141 -> 508,141
498,135 -> 502,135
499,13 -> 499,17 -> 497,17 -> 497,25 -> 505,25 -> 505,17 -> 502,17 -> 502,13
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
485,147 -> 490,147
483,153 -> 488,153
479,151 -> 484,151
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
510,141 -> 514,141
498,53 -> 498,57 -> 495,57 -> 495,64 -> 508,64 -> 508,57 -> 502,57 -> 502,53
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
476,153 -> 481,153
506,83 -> 506,85 -> 503,85 -> 503,88 -> 513,88 -> 513,85 -> 511,85 -> 511,83
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
506,83 -> 506,85 -> 503,85 -> 503,88 -> 513,88 -> 513,85 -> 511,85 -> 511,83
515,165 -> 520,165
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
508,165 -> 513,165
494,155 -> 499,155
506,83 -> 506,85 -> 503,85 -> 503,88 -> 513,88 -> 513,85 -> 511,85 -> 511,83
487,30 -> 487,31 -> 499,31 -> 499,30
497,129 -> 502,129
473,155 -> 478,155
517,123 -> 522,123
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
506,83 -> 506,85 -> 503,85 -> 503,88 -> 513,88 -> 513,85 -> 511,85 -> 511,83
489,149 -> 494,149
507,126 -> 512,126
489,144 -> 493,144
506,83 -> 506,85 -> 503,85 -> 503,88 -> 513,88 -> 513,85 -> 511,85 -> 511,83
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
486,151 -> 491,151
498,53 -> 498,57 -> 495,57 -> 495,64 -> 508,64 -> 508,57 -> 502,57 -> 502,53
521,126 -> 526,126
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
487,155 -> 492,155
513,120 -> 518,120
499,13 -> 499,17 -> 497,17 -> 497,25 -> 505,25 -> 505,17 -> 502,17 -> 502,13
482,149 -> 487,149
500,158 -> 500,159 -> 513,159
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
499,13 -> 499,17 -> 497,17 -> 497,25 -> 505,25 -> 505,17 -> 502,17 -> 502,13
492,40 -> 492,43 -> 484,43 -> 484,50 -> 499,50 -> 499,43 -> 496,43 -> 496,40
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
499,13 -> 499,17 -> 497,17 -> 497,25 -> 505,25 -> 505,17 -> 502,17 -> 502,13
505,168 -> 510,168
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
492,40 -> 492,43 -> 484,43 -> 484,50 -> 499,50 -> 499,43 -> 496,43 -> 496,40
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
509,117 -> 514,117
498,53 -> 498,57 -> 495,57 -> 495,64 -> 508,64 -> 508,57 -> 502,57 -> 502,53
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
498,53 -> 498,57 -> 495,57 -> 495,64 -> 508,64 -> 508,57 -> 502,57 -> 502,53
503,123 -> 508,123
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
498,53 -> 498,57 -> 495,57 -> 495,64 -> 508,64 -> 508,57 -> 502,57 -> 502,53
510,123 -> 515,123
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
513,144 -> 517,144
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
501,138 -> 505,138
507,144 -> 511,144
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
498,53 -> 498,57 -> 495,57 -> 495,64 -> 508,64 -> 508,57 -> 502,57 -> 502,53
492,40 -> 492,43 -> 484,43 -> 484,50 -> 499,50 -> 499,43 -> 496,43 -> 496,40
492,141 -> 496,141
495,138 -> 499,138
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
492,40 -> 492,43 -> 484,43 -> 484,50 -> 499,50 -> 499,43 -> 496,43 -> 496,40
498,141 -> 502,141
501,155 -> 506,155
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
497,153 -> 502,153
504,129 -> 509,129
518,129 -> 523,129
480,155 -> 485,155
513,114 -> 513,110 -> 513,114 -> 515,114 -> 515,110 -> 515,114 -> 517,114 -> 517,107 -> 517,114
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
506,83 -> 506,85 -> 503,85 -> 503,88 -> 513,88 -> 513,85 -> 511,85 -> 511,83
498,53 -> 498,57 -> 495,57 -> 495,64 -> 508,64 -> 508,57 -> 502,57 -> 502,53
492,40 -> 492,43 -> 484,43 -> 484,50 -> 499,50 -> 499,43 -> 496,43 -> 496,40
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
525,129 -> 530,129
504,135 -> 508,135
487,30 -> 487,31 -> 499,31 -> 499,30
487,30 -> 487,31 -> 499,31 -> 499,30
493,151 -> 498,151
514,126 -> 519,126
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
507,138 -> 511,138
495,37 -> 506,37
511,101 -> 511,96 -> 511,101 -> 513,101 -> 513,93 -> 513,101 -> 515,101 -> 515,94 -> 515,101
500,126 -> 505,126
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
519,168 -> 524,168
511,162 -> 516,162
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
499,13 -> 499,17 -> 497,17 -> 497,25 -> 505,25 -> 505,17 -> 502,17 -> 502,13
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
492,40 -> 492,43 -> 484,43 -> 484,50 -> 499,50 -> 499,43 -> 496,43 -> 496,40
506,120 -> 511,120
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
501,132 -> 505,132
511,129 -> 516,129
494,80 -> 507,80
506,83 -> 506,85 -> 503,85 -> 503,88 -> 513,88 -> 513,85 -> 511,85 -> 511,83
501,144 -> 505,144
490,153 -> 495,153
512,168 -> 517,168
490,77 -> 490,68 -> 490,77 -> 492,77 -> 492,75 -> 492,77 -> 494,77 -> 494,74 -> 494,77 -> 496,77 -> 496,72 -> 496,77
507,181 -> 507,171 -> 507,181 -> 509,181 -> 509,173 -> 509,181 -> 511,181 -> 511,176 -> 511,181 -> 513,181 -> 513,176 -> 513,181
499,13 -> 499,17 -> 497,17 -> 497,25 -> 505,25 -> 505,17 -> 502,17 -> 502,13
500,158 -> 500,159 -> 513,159
"""


class Coord(NamedTuple):
    x: int
    y: int

    def __add__(self: Coord, other: Coord) -> Coord:
        return Coord(self.x + other.x, self.y + other.y)

    def __sub__(self: Coord, other: Coord) -> Coord:
        return Coord(self.x - other.x, self.y - other.y)

    def __floordiv__(self: Coord, scalar: int) -> Coord:
        return Coord(self.x // scalar, self.y // scalar)

    @property
    def manhattan_norm(self) -> int:
        return abs(self.x) + abs(self.y)


def drop_sand(rock, source, floor, part):
    sand = set()
    while True:
        visualise(rock, sand)
        grain = fall(source, rock, sand, floor)
        if part == 1 and grain.y + 1 == floor:
            break
        sand.add(grain)
        if part == 2 and grain == source:
            break
    return sand


def fall(grain: Coord, rock: set[Coord], sand: set[Coord], floor: int) -> Coord:
    while True:
        deltas = (
            Coord(0, +1),
            Coord(-1, +1),
            Coord(+1, +1),
        )
        for delta in deltas:
            new_position = grain + delta
            if (
                new_position not in sand
                and new_position not in rock
                and new_position.y < floor
            ):
                grain = new_position
                break
        else:
            return grain


def main() -> int:
    lines = raw_data_string.strip().splitlines()

    rock = set()
    for line in lines:
        nodes = line.split("->")
        nodes = [Coord(*map(int, node.split(","))) for node in nodes]
        for a, b in pairwise(nodes):
            direction = (b - a) // (b - a).manhattan_norm
            position = a
            while position != b:
                rock.add(position)
                position += direction
            rock.add(b)

    source = Coord(500, 0)
    floor = 2 + max(coord.y for coord in rock)

    # Part 1
    sand = drop_sand(rock, source, floor, 1)
    print(len(sand))

    # Part 2
    sand = drop_sand(rock, source, floor, 2)
    print(len(sand))

    return 0


def visualise(rock: set[Coord], sand: set[Coord]) -> None:
    return
    both = rock | sand
    minx = min(c.x for c in both)
    maxx = max(c.x for c in both)
    miny = min(c.y for c in both)
    maxy = max(c.y for c in both)
    for y in range(miny, maxy + 1):
        for x in range(minx, maxx + 1):
            coord = Coord(x, y)
            char = "#" if coord in rock else "o" if coord in sand else "."
            print(char, end="")
        print()
    print()


if __name__ == "__main__":
    sys.exit(main())