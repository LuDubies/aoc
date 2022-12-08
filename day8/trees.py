

with open('input.txt', 'r') as inp:
    lines = inp.readlines()

    tree_map = [[int(tree) for tree in list(line.strip())] for line in lines]

    highest = - 1

    # visible for the left
    left_visible = [[highest := tree if i == 0 else max(highest, tree) for i, tree in enumerate(treeline)] for treeline in tree_map]
    left_visible = [[True] + [tree > treeline[i] for i, tree in enumerate(treeline[1::])] for treeline in left_visible]

    # visible from the right
    right_visible = [[highest := tree if i == 0 else max(highest, tree) for i, tree in enumerate(reversed(treeline))] for treeline in tree_map]
    right_visible = [[True] + [tree > treeline[i] for i, tree in enumerate(treeline[1::])] for treeline in right_visible]
    right_visible = [list(reversed(treeline)) for treeline in right_visible]
    