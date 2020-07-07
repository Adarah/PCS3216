import sys
from collections import OrderedDict
from dataclasses import dataclass
from typing import Dict, Iterator, List


def make_char_stream(filename: str) -> Iterator[str]:
    with open(filename) as f:
        return (letter for letter in f.read())


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
