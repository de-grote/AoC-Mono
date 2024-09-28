def main():
    with open("input.txt") as f:
        grid = [[ord(j.replace("S", chr(96)).replace("E", chr(123))) - 96 for j in i.rstrip()] for i in f.readlines()]
    size_y = len(grid)
    size_x = len(grid[0])
    q: set[tuple[int, int]] = set()
    end = (0, 0)
    for i, g in enumerate(grid):
        if 27 in g:
            end = (g.index(27), i)
            grid[i][g.index(27)] = 26
            break
    for i, g in enumerate(grid):
        if 0 in g:
            q.add((g.index(0), i))
            grid[i][g.index(0)] = 1
            break
    steps = 0
    been = set()
    while True:
        steps += 1
        been.update(q)
        new: set[tuple[int, int]] = set()
        for x, y in q:
            for i, j in ((0, 1), (1, 0), (0, -1), (-1, 0)):
                i += x
                j += y
                if 0 <= i < size_x and 0 <= j < size_y and (i, j) not in been and grid[j][i] <= grid[y][x] + 1:
                    new.add((i, j))
                    if (i, j) == end:
                        print(steps)
                        return
        q = new


if __name__ == '__main__':
    main()
