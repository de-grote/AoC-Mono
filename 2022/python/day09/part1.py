def main():
    head_x, head_y = 0, 0
    tail_x, tail_y = 0, 0
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
            if head_x - tail_x == 0 and abs(head_y - tail_y) == 2:
                tail_y += 1 if head_y > tail_y else -1
            elif head_y - tail_y == 0 and abs(head_x - tail_x) == 2:
                tail_x += 1 if head_x > tail_x else -1
            elif not (abs(head_y - tail_y) <= 1 and abs(head_x - tail_x) <= 1):
                tail_x += 1 if head_x > tail_x else -1
                tail_y += 1 if head_y > tail_y else -1
            been.add((tail_x, tail_y))
    print(len(been))


if __name__ == '__main__':
    main()
