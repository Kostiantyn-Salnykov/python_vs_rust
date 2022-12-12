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
| list (`:::py []` or `:::py list()`)              | Vec (`:::rs vec![]`)                                                |
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


## Boolean (bool)

=== "{{python}}"
    ```py
    --8<-- "docs/src/types/boolean/bool.py"
    ```
=== "{{rust}}"
    ```rs
    --8<-- "docs/src/types/boolean/bool.rs"
    ```
