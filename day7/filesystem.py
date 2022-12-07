from dataclasses import dataclass
from abc import ABC, abstractmethod
from typing import List, Union


class Node(ABC):
    name: str

    @abstractmethod
    def get_size(self) -> int:
        """implementation depends on subtype"""

    @abstractmethod
    def add_child(self, n: 'Node'):
        """implementation depends on subtype"""

    @abstractmethod
    def parse_shell(self, shell: List[str]):
        """implementation depends on subtype"""

    @abstractmethod
    def get_directories(self) -> List['Node']:
        """implementation depends on subtype"""


@dataclass
class Directory(Node):
    name: str
    parent: Union['Directory', None]
    children: List[Node]

    def get_size(self) -> int:
        return sum(c.get_size() for c in self.children)

    def add_child(self, n: Node):
        self.children.append(n)

    def parse_shell(self, shell: List[str]):
        if shell[0] != "$ ls":
            raise Exception(f"Unexpected first line {shell[0]} for dir parsing!")
        # split shell into ls output and remaining
        got_command = False
        listing = [sl for sl in shell[1::] if not (got_command := got_command or "$" in sl)]
        got_command = False
        remaining = [sl for sl in shell[1::] if (got_command := got_command or "$" in sl)]

        # add children
        for list_elem in listing:
            t, name = tuple(list_elem.split())
            if t == "dir":
                self.add_child(Directory(name, self, []))
            else:
                self.add_child(File(name, self, int(t)))

        while remaining:
            # let children parse
            _, command, name = tuple(remaining[0].split())
            if command == "cd" and name == "..":
                return remaining[1::]
            if command == "cd":
                child_dir = [d for d in self.children if d.name == name][0]
                remaining = child_dir.parse_shell(remaining[1::])

    def get_directories(self) -> List[Node]:
        return [self] + [d for c in self.children for d in c.get_directories()]


@dataclass
class File(Node):
    name: str
    parent: Directory
    size: int

    def get_size(self) -> int:
        return self.size

    def add_child(self, n: Node):
        raise Exception('Adding child to file is invalid')

    def parse_shell(self, shell: List[str]):
        raise Exception('parse_shell called on File.')

    def get_directories(self) -> List['Node']:
        return []


with open('input.txt', 'r') as inp:
    output = [l.strip() for l in inp.readlines()]

    root = Directory('/', None, [])
    root.parse_shell(output[1::])
    print(sum(d.get_size() for d in root.get_directories() if d.get_size() <= 100000))

    MAX_SPACE = 40_000_000
    current_size = root.get_size()
    candidates = [d for d in root.get_directories() if d.get_size() >= current_size - MAX_SPACE]
    candidate = sorted(candidates, key=lambda d: d.get_size())[0]
    print(f"Deleting {candidate.name} would free {candidate.get_size()} space.")
