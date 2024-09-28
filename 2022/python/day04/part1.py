def main():
    overlaps = 0
    with open("input.txt") as f:
        for i in f.readlines():
            b1, e1, b2, e2 = map(int, i.rstrip().replace(",", "-").split("-"))
            overlaps += (b1 <= b2 and e1 >= e2) or (b2 <= b1 and e2 >= e1)
    print(overlaps)


if __name__ == '__main__':
    main()
