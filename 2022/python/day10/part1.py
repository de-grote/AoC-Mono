def main():
    cycle = 1
    x = 1
    out = 0
    with open("input.txt") as f:
        for i in f.readlines():
            i = i.rstrip()
            if cycle % 40 == 20 or cycle % 40 == 19 and i.startswith("a"):
                out += x * (cycle if cycle % 2 == 0 else cycle + 1)
            if i == "noop":
                cycle += 1
            else:
                x += int(i.split(" ")[-1])
                cycle += 2
    print(out)


if __name__ == '__main__':
    main()
