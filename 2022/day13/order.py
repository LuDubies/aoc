from ast import literal_eval
from typing import List, Union
from functools import cmp_to_key

DEBUG = False


def in_order(a: Union[List, int], b: Union[List, int]) -> int:
    if isinstance(a, int) and isinstance(b, int):
        if a < b:
            return -1
        if b < a:
            if DEBUG:
                print(f"{a} > {b} invalid!")
            return 1
        if DEBUG:
            print(f"  --     {a} vs {b} undecided")
        return 0
    if isinstance(a, list) and isinstance(b, list):
        for i in range(min(len(a), len(b))):
            if DEBUG:
                print(f"  --  checking {a[i]} vs {b[i]}")
            elem_sorting = in_order(a[i], b[i])
            if elem_sorting != 0:
                return elem_sorting
        if len(a) == len(b):
            return 0
        else:
            if len(b) < len(a):
                print(f"{a} longer {b} invalid!")
                return 1
            else:
                return -1
    if isinstance(a, int):
        return in_order([a], b)
    else:
        return in_order(a, [b])


if __name__ == "__main__":
    with open('input.txt', 'r') as inp:
        lines = [l.strip() for l in inp.readlines() if l.strip() != ""]
        lists = [literal_eval(line) for line in lines]
        pairs = zip(lists[::2], lists[1::2])

        index_sum = 0
        for i, p in enumerate(pairs):
            if DEBUG:
                print(f"Checking pair{i+1}: {p[0]} : {p[1]}")
            if in_order(*p) == -1:
                if DEBUG:
                    print("Valid")
                index_sum += i+1
        print(f"Challenge 1: {index_sum}")

        lists.append([[2]])
        lists.append([[6]])

        lists.sort(key=cmp_to_key(in_order))
        print(f"Challenge 2: {(lists.index([[2]]) + 1) * (lists.index([[6]]) + 1)}")
