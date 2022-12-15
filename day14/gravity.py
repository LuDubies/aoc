from typing import Tuple, Union

if __name__ == "__main__":
    with open('input.txt', 'r') as inp:
        lines = [line.strip() for line in inp.readlines()]

        formations = [[tuple(map(int, coords.split(','))) for coords in line.split('->')] for line in lines]

        rocks = set()
        sand = set()

        for formation in formations:
            for i in range(len(formation) - 1):
                origin = formation[i]
                target = formation[i+1]
                # each range twice (one is of len 1)
                for x in range(origin[0], target[0] +1):
                    rocks.add((x, origin[1]))
                for x in range(target[0], origin[0] + 1):
                    rocks.add((x, origin[1]))
                for y in range(origin[1], target[1] + 1):
                    rocks.add((origin[0], y))
                for y in range(target[1], origin[1] + 1):
                    rocks.add((origin[0], y))

        # set lower limit (void) and sand spawn
        void = max(rocks, key=lambda r: r[1])[1] + 20  # marking void as way below the floor so part 1 still works
        spawn = (500, 0)

        def drop_sand(f: Tuple[int, int]) -> Tuple[int, int]:
            if (f[0], f[1] + 1) not in rocks and (f[0], f[1] + 1) not in sand:  # sand keeps falling
                if f[1] + 1 >= void:
                    return -1, -1
                return drop_sand((f[0], f[1] + 1))
            if (f[0] - 1, f[1] + 1) not in rocks and (f[0] - 1, f[1] + 1) not in sand:  # left free
                return drop_sand((f[0] - 1, f[1] + 1))
            if (f[0] + 1, f[1] + 1) not in rocks and (f[0] + 1, f[1] + 1) not in sand:  # right free
                return drop_sand((f[0] + 1, f[1] + 1))
            sand.add(f)
            return f  # nothing free

        sandcount = 0
        while 1:
            sandcount += 1
            if drop_sand(spawn) == (-1, -1):
                break

        print(f"Challenge 1: Sand falling after {sandcount} spawns. {sandcount-1} rested")

        # reset sand, add floor
        sand = set()
        floory = max(rocks, key=lambda r: r[1])[1] + 2
        # dont need infinite floor, sand can at most reach floory to the left or right
        for x in range(spawn[0] - (floory + 10), spawn[0] + (floory + 10)):
            rocks.add((x, floory))

        sandcount = 0
        while 1:
            sandcount += 1
            if drop_sand(spawn) == spawn:
                print(f"{sandcount} sand falling before spawn is blocked.")
                break
