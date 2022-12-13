import sys

if __name__ == "__main__":
    value_1 = "Hello"
    value_2 = "World"

    print(value_1, value_2, file=sys.stderr)
    print(value_1, value_2, sep="_", file=sys.stderr)
    print(value_1, value_2, end="", file=sys.stderr)
