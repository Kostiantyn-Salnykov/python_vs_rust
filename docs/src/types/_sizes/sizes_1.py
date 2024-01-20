import math
import sys
import array

types = [
    # Boolean
    True,
    False,
    # Signed Integer 8
    -128,
    127,
    # Signed Integer 16
    -32768,
    32767,
    # Signed Integer 32
    -2147483648,
    2147483647,
    # Signed Integer 64
    -9223372036854775808,
    9223372036854775807,
    # Signed Integer 128
    -170141183460469231731687303715884105728,
    170141183460469231731687303715884105727,
    # Zero integer
    0,
    # Unsigned Integer 8
    255,
    # Unsigned Integer 16
    65535,
    # Unsigned Integer 32
    4294967295,
    # Unsigned Integer 64
    18446744073709551615,
    # Unsigned Integer 128
    340282366920938463463374607431768211455,
    # Float 32
    math.e,
    math.pi,
    math.tau,
    -3.40282347E+38,
    3.40282347E+38,
    float("-inf"),
    float("inf"),
    # Float 64
    -1.7976931348623157E+308,
    1.7976931348623157E+308,
    array.array("i", [1]),
    array.array("q", [1, 2]),
]
sizes = (f"{type(type_)}({type_}) = {sys.getsizeof(type_)} bytes;" for type_ in types)


if __name__ == '__main__':
    for size in sizes:
        print(size)
