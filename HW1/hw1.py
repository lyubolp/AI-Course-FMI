from __future__ import annotations

import cProfile
import sys
from typing import List
from board import Board, pretty_print_move
from node import Node

Point = (int, int)
Row = List[int]


def IDA_star(root: Node):
    threshold = root.board.score

    while True:
        temp = search(root, 0, threshold)
        if isinstance(temp, Node):
            return temp

        threshold = temp


def search(node: Node, g: int, threshold: int):
    stack = [(node, g)]
    max_threshold = sys.maxsize
    while len(stack) != 0:
        current_node, current_depth = stack.pop()

        if current_node.board.score == -1:
            current_node.board.evaluate(current_node.parent.board)

        f = current_depth + current_node.board.score

        if f > threshold:
            max_threshold = min(max_threshold, f)
            continue

        if current_node.board.score == 0:
            print("Found solution at level {}".format(current_depth))
            return current_node

        if len(current_node.children) == 0:
            current_node.build_next_level()
        for child_node in current_node.children:
            stack.append((child_node, current_depth + 1,))

    return max_threshold


if __name__ == '__main__':
    n = int(input())
    l = int(input())

    board: Board = Board.user_input_board(n)

    board.evaluate()
    root: Node = Node(board)
    # solution = IDA_star(root)
    cProfile.run('solution = IDA_star(root)')

    reverse_result = []
    while solution.parent is not None:
        reverse_result.append(pretty_print_move(solution.board.parent_move))
        solution = solution.parent

    reverse_result.reverse()
    for item in reverse_result:
        print(item)
