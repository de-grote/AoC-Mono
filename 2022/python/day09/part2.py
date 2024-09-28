def main():
    head_x, head_y = 0, 0
    tail_xs, tail_ys = [0] * 9, [0] * 9
    been = {(0, 0)}
    with open("input.txt") as f:
        inp = [(i[0], int(i[2:])) for i in f.readlines()]
    for i in inp:
        for _ in range(i[1]):
            if i[0] == "U":
                head_y += 1
            elif i[0] == "D":
                head_y -= 1
            elif i[0] == "R":
                head_x += 1
            else:
                head_x -= 1
            for inx, (tail_x, tail_y) in enumerate(zip(tail_xs, tail_ys)):
                prev_x = tail_xs[inx-1] if inx != 0 else head_x
                prev_y = tail_ys[inx-1] if inx != 0 else head_y
                if prev_x - tail_x == 0 and abs(prev_y - tail_y) == 2:
                    tail_ys[inx] += 1 if prev_y > tail_y else -1
                elif prev_y - tail_y == 0 and abs(prev_x - tail_x) == 2:
                    tail_xs[inx] += 1 if prev_x > tail_x else -1
                elif not (abs(prev_y - tail_y) <= 1 and abs(prev_x - tail_x) <= 1):
                    tail_xs[inx] += 1 if prev_x > tail_x else -1
                    tail_ys[inx] += 1 if prev_y > tail_y else -1
                if inx == 8:
                    been.add((tail_xs[8], tail_ys[8]))
    print(len(been))


if __name__ == '__main__':
    main()
