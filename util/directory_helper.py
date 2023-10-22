"""Utility for directory creation"""

from argparse import ArgumentParser, Namespace
from pathlib import Path
import sys

COMMANDS = [
    'gen_year',
]


def argument_init() -> Namespace:
    """Parse arguments"""
    parser = ArgumentParser()

    parser.add_argument('command', type=str,
                        help="main command for the script\n" \
                        "available: gen_year")

    # flags for gen_year
    parser.add_argument('-y', '--year', type=int,
                        help="year input for directory generation")
    parser.add_argument('-p', '--parent_dir', type=str,
                        help='parent directory')


    return parser.parse_args()


if __name__ == "__main__":
    args = argument_init()
    cmd = args.command

    # exit when command is unknown
    if cmd not in COMMANDS:
        print(f"Invalid command: {cmd}!")
        sys.exit(1)
    elif cmd == 'gen_year':
        year = args.year
        pdir = Path(args.parent_dir)

        if year not in range(1,4000):
            print(f"Invalid year input: {year}!")
            sys.exit(1)
        if not pdir.is_dir():
            print(f"Invalid directory: {pdir}!")
            sys.exit(1)

        newdir = pdir.joinpath(f"{year}")
        try:
            newdir.mkdir()
        except FileExistsError as fee:
            print(f"Directory {newdir} already exists!")
            sys.exit(1)

        for day in range(1,26):
            daydir = newdir.joinpath(f"day{day}")
            daydir.mkdir()
            inpfile = daydir.joinpath("input.txt")
            inpfile.touch()
