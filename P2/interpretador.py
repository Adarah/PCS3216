from enum import Enum
from typing import Callable, Iterator, List

from utils import make_line_stream, preprocess


class State(Enum):
    LET = "LET"
    GOTO = "GOTO"
    READ = "READ"
    WRITE = "WRITE"
    LABEL = "LABEL"


class Interpreter:
    def __init__(self):
        self.state: State = None
        self.line_stream: Iterator = make_line_stream(
            "disk/assets/interpreter_test.script"
        )
        self.buffer: str = ""
        self.pre = preprocess("disk/assets/interpreter_test.script")

    def transition(self, current_state: State, arg: str) -> State:
        if arg[0] in State.__members__:
            return State.__members__[arg[0]]
        return State.LABEL

    def decode(self, state: State) -> Callable:
        transitions = {
            State.LET: self.assign,
            State.GOTO: self.goto,
            State.READ: self.read,
            State.WRITE: self.write,
            State.LABEL: self.add_label,
        }
        return transitions[state]

    def execute(self):
        for line in self.line_stream:
            self.state = self.transition(self.state, line)
            foo = self.decode(self.state)
            foo(line)

    def assign(self, arg: List[str]):
        pass

    def goto(self, arg: List[str]):
        pass

    def read(self, arg: List[str]):
        pass

    def write(self, arg: List[str]):
        pass

    def add_label(self, arg: List[str]):
        pass


i = Interpreter()
i.execute()
