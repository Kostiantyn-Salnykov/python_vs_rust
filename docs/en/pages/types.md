# Data Types 🗄️

## Association Types Table 📖

This is a roughly parallel table for types between {{python}} and {{rust}}.

| {{python}}                                       | {{rust}}                                                        |
|--------------------------------------------------|-----------------------------------------------------------------|
| bool (`:::py True` or `:::py False`)             | bool (`:::rs true` or `:::rs false`)                            |
| int                                              | i8, i16, i32, i64, i128, isize,<br/> u8, u16, u32, u64, u128, usize |
| float                                            | f32, f64                                                        |
| array.array                                      | array                                                           |
| str / bytes (**rough**)                          | char                                                            |
| str / bytes (**rough**)                          | str                                                             |
| str (`:::py ""` or `:::py str()`)                | String (`:::rs String::new()`)                                  |
| bytes (`:::py b""`)                              | str.as_bytes()                                                  |
| tuple (`:::py ()` or `:::py tuple()`)            | tuple (`:::rs ();`)                                             |
| ellipsis (`:::py ...`) / NoneType (`:::py None`) | unit (`:::rs ()`)                                               |
| list (`:::py []` or `:::py list()`)              | collections.Vec (`:::rs Vec::new()` or `:::rs vec![]`)          |
| dict (`:::py {}` or `:::py dict()`)              | collections.HashMap, collections.BTreeMap                       |
| set (`:::py set()`)                              | collections.HashSet, collections.BTreeSet                       |
| collections.deque(`:::py deque()`)               | collections.VecDeque (`:::rs VecDeque::new()`)                  |
| enum.Enum                                        | enum                                                            |
| NoneType (`:::py None`)                          | enum Option&lt;T&gt;                                            |
| Exception                                        | enum Result&lt;T, E&gt;                                         |
| generator                                        | slice (**rough**)                                               |
| function                                         | fn                                                              |
| lambda (`:::py lambda x, y: x + y`)              | closure (```:::rs |x, y| x + y```)                              |
| range (`:::py range(0, 10)`)                     | Range (`:::rs ..10`)                                            |
| decimal.Decimal                                  |                                                                 |
| fractions.Fraction                               |                                                                 |
| pathlib.Path                                     | path.Path                                                       |
| class / dataclass                                | struct (**rough**)                                              |
| abc.ABC / `@abstractmethod`                      | trait                                                           |
| datetime.datetime                                |                                                                 |
| datetime.time                                    |                                                                 |
| datetime.timedelta                               | time.Duration (**rough**)                                       |
|                                                  | Box&lt;T&gt; (`:::rs Box::new()`)                               |

## Association Sizes Table 💾
One of the main features of {{rust}} is working with memory, so I suggest that you familiarize yourself with this table.

!!! tip "Memory management"
    Also to clarify, {{rust}} stores most of the standard data types **on the stack**, while {{python}} **always uses the heap** for this.

This is a roughly parallel table for types sizes between {{python}} and {{rust}} (in bytes).

!!! info "💾"
    8️⃣ bits 🟰 1️⃣ byte 😉

| Type: {{py}} / {{rs}} | Value: {{py}} / {{rs}}                                                           | Size: {{py}} | {{rs}} |
|-----------------------|----------------------------------------------------------------------------------|-------------:|:-------|
| bool                  | `:::py False` / `:::rs true`                                                     |           24 | 1      |
| bool                  | `:::py True` / `:::rs true`                                                      |           28 | 1      |
| int / i8              | `:::py -128` ➖ `:::py 127`                                                       |           28 | 1      |
| int / i16             | `:::py -32768` ➖ `:::py 32767`                                                   |           28 | 2      |
| int / i32             | `:::py -2147483648` ➖ `:::py 2147483647`                                         |           32 | 4      |
| int / i64             | `:::py -9223372036854775808` ➖ `:::py 9223372036854775807`                       |           36 | 8      |
| int / i128            | `:::py -170141183460469231731687303715884105728`                                 |           44 | 16     |
| int / u8              | `:::py 0` ➖ `:::py 255`                                                          |           24 | 1      |
| int / u16             | `:::py 0` ➖ `:::py 65535`                                                        |           28 | 2      |
| int / u32             | `:::py 0` ➖ `:::py 4294967295`                                                   |           32 | 4      |
| int / u64             | `:::py 0` ➖ `:::py 18446744073709551615`                                         |           36 | 8      |
| int / u128            | `:::py 0` ➖ `:::py 340282366920938463463374607431768211455`                      |           44 | 16     |
| float / f32           | E `:::py 2.718281828459045` / `:::rs 2.7182817`                                  |           24 | 4      |
| float / f32           | PI `:::py 3.141592653589793` / `:::rs 3.1415927`                                 |           24 | 4      |
| float / f32           | TAU `:::py 6.283185307179586` / `:::rs 6.2831855`                                |           24 | 4      |
| float / f32           | `:::py float("-inf")` / `:::rs f32::NEG_INFINITY`                                |           24 | 4      |
| float / f32           | `:::py float("inf")` / `:::rs f32::INFINITY`                                     |           24 | 4      |
| float / f64           | E `:::py 2.718281828459045`                                                      |           24 | 8      |
| float / f64           | PI `:::py 3.141592653589793`                                                     |           24 | 8      |
| float / f64           | TAU `:::py 6.283185307179586`                                                    |           24 | 8      |
| float / f64           | `:::py float("-inf")` / `:::rs f64::NEG_INFINITY`                                |           24 | 8      |
| float / f64           | `:::py float("inf")` / `:::rs f64::INFINITY`                                     |           24 | 8      |
| array.array / array   | `:::py a = array.array("i", [1])` /<br/> `:::rs let mut a:[i8, 1] = [1]`         |           84 | 1      |
| array.array / array   | `:::py b = array.array("q", [1, 2])` /<br/> `:::rs let mut b:[i128, 2] = [1, 2]` |           96 | 32     |


## Boolean ✅❎

### {{python}}
[Docs](https://docs.python.org/3/library/stdtypes.html#boolean-values){{button}}

In Python bool has different sizes, for `:::py True` it's equal to 28 bytes, for `:::py False` is 24 bytes.

```py
--8<-- "docs/src/types/boolean/bool_size.py"
```

!!! warning
    In {{pyt}} Boolean (bool) type are subclass from Integer (int).
    So keep it in mind 🤯.
    
    Roughly `:::py True` is equal to `:::py 1` and `:::py False` is equal to `:::py 0`. 

    !!! example "bool values True / False rougly equavivalent to int values 1 / 0"
        ```{.python linenums="0"}
        >>> True == 1
        True
        >>> False == 0
        True
        ```

    !!! example "bool type actually instance of int type"
        ```{.python linenums="0"}
        >>> issubclass(bool, int)
        True
        ```

    !!! example "All bool values actually instance of int type"
        ```{.python linenums="0"}
        >>> isinstance(bool(), int)
        True
        >>> isinstance(True, int)
        True
        >>> isinstance(False, int)
        True
        ```

### {{rust}}
[Docs](https://doc.rust-lang.org/std/primitive.bool.html){{button}}

In Rust `:::rs bool` type always allocate the same size that equal to 1 byte.

```rs
--8<-- "docs/src/types/boolean/bool_size.rs"
```


### Examples

#### Declarations

=== "{{python}}"
    ```py
    --8<-- "docs/src/types/boolean/bool_1.py"
    ```

    ```{.commandline title="Output" linenums="0"}
    x=True, y=False
    ```

=== "{{rust}}"
    ```rs
    --8<-- "docs/src/types/boolean/bool_1.rs"
    ```

    ```{.commandline title="Output" linenums="0"}
    x=true, y=false
    ```

#### Conditions
Boolean conditions works the same in {{python}} and {{rust}}.

=== "{{python}}"
    ```py
    --8<-- "docs/src/types/boolean/bool_2.py"
    ```

    ```{.commandline title="Output" linenums="0"}
    --8<-- "docs/src/types/boolean/bool_2_py_output.txt"
    ```

=== "{{rust}}"
    ```rs
    --8<-- "docs/src/types/boolean/bool_2.rs"
    ```

    ```{.commandline title="Output" linenums="0"}
    --8<-- "docs/src/types/boolean/bool_2_rs_output.txt"
    ```


[//]: # (## Integer 🔟)

[//]: # (### {{python}})


[//]: # (## Float 🔢)