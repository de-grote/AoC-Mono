def main():
    m = [0, 0, 0]
    c = 0
    with open("input.txt") as f:
        for i in f.readlines():
            if i.strip() == "":
                if c > min(m):
                    m.remove(min(m))
                    m.append(c)
                c = 0
            else:
                c += int(i.strip())
    print(sum(m))


if __name__ == '__main__':
    main()
