import os
import subprocess
import sys
from enum import Enum, auto
from pathlib import Path
from typing import Callable, Set

from utils import make_char_stream, get_next_word


class State(Enum):
    WAITING_DOLLAR = auto()
    WAITING_COMMAND = auto()
    JOB = auto()
    DISK = auto()
    DIRECTORY = auto()
    CREATE = auto()
    DELETE = auto()
    LIST = auto()
    INFILE = auto()
    OUTFILE = auto()
    DISKFILE = auto()
    RUN = auto()
    ENDJOB = auto()


class ControleMVN:
    def __init__(self):
        self.state: State = State.WAITING_DOLLAR
        self.char_stream: Iterator[str] = make_char_stream("test.txt")
        self.buffer: str = ""
        self.allowed_users: Set[str] = set(
            ["admin", "somebody",]
        )
        self.disk: Path = Path(".")
        self.inp: Path = None
        self.out: Path = None
        self.executable: Path = None

    ######
    # transition/execution logic
    #####

    def transition(self, current_state: State, arg: str) -> State:
        command_table = {
            "JOB": State.JOB,
            "DISK": State.DISK,
            "DIRECTORY": State.DIRECTORY,
            "CREATE": State.CREATE,
            "DELETE": State.DELETE,
            "LIST": State.LIST,
            "INFILE": State.INFILE,
            "OUTFILE": State.OUTFILE,
            "DISKFILE": State.DISKFILE,
            "RUN": State.RUN,
            "ENDJOB": State.ENDJOB,
        }

        if current_state == State.WAITING_DOLLAR:
            return State.WAITING_COMMAND
        elif current_state == State.WAITING_COMMAND:
            return command_table[arg]
        else:
            return State.WAITING_DOLLAR

    def decode(self, state: State) -> Callable:
        op_table = {
            State.WAITING_DOLLAR: self.find_dollar_sign,
            State.WAITING_COMMAND: self.get_command,
            State.JOB: self.login,
            State.DISK: self.set_disk,
            State.DIRECTORY: self.ls,
            State.CREATE: self.touch,
            State.DELETE: self.rm,
            State.LIST: self.cat,
            State.INFILE: self.set_input,
            State.OUTFILE: self.set_output,
            State.DISKFILE: self.set_executable,
            State.RUN: self.run,
            State.ENDJOB: self.end,
        }

        return op_table[state]

    def execute(self):
        # TODO: loop ends inside the find_dollar_sign function, probably should be changed
        while True:
            foo = self.decode(self.state)
            self.buffer = foo()
            self.state = self.transition(self.state, self.buffer)

    ######
    # routines
    #####

    def find_dollar_sign(self):
        for c in self.char_stream:
            if c == "$":
                break
        else:
            # TODO: delete this "raise"
            print("EOF before $END was found. Exiting now")
            sys.exit(1)

    def get_command(self):
        word = get_next_word(self.char_stream)
        return word

    def login(self):
        user = get_next_word(self.char_stream)
        if user not in self.allowed_users:
            print(f"{user} is not in the list of allowed users. Exiting now.")
            sys.exit(1)

    def set_disk(self):
        word = get_next_word(self.char_stream)
        self.disk = Path(word)

    def ls(self):
        print(os.listdir())

    def touch(self):
        filename = get_next_word(self.char_stream)
        Path(self.disk.joinpath(filename)).touch()

    def rm(self):
        filename = get_next_word(self.char_stream)
        Path(self.disk.joinpath(filename)).unlink()

    def cat(self):
        filename = get_next_word(self.char_stream)
        with open(self.disk.joinpath(filename)) as f:
            print(f.read())

    def set_input(self):
        filename = get_next_word(self.char_stream)
        self.inp = self.disk.joinpath(filename)

    def set_output(self):
        filename = get_next_word(self.char_stream)
        self.out = self.disk.joinpath(filename)

    def set_executable(self):
        filename = get_next_word(self.char_stream)
        self.executable = self.disk.joinpath(filename)

    def run(self):
        mode = get_next_word(self.char_stream)

        if mode == "assembler":
            code = subprocess.run(
                [f"{self.executable}", "assembler", self.out, self.inp]
            )
        elif mode == "cpu":
            loader_path = self.disk.joinpath("build").joinpath("loader.bin")
            code = subprocess.run(
                [
                    f"{self.executable}",
                    "cpu",
                    self.out,
                    self.inp,
                    "-L",
                    f"{loader_path}",
                ]
            )
        else:
            print("The sisprog rust binary only accepts two modes: cpu or assembler")
            raise ValueError

    def end(self):
        sys.exit(0)


if __name__ == "__main__":
    c = ControleMVN()
    c.execute()
