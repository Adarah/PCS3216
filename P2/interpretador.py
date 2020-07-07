import argparse
from pathlib import Path
import os
import re
import sys
from collections import OrderedDict
from dataclasses import dataclass
from enum import Enum
from typing import Callable, Dict, Iterator, List

### UTILS

def get_next_word(char_stream: Iterator[str]) -> str:
    word = ""
    for c in char_stream:
        if c == " " or c == "\n":
            break
        word += c
    return word


def make_word_stream(filename: str) -> Iterator[str]:
    with open(filename) as f:
        for word in f.read().replace("\n", " ").split(" "):
            if len(word) == 0:
                continue
            yield word


def make_line_stream(filename: str) -> Iterator[List[str]]:
    with open(filename) as f:
        for line in f.readlines():
            uncomment = line.split("#")[0]
            words = uncomment.replace("\n", "").split(" ")
            clean_words = [word for word in words if word != ""]
            if len(clean_words) == 0:
                continue
            yield clean_words


@dataclass
class Line:
    keyword: str
    expression: List[str]

    def __str__(self):
        return self.keyword + " " + " ".join(self.expression)


def preprocess(filename: str) -> Dict[str, Line]:
    line_stream = make_line_stream(filename)
    keywords = set(["LET", "GOTO", "READ", "WRITE"])
    d = OrderedDict()
    for line_num, line in enumerate(line_stream):
        if line[0] in keywords:
            d[str(line_num)] = Line(line[0], line[1:])
        elif line[1] == ":" and line[2] in keywords:
            # label : keyword expression expression expression
            # index 1 is a colon
            d[line[0]] = Line(line[2], line[3:])
        else:
            e = SyntaxError("Expected statement or label, got neither")
            l = Line("", line)
            make_graceful(l, e)

    return d


def make_graceful(line: Line, e: Exception):
    print(line)
    print(type(e).__name__, end=": ")
    print(e)
    sys.exit(1)

#### UTILS END
#### The reason these functions are not in a separate file is that the "interpreter"
#### should be a standalone module per requirements


class State(Enum):
    LET = "LET"
    GOTO = "GOTO"
    READ = "READ"
    WRITE = "WRITE"


class Interpreter:
    def __init__(self, input_file: str, output_file: str):
        self.state: State = State.LET
        self.line_stream: Iterator = make_line_stream(
            input_file
        )
        self.buffer: str = ""
        self.code = preprocess(input_file)
        self.variables: Dict[str, float] = {}
        self.line_num: int = 0
        self.inp: Iterator = make_word_stream("disk/inputs/interpreter_input.txt")
        self.out: str = output_file
        if not Path(output_file).parent.exists():
            os.makedirs(Path(output_file).parent)
        with open(self.out, "w+"):
            pass

    def transition(self, current_state: State, line: Line) -> State:
        if line.keyword in State.__members__:
            return State.__members__[line.keyword]
        else:
            print(line)
            raise NotImplementedError("Unknown keyword")

    def decode(self, state: State) -> Callable:
        transitions = {
            State.LET: self.assign,
            State.GOTO: self.goto,
            State.READ: self.read,
            State.WRITE: self.write,
            # State.LABEL: self.add_label,
        }
        return transitions[state]

    def execute(self):
        while self.line_num < len(self.code):
            line = list(self.code.values())[self.line_num]
            self.state = self.transition(self.state, line)
            foo = self.decode(self.state)
            try:
                self.line_num += 1
                foo(line)
            except Exception as e:
                make_graceful(line, e)

    def is_valid_name(self, name: str) -> bool:
        # this looks like bad code, but re.search doesn't actually return a boolean
        # If no match is found, it returns None
        if re.search(r"^[A-Za-z][\w]*$", name):
            return True
        else:
            return False

    def assign(self, line: Line):
        if self.is_valid_name(line.expression[0]) and line.expression[1] == "=":
            if line.expression[2] in self.variables:
                self.variables[line.expression[0]] = self.variables[line.expression[2]]
            else:
                try:
                    # TODO: change eval to valid state machine
                    self.variables[line.expression[0]] = eval(
                        " ".join(line.expression[2:])
                    )
                except:
                    raise NameError("Reference to uninitiliazed variable")
        else:
            raise SyntaxError("Invalid assignment")

    def goto(self, line: Line):
        if len(line.expression) == 1:
            self.line_num = list(self.code.keys()).index(line.expression[0])
        elif line.expression[1] == "IF":
            if self.is_expression_true(line.expression):
                self.line_num = list(self.code.keys()).index(line.expression[0])
        else:
            raise SyntaxError("Invalid jump")

    def is_expression_true(self, expression: List[str]) -> bool:
        comp_idx = self.find_comparator_index(expression)
        expression = list(expression)  # shallow copy
        for idx, i in enumerate(expression):
            if i in self.variables:
                expression[idx] = str(self.variables[i])
        try:
            LHS = eval(" ".join(expression[2:comp_idx]))
            comparator = expression[comp_idx]
            RHS = eval(" ".join(expression[comp_idx + 1 :]))
        except:
            raise SyntaxError("Invalid Expression")

        if comparator == "=":
            return LHS == RHS
        elif comparator == ">":
            return LHS > RHS
        elif comparator == "<":
            return LHS < RHS
        else:
            raise NotImplementedError("Unknown comparator")

    def find_comparator_index(self, expr: List[str]) -> int:
        valid_comparators = set(["<", ">", "="])
        for idx, i in enumerate(expr):
            if i in valid_comparators:
                return idx
        raise SyntaxError("Expected boolean expression, but found no comparators")

    def read(self, line: Line):
        if self.is_valid_name(line.expression[0]):
            self.variables[line.expression[0]] = eval(next(self.inp))

    def write(self, line: Line):
        if (val := self.variables.get(line.expression[0])) is not None:
            with open(self.out, "a") as f:
                f.write(str(val) + "\n")
        else:
            raise NameError("Reference to uninitiliazed variable")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="A simple programming language interpreter"
    )
    parser.add_argument(
        "OUTPUT", type=str, help="The file where output will be written to"
    )
    parser.add_argument("INPUT", type=str, help="The file where input will come from")

    args = parser.parse_args()
    i = Interpreter(input_file=args.INPUT, output_file=args.OUTPUT)
    i.execute()
