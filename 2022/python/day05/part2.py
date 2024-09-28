import re


def main():
    crates = [[] for _ in range(9)]
    with open("input.txt") as f:
        for _ in range(9):
            for i in re.finditer(r"\[(\w)]", f.readline()):
                crates[i.start() // 4].append(i.group(1))
        crates = [i[::-1] for i in crates]
        f.readline()
        for i in f.readlines():
            move, frm, to = map(int, i.removeprefix("move ").replace("from", "to").split(" to "))
            crates[to - 1].extend(crates[frm - 1][-move:])
            crates[frm - 1] = crates[frm - 1][:-move]
    print("".join(c[-1] for c in crates))


if __name__ == '__main__':
    main()
