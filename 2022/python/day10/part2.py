def main():
    x = 1
    screen = [[] for _ in range(6)]
    pointer_x, pointer_y = 0, 0
    with open("input.txt") as f:
        for i in f.readlines():
            i = i.rstrip()
            if i != "noop":
                for _ in range(2):
                    screen[pointer_y].append("#" if abs(pointer_x - x) <= 1 else ".")
                    pointer_x += 1
                    if pointer_x == 40:
                        pointer_x = 0
                        pointer_y += 1
                x += int(i.split(" ")[-1])
            else:
                screen[pointer_y].append("#" if abs(pointer_x - x) <= 1 else ".")
                pointer_x += 1
                if pointer_x == 40:
                    pointer_x = 0
                    pointer_y += 1
    print("\n".join("".join(s) for s in screen))


if __name__ == '__main__':
    main()
