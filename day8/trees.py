

with open('input.txt', 'r') as inp:
    lines = inp.readlines()

    tree_map = [[int(tree) for tree in list(line.strip())] for line in lines]

    highest = - 1
    left_visible = [[highest := tree if i == 0 else max(highest, tree) for i, tree in enumerate(treeline)] for treeline in tree_map]
    print(left_visible)
