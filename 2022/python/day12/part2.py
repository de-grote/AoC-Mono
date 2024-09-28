def main():
    with open("input.txt") as f:
        grid = [[ord(j.replace("S", chr(97)).replace("E", chr(123))) - 96 for j in i.rstrip()] for i in f.readlines()]
    size_y = len(grid)
    size_x = len(grid[0])
    q: set[tuple[int, int]] = set()
    for i, g in enumerate(grid):
        if 27 in g:
            q.add((g.index(27), i))
            grid[i][g.index(27)] = 26
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
                if 0 <= i < size_x and 0 <= j < size_y and (i, j) not in been and grid[y][x] <= grid[j][i] + 1:
                    new.add((i, j))
                    if grid[j][i] == 1:
                        print(steps)
                        return
        q = new


if __name__ == '__main__':
    main()
