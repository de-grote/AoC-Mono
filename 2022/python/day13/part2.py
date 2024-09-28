import json
import itertools
import functools


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
    with open("input.txt") as f:
        it = [json.loads(i) for i in f.read().splitlines() if i]
    it.extend(([[2]], [[6]]))
    it.sort(key=functools.cmp_to_key(compare), reverse=True)
    print((it.index([[2]]) + 1) * (it.index([[6]]) + 1))


if __name__ == '__main__':
    main()
