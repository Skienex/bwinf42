from typing import Callable

CELL_EMPTY = 0  # X
CELL_WHITE = 1  # W
CELL_RED = 2  # r
CELL_RED_WITH_INPUT = 3  # R
CELL_BLUE = 4  # B
CELL_INPUT = 5  # Q
CELL_OUTPUT = 6  # L

CELL_NAMES = {
    CELL_EMPTY: 'X',
    CELL_WHITE: 'W',
    CELL_RED: 'r',
    CELL_RED_WITH_INPUT: 'R',
    CELL_BLUE: 'B',
    CELL_INPUT: 'Q',
    CELL_OUTPUT: 'L',
}

STATE_UNKNOWN = -1
STATE_INACTIVE = 0
STATE_ACTIVE = 1

PROCESSORS: dict[int, Callable[[int, int], int]] = {
    CELL_EMPTY: (lambda this, that: STATE_UNKNOWN),
    CELL_WHITE: (lambda this, that: negate(state_and(this, that))),
    CELL_RED: (lambda this, that: negate(that)),
    CELL_RED_WITH_INPUT: (lambda this, that: negate(this)),
    CELL_BLUE: (lambda this, that: this),
}


def negate(state: int) -> int:
    if state == STATE_ACTIVE:
        return STATE_INACTIVE
    if state == STATE_INACTIVE:
        return STATE_ACTIVE
    return STATE_INACTIVE


def state_and(left: int, right: int) -> int:
    if left == STATE_ACTIVE and right == STATE_ACTIVE:
        return STATE_ACTIVE
    return STATE_INACTIVE


class Cell:
    def __init__(self, cell_type: int):
        self.cell_type = cell_type
        self.neighbor: int | None = None

    @staticmethod
    def input(index: int):
        cell = Cell(CELL_INPUT)
        cell.index = index
        return cell

    @staticmethod
    def output(index: int):
        cell = Cell(CELL_OUTPUT)
        cell.index = index
        return cell

    def process(self, this: int, that: int) -> int:
        return PROCESSORS[self.cell_type](this, that)

    def can_connect(self, other) -> bool:
        return (self.cell_type == CELL_WHITE and other.cell_type == CELL_WHITE) or (
                self.cell_type == CELL_BLUE and other.cell_type == CELL_BLUE) or (
                self.cell_type == CELL_RED and other.cell_type == CELL_RED_WITH_INPUT) or (
                self.cell_type == CELL_RED_WITH_INPUT and other.cell_type == CELL_RED)


class Matrix:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.output_count = 0
        self.rows: list[MatrixRow] = []

    def connect(self):
        for x in range(self.width):
            for y in range(self.height):
                cell = self.rows[y].cells[x]
                if cell.neighbor is not None:
                    continue
                if cell.cell_type in [CELL_EMPTY, CELL_INPUT, CELL_OUTPUT]:
                    continue
                if x + 1 >= self.width:
                    continue
                next_cell = self.rows[y].cells[x + 1]
                if next_cell.neighbor is not None:
                    continue
                if cell.can_connect(next_cell):
                    cell.neighbor = x + 1
                    next_cell.neighbor = x

    def process(self, inputs: list[int]) -> list[int]:
        states = [[STATE_UNKNOWN for _ in range(self.width)] for _ in range(self.height)]
        outputs = [STATE_UNKNOWN for _ in range(self.output_count)]
        for i_cell, cell in enumerate(self.rows[0].cells):
            if cell.cell_type == CELL_INPUT:
                states[0][i_cell] = inputs[cell.index]
        for i_row, row in enumerate(self.rows[1:]):
            for i_cell, cell in enumerate(row.cells):
                this = states[i_row][i_cell]
                if cell.cell_type == CELL_OUTPUT:
                    outputs[cell.index] = this
                    continue
                that = states[i_row][cell.neighbor] if cell.neighbor is not None else STATE_UNKNOWN
                states[i_row + 1][i_cell] = cell.process(this, that)
            # print(states[i_row])
        return outputs


class MatrixRow:
    def __init__(self):
        self.cells: list[Cell] = []


def parse_matrix(src: str):
    starts = 0
    lines = iter(src.splitlines(keepends=False))
    size = next(lines).split(' ')
    width = int(size[0])
    height = int(size[1])
    matrix = Matrix(width, height)
    for line in lines:
        row = MatrixRow()
        for cell_s in map(lambda it: it.strip(), filter(lambda it: len(it) > 0, line.split(' '))):
            cell: Cell
            match cell_s[0]:
                case 'Q':
                    index = int(cell_s[1:]) - 1
                    cell = Cell.input(index)
                    starts += 1
                case 'L':
                    index = int(cell_s[1:]) - 1
                    cell = Cell.output(index)
                    if matrix.output_count <= index:
                        matrix.output_count = index + 1
                case 'X':
                    cell = Cell(CELL_EMPTY)
                case 'W':
                    cell = Cell(CELL_WHITE)
                case 'R':
                    cell = Cell(CELL_RED_WITH_INPUT)
                case 'r':
                    cell = Cell(CELL_RED)
                case 'B':
                    cell = Cell(CELL_BLUE)
                case _:
                    raise ValueError(f'Invalid cell: {cell_s}')
            row.cells.append(cell)
        matrix.rows.append(row)
    matrix.connect()
    return matrix, starts


def get_file(path):
    # Open file in path
    with open(path, 'r') as file:
        output = file.read().strip()
    return output

def calculate(file): # Only for Server
    result = []
    source = get_file(file)
    mat, starts = parse_matrix(source)
    possible_starts = 2 ** starts
    for j in range(possible_starts):
        possible_start = bin(j)[2:].zfill(starts)
        current_start = [int(char) for char in possible_start]
        inputs = current_start
        outputs = mat.process(inputs)
        result.append([inputs, outputs])
    return result


if __name__ == '__main__':
    for i in range(5):
        result = []
        print(f"\n\nDatei {i + 1}:")
        source = get_file(f"nandu{i + 1}.txt")
        mat, starts = parse_matrix(source)
        print(f"mat: {mat}, starts: {starts}")
        possible_starts = 2 ** starts
        for j in range(possible_starts):
            possible_start = bin(j)[2:].zfill(starts)
            current_start = [int(char) for char in possible_start]
            inputs = current_start
            outputs = mat.process(inputs)
            print(f"Input: {inputs} | Output: {outputs}")