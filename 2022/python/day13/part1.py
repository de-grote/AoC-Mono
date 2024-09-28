import json
import itertools


def compare(left, right) -> int:
    left_is_list = isinstance(left, list)
    right_is_list = isinstance(right, list)
    if not left_is_list and not right_is_list:
        if left == right:
            return 0
        if left < right:
            return 1
        return -1
    if left_is_list is not right_is_list:
        if left_is_list:
            return compare(left, [right])
        return compare([left], right)
    for i, j in itertools.zip_longest(left, right, fillvalue=None):
        if i is None:
            return 1
        if j is None:
            return -1
        c = compare(i, j)
        if c == -1 or c == 1:
            return c


def main():
    inx = 1
    out = 0
    with open("input.txt") as f:
        it = iter(f.read().splitlines())
    for i in it:
        if i.rstrip() == "":
            continue
        left = json.loads(i)
        s = next(it)
        right = json.loads(s)
        if compare(left, right) != -1:
            out += inx
        inx += 1
    print(out)


if __name__ == '__main__':
    main()
