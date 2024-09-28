import itertools


def main():
    collision = set()
    lowest_point = 0
    with open("input.txt") as f:
        for i in f.readlines():
            prev = None
            for j in (tuple(map(int, j.split(","))) for j in i.rstrip().split(" -> ")):
                lowest_point = max(lowest_point, j[1])
                collision.add(j)
                if prev is not None:
                    if j[0] == prev[0]:
                        for y in range(min(j[1], prev[1]), max(j[1], prev[1])):
                            collision.add((j[0], y))
                    else:
                        for x in range(min(j[0], prev[0]), max(j[0], prev[0])):
                            collision.add((x, j[1]))
                prev = j
    for salt in itertools.count(0):
        x, y = 500, 0
        while True:
            if (500, 0) in collision:
                print(salt)
                return
            if y == lowest_point + 1:
                collision.add((x, y))
                break
            if (x, y + 1) not in collision:
                y += 1
            elif (x - 1, y + 1) not in collision:
                y += 1
                x -= 1
            elif (x + 1, y + 1) not in collision:
                y += 1
                x += 1
            else:
                collision.add((x, y))
                break


if __name__ == '__main__':
    main()
