def main():
    score = 0
    with open("input.txt") as f:
        for i in f.readlines():
            op, res = i.split()
            if res == "Y":
                score += 3 + ord(op) - 64
            elif res == "Z":
                score += 7 + (ord(op) + 2) % 3
            else:
                score += 1 + ord(op) % 3
    print(score)


if __name__ == '__main__':
    main()
