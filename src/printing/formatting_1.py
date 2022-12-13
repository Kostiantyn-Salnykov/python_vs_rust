if __name__ == "__main__":
    value_1 = "Hello"
    value_2 = "World"

    # Use <obj.>__str__() or str(<obj>)
    print(f"{value_1} {value_2}")  # default with f-string
    print("{0} {1}".format(value_1, value_2))  # with .format() by indexes
    print("{hello} {world}".format(hello=value_1, world=value_2))  # with .format() by names
    print(f"{value_1=} {value_2=}")  # with names
    # Use <obj>.__str__() or str(<obj>)
    print(f"{value_1!s} {value_2!s}")
    # Use <obj>.__repr__() or repr(<obj>)
    print(f"{value_1!r} {value_2!r}")
    # Use ascii(<obj>)
    print(f"{value_1!a} {value_2!a}")
