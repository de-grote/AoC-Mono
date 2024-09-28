if __name__ == "__main__":
    with open("input7.txt", "r") as f:
        crabs = [int(i) for i in f.readline().split(",")]
    out = float('inf')
    for c in range(min(crabs), max(crabs) + 1):
        out = min(sum(abs(crab - c) for crab in crabs), out)
    print(out)
