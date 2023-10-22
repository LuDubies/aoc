from typing import Tuple, List


def convert_to_priority(item: List[str]) -> int:
    item = item[0]
    return ord(item) - 96 if ord(item) >= 97 else ord(item) - (65 - 27)


def compartmentalize(backpack: str) -> Tuple[str, str]:
    return backpack[:len(backpack)//2], backpack[len(backpack)//2:]


def find_same(*packs) -> List[str]:
    return [c for c in packs[0] if all(c in p for p in packs[1:])]


def group_up(backpacks: List[str]) -> List[Tuple[str, str, str]]:
    return list(zip(backpacks[::3], backpacks[1::3], backpacks[2::3]))


with open('input.txt', 'r') as inp:
    rucksacks = inp.readlines()
    rucksacks = [r.strip() for r in rucksacks]
    print(f"Sum of double item priorities: "
          f"{sum(convert_to_priority(find_same(*compartmentalize(r))) for r in rucksacks)}")
    print(f"Badge priority sum: {sum(convert_to_priority(find_same(*group)) for group in group_up(rucksacks))}")

