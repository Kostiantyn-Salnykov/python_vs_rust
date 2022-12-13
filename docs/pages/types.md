# Data Types 🗄

## Association Types Table 📖

This is roughly parallel table for types between {{python}} and {{rust}}.

| {{python}}                                       | {{rust}}                                                            |
|--------------------------------------------------|---------------------------------------------------------------------|
| bool (`:::py True` or `:::py False`)             | bool (`:::rs true` or `:::rs false`)                                |
| int                                              | i8, i16, i32, i64, i128, isize,<br/> u8, u16, u32, u64, u128, usize |
| float                                            | f32, f64                                                            |
| array.array                                      | array                                                               |
| str / bytes (**rough**)                          | char                                                                |
| str / bytes (**rough**)                          | str                                                                 |
| str (`:::py ""` or `:::py str()`)                | String (`:::rs String::new()`)                                      |
| bytes (`:::py b""`)                              | str.as_bytes()                                                      |
| tuple (`:::py ()` or `:::py tuple()`)            | tuple (`:::rs ();`)                                                 |
| ellipsis (`:::py ...`) / NoneType (`:::py None`) | unit (`:::rs ()`)                                                   |
| list (`:::py []` or `:::py list()`)              | collections.Vec (`:::rs Vec::new()` or `:::rs vec![]`)              |
| dict (`:::py {}` or `:::py dict()`)              | collections.HashMap, collections.BTreeMap                           |
| set (`:::py set()`)                              | collections.HashSet, collections.BTreeSet                           |
| collections.deque(`:::py deque()`)               | collections.VecDeque (`:::rs VecDeque::new()`)                      |
| enum.Enum                                        | enum                                                                |
| NoneType (`:::py None`)                          | enum Option&lt;T&gt;                                                |
| Exception                                        | enum Result&lt;T, E&gt;                                             |
| generator                                        | slice (**rough**)                                                   |
| function, lambda                                 | fn                                                                  |
| range (`:::py range(0, 10)`)                     | Range (`:::rs ..10`)                                                |
| decimal.Decimal                                  |                                                                     |
| fractions.Fraction                               |                                                                     |
| pathlib.Path                                     | path.Path                                                           |
| class / dataclass                                | struct (**rough**)                                                  |
| abc.ABC / `@abstractmethod`                      | trait                                                               |
| datetime.datetime                                |                                                                     |
| datetime.time                                    |                                                                     |
| datetime.timedelta                               | time.Duration (**rough**)                                           |
|                                                  | Box&lt;T&gt; (`:::rs Box::new()`)                                   |


## Boolean

### {{python}}

[Docs](https://docs.python.org/3/library/stdtypes.html#boolean-values){{button}}

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

### Examples

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
---

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