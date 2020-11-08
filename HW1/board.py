from __future__ import annotations

from typing import List
from math import sqrt
from enum import Enum
import numpy as np

from node import Node
Point = np.array
Row = List[int]


def is_location_valid(current_location: Point, board_size: int):
    return 0 <= current_location[0] < board_size and 0 <= current_location[1] < board_size


def add_points(a: Point, b: Point) -> Point:
    return a + b


def manhattan_distance(p1: Point, p2: Point) -> int:
    return np.abs(p1[0] - p2[0]) + np.abs(p1[1] - p2[1])


class Move(Enum):
    UP = (-1, 0)
    DOWN = (1, 0)
    LEFT = (0, -1)
    RIGHT = (0, 1)


class Board:
    seen_boards = set()

    def __init__(self, board: np.array, zero: Point, score: int = -1):
        self.board = board
        self.zero = zero
        self.size = (len(self.board))
        self.score = score
        self.parent_move = None

    def copy(self) -> Board:
        return Board(self.board.copy(), self.zero, self.score)

    @staticmethod
    def user_input_board(n: int):
        new_board_list: List[Row] = []

        row_count = int(sqrt(n + 1))
        empty_index: Point = np.array([-1, -1])
        for i in range(row_count):
            new_board_list.append([int(item) for item in input().split(' ')])

        for i, row in enumerate(new_board_list):
            for j, item in enumerate(row):
                if item == 0:
                    empty_index = np.array([i, j])

        return Board(np.array(new_board_list), empty_index)

    def __create_new_board(self, new_zero: Point, move: Move) -> Board:
        new_board = self.copy()
        current_zero = new_board.zero

        new_board.board[current_zero[0]][current_zero[1]] = new_board.board[new_zero[0]][new_zero[1]]
        new_board.board[new_zero[0]][new_zero[1]] = 0
        new_board.zero = new_zero

        new_board.parent_move = move
        new_board.__evaluate_parent(self)

        return new_board

    def get_next_moves(self) -> List[Board]:
        result = []
        for offset in [Move.RIGHT, Move.LEFT, Move.DOWN, Move.UP]:
            next_position = add_points(self.zero, np.array([offset.value[0], offset.value[1]]))

            if is_location_valid(next_position, self.size):
                new_board = self.__create_new_board(next_position, offset)
                result.append(new_board)

        return result

    def __convert_index_to_point(self, index: int) -> Point:
        index -= 1
        return index // self.size, index % self.size

    def __evaluate_alone(self) -> int:
        score = 0
        for i in range(self.size):
            for j in range(self.size):
                if self.board[i][j] != 0:
                    target_position = self.__convert_index_to_point(self.board[i][j])
                    score += manhattan_distance((i, j), target_position)
        return score

    def __evaluate_parent(self, parent: Board):
        moved_element = self.board[parent.zero[0]][parent.zero[1]]
        self.score -= manhattan_distance(self.zero, parent.__convert_index_to_point(moved_element))
        self.score += manhattan_distance(parent.zero, self.__convert_index_to_point(moved_element))

    def evaluate(self, parent: Board = None) -> int:
        if parent is None:
            self.score = self.__evaluate_alone()
        else:
            self.__evaluate_parent(parent)

        return self.score

    def print(self):
        print("Score: {}".format(self.score))

        for row in self.board:
            for item in row:
                print(item, end=' ')
            print("\n", end="")

    def __count_inversion(self) -> int:
        flat_list = [item for row in self.board for item in row]

        inversion_count = 0

        for i in range(len(flat_list)):
            for j in range(i + 1, len(flat_list)):
                if flat_list[i] > flat_list[j]:
                    inversion_count += 1

        return inversion_count

    def is_board_solvable(self) -> bool:
        if self.size % 2 == 0:
            return self.__count_inversion() + self.zero[0] % 2 == 1
        else:
            return self.__count_inversion() % 2 == 0
