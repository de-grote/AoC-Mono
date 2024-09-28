def main():
    p = set()
    with open("input.txt") as f:
        for i in f.readlines():
            a, b = i.rstrip().removeprefix("Sensor at x=").split(": closest beacon is at x=")
            x, y = map(int, a.split(", y="))
            bx, by = map(int, b.split(", y="))
            dist = abs(x - bx) + abs(y - by)
            p.add((x, y, dist))
    for x, y, dist in p:
        for dx in range(dist+2):
            dy = dx - dist - 1
            for a, b in ((1, -1), (-1, 1), (1, 1), (-1, -1)):
                tx, ty = a*(x+dx), b*(y+dy)
                if 0 <= tx < 4_000_000 and 0 <= ty <= 4_000_000:
                    if all(abs(tx - pos[0]) + abs(ty - pos[1]) > pos[2] for pos in p):
                        print(tx * 4_000_000 + ty)
                        return


if __name__ == '__main__':
    main()
