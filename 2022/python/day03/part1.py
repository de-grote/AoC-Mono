def main():
    priorities = 0
    with open("input.txt") as f:
        for i in f.readlines():
            mid = len(i) // 2
            c = ord((set(i[mid:]) & set(i[:mid])).pop())
            priorities += c - 38 if c <= 90 else c - 96
    print(priorities)


if __name__ == '__main__':
    main()
