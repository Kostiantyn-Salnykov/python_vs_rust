if __name__ == "__main__":
    value_1 = "Hello"
    value_2 = "World"

    # Simple write to stdout,
    # with new line, and space between printed objects.
    print(value_1, value_2)

    # Write to stdout (change space between objects to "_").
    print(value_1, value_2, sep="_")

    # Simple write to stdout (without new line)
    print(value_1, value_2, end="")
