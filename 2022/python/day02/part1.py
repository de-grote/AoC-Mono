def main():
    score = 0
    with open("input.txt") as f:
        for i in f.readlines():
            op, me = i.split()
            score += ord(me) - 87
            diff = ord(me) - ord(op)
            if diff == 23:
                score += 3
            elif diff == 21 or diff == 24:
                score += 6
    print(score)


if __name__ == '__main__':
    main()
