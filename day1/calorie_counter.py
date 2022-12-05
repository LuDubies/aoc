with open('input.txt', 'r') as inp:
    lines = inp.readlines()
    elves = dict()
    elve_counter = 1
    cal_counter = 0

    for line in lines:
        if line.strip() == "":
            elves.update({elve_counter: cal_counter})
            elve_counter += 1
            cal_counter = 0
        else:
            cal_counter += int(line)

    if cal_counter > 0:
        elves.update({elve_counter: cal_counter})

    print(f"The elf with the most calories is carrying {elves.get(max(elves, key=elves.get))} calories.")

    sorted_elves = sorted(elves.items(), key=lambda kv: kv[1])
    print(f"The top 3 elves carry {sum([t[1] for t in sorted_elves[-1:-4:-1]])} calories together.")
