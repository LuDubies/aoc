import numpy as np


def visible(x: int, y: int, tmap: np.array) -> bool:
    xcnt, ycnt = tmap.shape

    if x == 0 or y == 0 or x == xcnt - 1 or y == ycnt - 1:
        return True
    return np.all(np.less(tmap[x, :y], tmap[x, y])) or np.all(np.less(tmap[x, y + 1:], tmap[x, y])) or \
           np.all(np.less(tmap[:x, y], tmap[x, y])) or np.all(np.less(tmap[x+1:, y], tmap[x, y]))


def view_distance(height: int, treeline: np.array) -> int:
    if treeline.size == 0:
        return 0
    overlook = np.less(treeline, height)
    if np.all(overlook):
        return overlook.size
    return np.nonzero(np.invert(overlook))[0][0] + 1


def scenic_score(x: int, y: int, tmap: np.array) -> int:
    tv = view_distance(tmap[x, y], np.flip(tmap[:x, y]))
    bv = view_distance(tmap[x, y], tmap[x+1:, y])
    lv = view_distance(tmap[x, y], np.flip(tmap[x, :y]))
    rv = view_distance(tmap[x, y], tmap[x, y+1:])
    # print(f"Scenic score of {x}, {y} is {tv} * {bv} * {lv} * {rv} = {tv * bv * lv * rv}.")
    return tv * bv * lv * rv

with open('input.txt', 'r') as inp:
    lines = inp.readlines()

    tree_map = [[int(tree) for tree in list(line.strip())] for line in lines]
    tree_map = np.array(tree_map)
    print(f"Visible tree count is {sum(visible(i, j, tree_map) for i in range(tree_map.shape[0]) for j in range(tree_map.shape[1]))}")
    print(f"Gighest scenic score is {max(scenic_score(i, j, tree_map) for i in range(tree_map.shape[0]) for j in range(tree_map.shape[1]))}")
