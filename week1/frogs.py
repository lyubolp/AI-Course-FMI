from typing import List, NewType
Field = NewType('Field', List[int])
State = NewType('State', (Field, int))

class Node:
    def __init__(self, value: State):
        self.value = value
        self.children = []
        self.parent = None

    def append(self, child: State):
        new_node = Node(child)
        new_node.parent = self
        self.children.append(new_node)
        return new_node

def build_init_array(n) -> State:
    result_array = [0] * (2*n+1)

    for i in range(n):
        result_array[i] = 1
        result_array[-(i+1)] = -1

    return (result_array, n)

def get_next_moves(current_state: State) -> List[State]:
    result = []
    for current_index in [current_state[1] - 2, current_state[1] - 1, current_state[1] + 1, current_state[1] + 2]:
        for move_offset in [1, 2]:

            if current_index < 0 or current_index > len(current_state[0]) - 1:
                continue

            target_index = current_index + (move_offset * current_state[0][current_index])
            if target_index < 0 or target_index > len(current_state[0]) - 1:
                continue

            if current_state[0][target_index] == 0:
                new_state = current_state[0].copy()
                new_state[current_index], new_state[target_index] = new_state[target_index], new_state[current_index] 
                result.append((new_state, current_index))
    return result

def build_tree(init_state: State) -> Node:
    root = Node(init_state)
    state_queue = [(root, init_state)]

    while len(state_queue) != 0:
        current_node, current_state = state_queue.pop(0)

        for next_state in get_next_moves(current_state):
            new_node = current_node.append(next_state)
            state_queue.append((new_node, next_state))

    return root


def dfs(root: Node, expected_result: State) -> Node:
    
    stack = []

    stack.append(root)

    while len(stack) != 0:
        current_node = stack.pop()
        if current_node.value[0] == expected_result:
            return current_node
        
        for child in current_node.children:
            stack.append(child)

    return None

if __name__ == "__main__":
    n = int(input())
    start = build_init_array(n)
    
    root = build_tree(start)
    expected_state = [0] * (2*n + 1)

    for i in range(n):
        expected_state[i] = -1
        expected_state[-(i+1)] = 1
    
    result_leaf = dfs(root, expected_state)
    print(result_leaf)
    
    while result_leaf.parent != None:
        print(result_leaf.value[0])
        result_leaf = result_leaf.parent
    print(result_leaf.value[0])

