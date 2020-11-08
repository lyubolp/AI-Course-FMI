class Node:
    def __init__(self, board):
        self.board = board
        self.parent = None
        self.children = []

    def add_child(self, value):
        child = Node(value)
        child.parent = self
        self.children.append(child)

        return child

    def build_next_level(self):
        for child in self.board.get_next_moves():
            self.add_child(child)
