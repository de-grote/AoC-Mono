if __name__ == "__main__":
    with open("input15.txt", "r") as f:
        inp = [list(map(int, list(i.removesuffix("\n")))) for i in f.readlines()]
    q: dict = {}
    q.update(((i, j), float("inf")) for i in range(len(inp[0])) for j in range(len(inp)))
    q[(0, 0)] = 0
    while True:
        u = min((j, i) for i, j in q.items())
        print(u)
        for place in ((u[1][0] + 1, u[1][1]), (u[1][0], u[1][1] + 1), (u[1][0] - 1, u[1][1]), (u[1][0], u[1][1] - 1)):
            if place in q:
                q[place] = min(u[0] + inp[place[1]][place[0]], q[place])
        if u[1] == (len(inp[0]) - 1, len(inp) - 1):
            print(u[0])
            break
        del q[u[1]]
