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
    elephant_position = ["AA"]
    max_so_far = 0
    max_open = sum(bool(m["rate"]) for m in d.values())
    max_flow_rate = max(m["rate"] for m in d.values())

    def dfs(depth=0, score=0) -> int:
        nonlocal max_so_far
        speed = sum(opened.values())
        score += speed
        if depth == 25:
            return max(score, max_so_far)
        # changing this line is basically all the debugging today
        if score + ((speed + max_flow_rate * (26 - depth) / (depth or 1)) * (26 - depth)) < max_so_far - 70:
            return max_so_far
        m = 0
        if position[-1] not in opened and d[position[-1]]["rate"] != 0:
            opened[position[-1]] = d[position[-1]]["rate"]
            m = move_elephant(depth, score)
            opened.pop(position[-1])
        for p in sorted(d[position[-1]]["next"], key=lambda x: 2 * (x in opened) + (x in position or x in elephant_position)):
            position.append(p)
            m = max(m, move_elephant(depth, score))
            position.pop()
        if m > max_so_far:
            max_so_far = m
            print(m)
        return m

    def move_elephant(depth, score):
        m = 0
        if elephant_position[-1] not in opened and d[elephant_position[-1]]["rate"] != 0:
            opened[elephant_position[-1]] = d[elephant_position[-1]]["rate"]
            m = dfs(depth + 1, score)
            opened.pop(elephant_position[-1])
        for p in sorted(d[elephant_position[-1]]["next"], key=lambda x: 2 * (x in opened) + (x in position or x in elephant_position)):
            elephant_position.append(p)
            m = max(m, dfs(depth + 1, score))
            elephant_position.pop()
        return m

    print(dfs())


if __name__ == '__main__':
    main()
