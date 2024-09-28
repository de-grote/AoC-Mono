import re


def main():
    d: dict[str, dict[str, tuple | int]] = {}
    with open("input.txt") as f:
        for i in f.readlines():
            a, b = i.rstrip().split(" has flow rate=")
            c, e = re.split(r"; tunnels? leads? to valves? ", b)
            d[a.removeprefix("Valve ")] = {"rate": int(c), "next": tuple(e.split(", "))}
    opened = {}
    position = ["AA"]
    max_so_far = 0
    max_flow_rate = max(m["rate"] for m in d.values())

    def dfs(depth=0, score=0) -> int:
        nonlocal max_so_far
        speed = sum(opened.values())
        score += speed
        if depth == 29:
            return max(score, max_so_far)
        # black magic formula (lower the constant at the end for faster speeds but might break idk)
        if score + ((speed + max_flow_rate * (31 - depth) / (depth + 1)) * (30 - depth)) < max_so_far - 300:
            return max_so_far
        m = 0
        if position[-1] not in opened and d[position[-1]]["rate"] != 0:
            opened[position[-1]] = d[position[-1]]["rate"]
            m = dfs(depth + 1, score)
            opened.pop(position[-1])
        for p in sorted(d[position[-1]]["next"], key=lambda x: (x in opened) + (x in position)):
            position.append(p)
            m = max(m, dfs(depth + 1, score))
            position.pop()
        if m > max_so_far:
            max_so_far = m
        return m

    print(dfs())


if __name__ == '__main__':
    main()
