if __name__ == "__main__":
    if True:
        print("if True")  # always works code

    if False:
        print("if False")  # unreachable code

    if not True:
        print("if not True")  # unreachable code

    if not False:
        print("if not False")  # always works code

    if True & False:
        print("if True & False")  # unreachable code

    if True & True:
        print("if True & True")  # always works code

    if False & False:
        print("if False & False")  # unreachable code

    if True | False:
        print("if True | False")  # always works code

    if True | True:
        print("if True | True")  # always works code

    if False | False:
        print("if False | False")  # unreachable code
