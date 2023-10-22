from typing import Tuple, List, Union
from dataclasses import dataclass


@dataclass
class Knot:
    x: int
    y: int
    parent: Union['Knot', None]
    visited: set

    def move(self, direction: str):
        match direction:
            case'U':
                self.x += 1
            case 'D':
                self.x -= 1
            case 'R':
                self.y += 1
            case 'L':
                self.y -= 1
        self.visited.add((self.x, self.y))

    def follow(self):
        xdiff = self.parent.x - self.x
        ydiff = self.parent.y - self.y
        if abs(xdiff) > 1 or abs(ydiff) > 1:
            if xdiff < 0:
                self.x -= 1
            if xdiff > 0:
                self.x += 1
            if ydiff < 0:
                self.y -= 1
            if ydiff > 0:
                self.y += 1
        self.visited.add((self.x, self.y))

    @property
    def pos(self):
        return self.x, self.y


def make_move(rope: List[Knot], move: Tuple[str, int]) -> List[Knot]:
    direction, times = move
    for _ in range(times):
        rope[0].move(direction)
        for k in rope[1:]:
            k.follow()
    print(f"After move {move}, head is at {rope[0].pos}, tail is at {rope[-1].pos}.")
    return rope


with open('input.txt', 'r') as inp:
    moves = [(line.strip().split()[0], int(line.strip().split()[1])) for line in inp.readlines()]
    ROPE_LENGTH = 10
    rope = [Knot(0, 0, None, set())]
    for i in range(1, ROPE_LENGTH):
        rope.append(Knot(0, 0, rope[i-1], set()))
    for move in moves:
        rope = make_move(rope, move)
    print(len(rope[-1].visited))





