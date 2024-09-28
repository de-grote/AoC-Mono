import itertools


def count(it, height):
    c = 0
    for i in it:
        if i >= height:
            c += 1
            break
        c += 1
    return c


def main():
    with open("input.txt") as f:
        inp = [[int(j) for j in i.rstrip()] for i in f.readlines()]
    best = 0
    size_x = len(inp[0])
    size_y = len(inp)
    for x, y in itertools.product(range(size_x), range(size_y)):
        height = inp[y][x]
        best = max(best, (
           count((inp[h][x] for h in reversed(range(y))), height) *
           count((inp[h][x] for h in range(y + 1, size_y)), height) *
           count((inp[y][h] for h in reversed(range(x))), height) *
           count((inp[y][h] for h in range(x + 1, size_x)), height)
        ))
    print(best)


if __name__ == '__main__':
    main()
