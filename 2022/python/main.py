import os
import pathlib


def main():
    day = int(input("Which day do you want to solve?: "))
    part = int(input("Do you want to solve part 1 or 2?: "))
    if part not in range(1, 3) or day not in range(1, 26):
        raise SystemExit("Invalid Input")
    day = f"day{day:0>2}"
    part = f"part{part}"
    try:
        os.chdir(pathlib.Path(__file__).parent / day)
        __import__(f"{day}.{part}").__getattribute__(part).main()
    except IOError:
        print("This day is not yet solved by me")


if __name__ == '__main__':
    main()
