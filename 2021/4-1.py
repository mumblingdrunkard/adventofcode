def check_board(board, numbers):
    marked = []
    for row in board:
        marked_row = [1 if n in numbers else 0 for n in row]
        marked.append(marked_row)
    # do row sums
    winning_row = True in [sum(row) == 5 for row in marked]

    # do column sums
    winning_column = True in [sum(col) == 5 for col in zip(*marked)]
    return winning_row or winning_column

def find_winner(boards, numbers):
    for i in range(1, len(numbers)):
        for b, board in enumerate(boards):
            if check_board(board, numbers[:i]):
                return (b, numbers[:i])
    return []

def score(board, numbers):
    total = 0
    for row in board:
        row = [0 if n in numbers else n for n in row]
        total += sum(row)
    return total*numbers[-1]


numbers = [int(s) for s in input().split(',')]

boards = []

while True:
    board = []
    try:
        input()
        for r in range(5):
            row = [int(s) for s in input().split()]
            board.append(row)
        boards.append(board)
    except EOFError:
        break

(b, n) = find_winner(boards, numbers)
print(score(boards[b], n))
