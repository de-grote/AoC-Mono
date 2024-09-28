def main():
    illegal_positions = set()
    with open("input.txt") as f:
        for i in f.readlines():
            a, b = i.rstrip().removeprefix("Sensor at x=").split(": closest beacon is at x=")
            x, y = map(int, a.split(", y="))
            bx, by = map(int, b.split(", y="))
            dist = abs(x - bx) + abs(y - by)
            dy = abs(y - 2_000_000)
            dx = dist - dy
            for p in range(x-dx, x+dx):
                illegal_positions.add(p)
    print(len(illegal_positions))


if __name__ == '__main__':
    main()
