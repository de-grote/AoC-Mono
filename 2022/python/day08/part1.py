import itertools


def main():
    with open("input.txt") as f:
        inp = [[int(j) for j in i.rstrip()] for i in f.readlines()]
    visible = 0
    size_x = len(inp[0])
    size_y = len(inp)
    for x, y in itertools.product(range(size_x), range(size_y)):
        height = inp[y][x]
        visible += any((
            all(inp[h][x] < height for h in range(y)),
            all(inp[h][x] < height for h in range(y + 1, size_y)),
            all(inp[y][h] < height for h in range(x)),
            all(inp[y][h] < height for h in range(x + 1, size_x))
        ))
    print(visible)


if __name__ == '__main__':
    main()
