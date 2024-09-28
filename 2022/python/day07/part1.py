small_dirs = 0


def calc_size(directory):
    s = 0
    for k, v in directory.items():
        if k == "..":
            continue
        if isinstance(v, int):
            s += v
        else:
            s += calc_size(v)
    if s < 100000:
        global small_dirs
        small_dirs += s
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
    calc_size(main_dir)
    print(small_dirs)


if __name__ == '__main__':
    main()
