from typing import Tuple, List

if __name__ == "__main__":
    with open('input.txt', 'r') as inp:

        # each scanner is tuple of (x, y, range)
        scanners = set()
        beacons = set()
        lines = [line.strip() for line in inp.readlines()]

        for line in lines:
            sx = int(line[line.index('x') + 2: line.index(',')])
            sy = int(line[line.index('y') + 2: line.index(':')])

            bx = int(int(line[line.index('x', line.index('x') + 1) + 2: line.index(',', line.index(',') + 1)]))
            by = int(line[line.index('y', line.index('y') + 1) + 2:])

            scanners.add((sx, sy, abs(sx - bx) + abs(sy - by)))
            beacons.add((bx, by))

        # return blocking range for scanner in row
        def scanner_block_range(s: Tuple[int, int, int], row: int) -> Tuple[int, int]:
            ydiff = abs(s[1] - row)
            cover = s[2] - ydiff
            return s[0] - cover, s[0] + cover + 1

        def row_ranges(row: int) -> List[Tuple[int, int]]:
            ranges = [scanner_block_range(s, row) for s in scanners]
            return list(filter(lambda r: r[0] < r[1], ranges))

        def blocked_in_row(row: int) -> int:
            ranges = row_ranges(row)
            blockedx = set()
            for r in ranges:
                for x in range(*r):
                    blockedx.add(x)
            # remove pos with beacon on it
            for b in beacons:
                if b[1] == row and b[0] in blockedx:
                    blockedx.remove(b[0])
            return len(blockedx)

        def free_in_row(start: int, end: int, row: int) -> int:
            ranges = row_ranges(row)
            currx = start
            while 1:
                available_ranges = list(filter(lambda r: r[0] <= currx, ranges))
                if not available_ranges:
                    return currx  # no range that blocks from here
                available_ranges.sort(key=lambda r: r[1])  # sort for furthest range
                best_range = available_ranges[-1]
                currx = best_range[1]
                if currx > end:
                    return -1
                ranges = list(filter(lambda r: r[1] > currx, ranges))

        print(f"Challenge 1: {blocked_in_row(2000000)}")

        for y in range(4000001):
            x = free_in_row(0, 4000000, y)
            if x >= 0:
                print(f"found free space at {x=},{y=}, frequency is {x * 4000000 + y}!")
                break
            if y % 100 == 0:
                print(f"Checked up to row {y}!")




