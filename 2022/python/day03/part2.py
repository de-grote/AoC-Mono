def main():
    priorities = 0
    count = 0
    s = set()
    with open("input.txt") as f:
        for i in f.readlines():
            if count == 0:
                s.update(i.strip())
                count += 1
            else:
                s &= set(i)
                count += 1
                if count == 3:
                    c = ord(s.pop())
                    priorities += c - 38 if c <= 90 else c - 96
                    count = 0
    print(priorities)


if __name__ == '__main__':
    main()
