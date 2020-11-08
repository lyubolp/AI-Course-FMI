from __future__ import annotations

import cProfile
from typing import List
from board import Board
from node import Node

Point = (int, int)
Row = List[int]


# def ids(root: Node, min_threshold: int) -> (bool, List[int], Node):
#     stack = []
#
#     for child in root.children:
#         stack.append(child)
#
#     thresholds = []
#
#     while len(stack) != 0:
#         current_element = stack.pop(0)
#
#         if current_element.board.score <= min_threshold:
#             thresholds.append(current_element.board.score)
#
#         if current_element.board.score == 0:
#             return True, [], current_element
#
#         if current_element.board.score >= min_threshold:
#             for child in current_element.children:
#                 stack.append(child)
#         if len(current_element.children) == 0:
#             current_element.build_next_level()
#
#     return False, thresholds, None
#
#
# def IDA_star(root: Node):
#     threshold = root.board.score
#
#     result = ids(root, threshold)
#
#     while not result[0]:
#         threshold = max([item for item in result[1] if item < threshold])
#         result = ids(root, threshold)
#
#     return result[2]


def IDA_star(root: Node):
    threshold = root.board.score

    while True:
        temp = search(root, 0, threshold)
        if isinstance(temp, Node):
            return temp

        threshold = temp


def search(node: Node, g: int, threshold: int):
    stack = [(node, g, 0)]
    max_threshold = 0
    while len(stack) != 0:
        current_node, current_g, current_level = stack.pop()

        f = current_g + current_node.board.score

        if f > threshold:
            max_threshold = max(max_threshold, f)
            continue

        if current_node.board.score == 0:
            print("Found solution at level {}".format(current_level))
            return current_node

        current_node.build_next_level()
        for child_node in current_node.children:
            stack.append((child_node, current_g + 1, current_level + 1))

    return max_threshold


if __name__ == '__main__':
    n = int(input())
    l = int(input())

    board: Board = Board.user_input_board(n)

    # if not board.is_board_solvable():
    #     print("Unsolvable")
    #     exit(0)

    board.evaluate()

    root: Node = Node(board)

    # print(IDA_star(root))
    # cProfile.run('solution = IDA_star(root)')

    # solution = IDA_star(root)
    cProfile.run('solution = IDA_star(root)')

    while solution.parent is not None:
        print(solution.board.parent_move)
        solution = solution.parent
