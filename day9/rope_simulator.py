from typing import Tuple

TAIL_VISITED = set()


def make_step(head: Tuple[int, int], tail: Tuple[int, int], direction: str) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    hx, hy = head
    tx, ty = tail
    match direction:
        case 'U':
            hx += 1
        case 'D':
            hx -= 1
        case 'R':
            hy += 1
        case 'L':
            hy -= 1

    # does the tail move?
    xdiff = hx - tx
    ydiff = hy - ty
    need_move = abs(xdiff) > 1 or abs(ydiff) > 1

    if need_move:
        if xdiff < 0:
            tx -= 1
        elif xdiff > 0:
            tx += 1
        if ydiff < 0:
            ty -= 1
        elif ydiff > 0:
            ty += 1

    TAIL_VISITED.add((tx, ty))
    return (hx, hy), (tx, ty)


def make_move(head: Tuple[int, int], tail: Tuple[int, int], move: Tuple[str, int]) -> Tuple[Tuple[int, int], Tuple[int, int]]:
    direction, times = move
    for _ in range(times):
        head, tail = make_step(head, tail, direction)
    print(f"After move {move}, head is at {head}, tail is at {tail}.")
    return head, tail


with open('input.txt', 'r') as inp:
    moves = [(line.strip().split()[0], int(line.strip().split()[1])) for line in inp.readlines()]

    h = (0, 0)
    t = (0, 0)
    for move in moves:
        h, t = make_move(h, t, move)
    print(len(TAIL_VISITED))





