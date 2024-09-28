def main():
    m = 0
    c = 0
    with open("input.txt") as f:
        for i in f.readlines():
            if i.strip() == "":
                m = max(m, c)
                c = 0
            else:
                c += int(i.strip())
    print(m)


if __name__ == '__main__':
    main()
