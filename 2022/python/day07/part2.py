largest_delete = 1e309


def calc_size(directory, left_over=None):
    s = 0
    for k, v in directory.items():
        if k == "..":
            continue
        if isinstance(v, int):
            s += v
        else:
            s += calc_size(v, left_over)
    if left_over is not None and s > left_over:
        global largest_delete
        largest_delete = min(largest_delete, s)
    return s


def main():
    main_dir = {}
    directory = main_dir
    with open("input.txt") as f:
        for i in f.readlines():
            if i.startswith("$ c"):
                d = i.rstrip().rsplit(" ")[-1]
                directory.setdefault(d, {"..": directory})
                directory = directory[d]
            elif not i.startswith("$"):
                a, name = i.split(" ")
                if a.isdecimal():
                    directory[name] = int(a)
    calc_size(main_dir, calc_size(main_dir) - 40000000)
    print(largest_delete)


if __name__ == '__main__':
    main()
