from typing import List, Tuple


NEW_CRANE = True


def build_stacks(full_input: List[str]) -> List[List[str]]:
    # find empty line to signal bottom of stacks
    floor = full_input.index('\n') - 1
    stack_input = full_input[floor - 1:: -1]
    stack_input = [s[1::4] for s in stack_input]
    stacks = [[] for _ in stack_input[0]]
    for level in stack_input:
        for i, c in enumerate(level):
            if c != ' ':
                stacks[i].append(c)

    return stacks


def get_actions(full_input: list[str]) -> List[Tuple[int, ...]]:
    actions = [i.strip() for i in full_input[full_input.index('\n') + 1::]]
    actions = [i.strip('move ').replace('from ', '').replace('to ', '').split() for i in actions]
    return [tuple(map(int, i)) for i in actions]


def execute_instruction(stacks: List[List[str]], action: Tuple[int, ...]):
    times, origin, target = action
    origin -= 1
    target -= 1
    if not NEW_CRANE:
        for _ in range(times):
            stacks[target].append(stacks[origin].pop())
    else:
        stacks[target].extend(stacks[origin][len(stacks[origin]) - times::])
        stacks[origin] = stacks[origin][:len(stacks[origin]) - times:]
    return stacks


with open('input.txt', 'r') as inp:
    lines = inp.readlines()
    stacks = build_stacks(lines)
    instructions = get_actions(lines)
    for i, instruction in enumerate(instructions):
        stacks = execute_instruction(stacks, instruction)
    print(''.join((s[-1] for s in stacks)))

