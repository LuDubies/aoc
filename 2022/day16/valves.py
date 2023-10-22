from dataclasses import dataclass
from typing import List, Set, Tuple
from math import inf
from argparse import ArgumentParser
import time
from itertools import combinations


@dataclass
class Valve:
    flowr: int
    direct: list
    distance: dict


def bad_bfs(valves: dict, starter: str):
    to_check = [starter]

    distances = dict()
    for k in valves.keys():
        if k == starter:
            distances.update({k: 0})
        else:
            distances.update({k: inf})

    while to_check:
        current = to_check.pop(0)
        for nbr in valves[current].direct:
            if distances[nbr] == inf:
                to_check.append(nbr)
            if distances[nbr] > distances[current] + 1:
                distances[nbr] = distances[current] + 1

    valves[starter].distance = distances


def parse_input(lines):
    valves = dict()
    # some dirty parsing to get all valves
    for line in lines:
        name = line[6:8]
        flowrate = int(line[line.index('=') + 1: line.index(';')])
        if 'valves' in line:
            direct = [neighbour.strip() for neighbour in line[line.index('valves') + 7:].split(', ')]
        else:
            direct = [line[line.index('valve') + 6: line.index('valve') + 8]]
        valves.update({name: Valve(flowrate, direct, dict())})
    return valves


if __name__ == "__main__":
    parser = ArgumentParser()
    parser.add_argument('-i', '--input', type=str, default='test.txt')
    parser.add_argument('-c', '--challenge', type=int, default=1)
    args = parser.parse_args()

    with open(args.input, 'r') as inp:
        lines = [line.strip() for line in inp.readlines()]

        '''
        IDEA:
            - get distances for each valve to all other valves
            - simplify graph by removing all 0 flow-rate nodes
            - brute force it
        '''
        tp0 = time.time_ns()
        valves = parse_input(lines)
        tp = (time.time_ns() - tp0) / 10**9
        print(f"Time parsing: {tp: .5f}s")

        # get all distances
        td0 = time.time_ns()
        for vname in valves.keys():
            bad_bfs(valves, vname)
        td = (time.time_ns() - td0) / 10**9
        print(f"Time for distances: {td: .5f}s")

        # remove all 0 flows except AA
        print(f"Total valves = {len(valves)}")
        discard = list()
        for vname in valves.keys():
            if vname != 'AA' and valves[vname].flowr == 0:
                discard.append(vname)
        print(f"Discarding {len(discard)} valves with 0 flow")
        for d in discard:
            valves.pop(d)
        for vname in valves.keys():
            for d in discard:
                valves[vname].distance.pop(d)
        relevant_valve_count = len(valves)
        print(f"Remaining valves {relevant_valve_count}")

        if args.challenge == 1:
            # brute force challenge 1
            def max_release(position: str, remaining_minutes: int, already_open: List[str]) -> int:
                if remaining_minutes <= 0:
                    return 0
                release_from_this = remaining_minutes * valves[position].flowr
                options = list(filter(lambda v: v not in already_open and v != position and v != 'AA', valves.keys()))
                if not options:
                    return release_from_this

                possibilities = [max_release(next, remaining_minutes - valves[position].distance[next] - 1, already_open.copy() + [position]) for next in options]
                return max(possibilities) + release_from_this

            ts0 = time.time_ns()
            solution = max_release('AA', 30, [])
            ts = (time.time_ns() - ts0) / 10**9
            print(f"Time to solve: {ts: .5f}s")
        else:
            # brute force with the elephant helper
            def max_release_el(position_me: str, position_el: str, worktime_me: int, worktime_el: int, remaining_minutes: int, already_targeted: List[str], h) -> Tuple[int, List]:
                if remaining_minutes <= 0:
                    return 0, []

                # one of the worktimes has to be 0 always (either I or elephant opened valve)
                if worktime_me != 0 and worktime_el != 0:
                    raise Exception("You implemented some dumb stuff. worktimes both nonzero")

                # am I or the elephant opening a valve this minute
                release_me = remaining_minutes * valves[position_me].flowr if worktime_me == 0 else 0
                release_el = remaining_minutes * valves[position_el].flowr if worktime_el == 0 else 0
                release_from_this = release_el + release_me

                options = list(filter(lambda v: v not in already_targeted and v != position_me and v != position_el and v != 'AA', valves.keys()))

                if release_me > 0:
                    h.append(f"I open valve {position_me} with {remaining_minutes} minutes remaining for {release_me} pressure. Options: {options}")
                if release_el > 0:
                    h.append(f"The elephant opens valve {position_el} with {remaining_minutes} minutes remaining for {release_el} pressure. Options: {options}")

                # without any decisions remaining calculate release for this configuration
                if not options:
                    if worktime_me > 0 and remaining_minutes - worktime_me > 0:
                        future_release_me = (remaining_minutes - worktime_me) * valves[position_me].flowr
                        h.append(f"Lastly I open valve {position_me} with {remaining_minutes - worktime_me} minutes remaining for {future_release_me} pressure.")
                    else:
                        future_release_me = 0
                    if worktime_el > 0 and remaining_minutes - worktime_el > 0:
                        future_release_el = (remaining_minutes - worktime_el) * valves[position_el].flowr
                        h.append(f"Lastly the elephant opens valve {position_el} with {remaining_minutes - worktime_el} minutes remaining for {future_release_el} pressure.")
                    else:
                        future_release_el = 0
                    return release_from_this + future_release_me + future_release_el, h

                # check who needs a new target valve (position)
                if worktime_me == 0 and worktime_el == 0:
                    # new target for both
                    if len(options) == 1:
                        # can only open one more valve
                        last_target = options.pop(0)
                        min_dist = min(valves[position_me].distance[last_target], valves[position_el].distance[last_target])
                        from_last = max((remaining_minutes - (min_dist + 1)) * valves[last_target].flowr, 0)
                        h.append(f"One of us opens valve {last_target} with {remaining_minutes - min_dist - 1} minutes remaining.")
                        return release_from_this + from_last, h

                    target_combinations = list(combinations(options, 2))
                    best_case, h_best = -1, []
                    for t_el, t_me in target_combinations:
                        new_worktime_me = valves[position_me].distance[t_me] + 1
                        new_worktime_el = valves[position_el].distance[t_el] + 1
                        next_choice_in = min(new_worktime_me, new_worktime_el)

                        comb_solution, h_new = max_release_el(t_me, t_el,
                                                       new_worktime_me - next_choice_in,
                                                       new_worktime_el - next_choice_in,
                                                       remaining_minutes - next_choice_in,
                                                       already_targeted.copy() + [t_me, t_el],
                                                       h.copy())

                        if comb_solution >= best_case:
                            best_case, h_best = comb_solution, h_new
                    return best_case + release_from_this, h_best

                elif worktime_me == 0 and worktime_el != 0:
                    # new target valve for me
                    best_case, h_best = -1, []
                    for t_me in options:
                        new_worktime_me = valves[position_me].distance[t_me] + 1
                        next_choice_in = min(new_worktime_me, worktime_el)
                        target_solution, h_new = max_release_el(t_me, position_el,
                                                         new_worktime_me - next_choice_in,
                                                         worktime_el - next_choice_in,
                                                         remaining_minutes - next_choice_in,
                                                         already_targeted.copy() + [t_me],
                                                         h.copy())
                        if target_solution >= best_case:
                            best_case, h_best = target_solution, h_new
                    return best_case + release_from_this, h_best
                elif worktime_me != 0 and worktime_el == 0:
                    # new target valve for el
                    best_case, h_best = -1, []
                    for t_el in options:
                        new_worktime_el = valves[position_el].distance[t_el] + 1
                        next_choice_in = min(worktime_me, new_worktime_el)
                        target_solution, h_new = max_release_el(position_me, t_el,
                                                         worktime_me - next_choice_in,
                                                         new_worktime_el - next_choice_in,
                                                         remaining_minutes - next_choice_in,
                                                         already_targeted.copy() + [t_el],
                                                         h.copy())
                        if target_solution >= best_case:
                            best_case, h_best = target_solution, h_new
                    return best_case + release_from_this, h_best
                else:
                    raise Exception("Worktime Error")


            ts0 = time.time_ns()
            solution, hist = max_release_el('AA', 'AA', 0, 0, 26, [], [])
            ts = (time.time_ns() - ts0) / 10 ** 9
            print(f"Time to solve: {ts: .5f}s")
            for moment in hist:
                print(moment)

        print(f"Challenge {args.challenge}: {solution}")

