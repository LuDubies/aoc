def all_unique(snippet: str) -> bool:
    return all(c not in snippet[:i:] + snippet[i+1::] for i, c in enumerate(snippet))


with open('input.txt', 'r') as inp:
    line = inp.readlines()[0]

    print(f"Start of packet at {[ind + 4 for ind, _ in enumerate(line) if all_unique(line[ind:ind+4:])][0]}.")
    print(f"Start of message after {[ind + 14 for ind, _ in enumerate(line) if all_unique(line[ind:ind+14:])][0]}.")





