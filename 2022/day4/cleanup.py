from typing import Tuple


def contained(first_range: range, second_range: range) -> bool:
    return all(s in first_range for s in second_range) or all(f in second_range for f in first_range)


def overlap(first_range: range, second_range: range) -> bool:
    return any(s in first_range for s in second_range)


def get_ranges(assignment: str) -> Tuple[range, range]:
    as_list = [r.split('-') for r in assignment.split(',')]
    return range(int(as_list[0][0]), int(as_list[0][1]) + 1), range(int(as_list[1][0]), int(as_list[1][1]) + 1)


with open('input.txt', 'r') as inp:
    assignments = [line.strip() for line in inp.readlines()]

    print(sum(contained(*get_ranges(a)) for a in assignments))
    print(sum(overlap(*get_ranges(a)) for a in assignments))

