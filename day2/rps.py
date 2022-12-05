from typing import List, Mapping, Tuple

CHALLENGES = ["A", "B", "C"]
RESPONSES = ["X", "Y", "Z"]

SECOND_CHALLENGE = True


# returns result as one of (0, 3, 6) for lost, won, draw
def result(choices: Tuple[int, int]) -> int:
    r = choices[0] - choices[1]
    if r == 0:
        return 3
    if r in (-1, 2):
        return 6
    if r in (-2, 1):
        return 0
    return 0


def score(game: List[str]) -> int:
    their_choice = CHALLENGES.index(game[0])
    my_choice = RESPONSES.index(game[1])
    if SECOND_CHALLENGE:  # change my_response to fit challenge
        if my_choice == 0:  # need to loose
            my_choice = (their_choice - 1) % 3
        elif my_choice == 1:  # need to draw
            my_choice = their_choice
        else:  # need to win
            my_choice = (their_choice + 1) % 3
    return result((their_choice, my_choice)) + my_choice + 1


with open('input.txt', 'r') as inp:
    lines = inp.readlines()
    print(sum(map(score, list(map(str.split, lines)))))
