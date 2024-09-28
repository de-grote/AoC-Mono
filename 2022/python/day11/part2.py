import math


def main():
    monkeys = []
    with open("input.txt") as f:
        for i in f.readlines():
            if i.startswith("M"):
                d = {}
            elif "items" in i:
                d["items"] = [int(j) for j in i.split(": ")[-1].split(", ")]
            elif "Operation" in i:
                d["operation"] = eval(f"lambda old:{i.split('=')[-1]}")
            elif "Test" in i:
                d["mod"] = int(i.split("by ")[-1])
            elif "true" in i:
                d["true"] = int(i.split("y ")[-1])
            elif "false" in i:
                d["false"] = int(i.split("y ")[-1])
            else:
                monkeys.append(d)
        monkeys.append(d)
    inspects = [0 for _ in range(len(monkeys))]
    rage = math.lcm(*(i["mod"] for i in monkeys))
    for _ in range(10000):
        for n, monkey in enumerate(monkeys):
            while monkey["items"]:
                inspects[n] += 1
                item = monkey["operation"](monkey["items"].pop()) % rage
                if item % monkey["mod"] == 0:
                    monkeys[monkey["true"]]["items"].append(item)
                else:
                    monkeys[monkey["false"]]["items"].append(item)
    sort = sorted(inspects)
    print(sort[-1]*sort[-2])


if __name__ == '__main__':
    main()
