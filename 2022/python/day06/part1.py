def main():
    with open("input.txt") as f:
        i = f.read()
    s = ""
    for inx, c in enumerate(i):
        if c in s:
            s = s.split(c)[-1]
            s += c
        else:
            s += c
            if len(s) == 4:
                print(inx + 1)
                return


if __name__ == '__main__':
    main()
