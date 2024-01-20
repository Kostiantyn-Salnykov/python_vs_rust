# Головна 🏡

![Logo](images/rust_python_no_bg.png){width="300" align=right}
## {{python}} проти {{rust}}

Це не змагання чи порівняння тестів, це проста документація, яка може бути корисною для Розробники програмного 
забезпечення на Python, які хочуть вивчити Rust.

## Опис 📑

Ця документація призначена для розробників, які знають {{python}} (як я), 
щоб поступово зрозуміти {{rust}} шляхом аналізу прикладів коду, відмінностей і 
подібностей в цих мовах програмування.

## Посилання 🔗

=== "{{python}}"
    [Офіційний сайт 🐍](https://www.python.org/){{button}}

    [Завантажити Python ⬇️](https://www.python.org/downloads/){{button}}

    [Документація Python 🗃️](https://docs.python.org/3/){{button}}

    [Стандартна бібліотека Python 🧱](https://docs.python.org/3/library/index.html){{button}}
=== "{{rust}}"
    [Офіційний сайт 🦀](https://www.rust-lang.org/){{button}}

    [Завантажити Rust ⬇️](https://www.rust-lang.org/tools/install){{button}}

    [Документація Rust 🗃️](https://www.rust-lang.org/learn){{button}}

    [Стандартна бібліотека Rust 🧱](https://doc.rust-lang.org/std/index.html){{button}}



## Визначення в документації 🗃️
Код в прикладах буде написаний у 2, 3 або 4 блоки, де:

- "{{python}}" - повний {{pyt}} приклад (зазвичай декларується з `:::py3 if __name__ = "__main__:`), можливо запустити через 
`python -m <path.to.module>`;
- "{{py}}" - скорочений {{pyt}} приклад;
- "{{rust}}" - повний {{rst}} приклад (зазвичай декларується з `:::rs fn main() {}`), можливо запистити через 
`cargo run --bin <bin_name>`;
- "{{rs}}" - скорочений {{rst}} приклад (^^ймовірно неможливо скомпілювати^^);

!!! info "Приклад з 4 блоками"
    === "{{python}}"
        ```{.python}
        --8<-- "hello_world/hello_world.py"
        ```
    === "{{rust}}"
        ```rs
        --8<-- "hello_world/hello_world.rs"
        ```
    === "{{py}}"
        ```{.python linenums="0"}
        --8<-- "hello_world/hello_world_short.py"
        ```
    === "{{rs}}"
        ```{.rust linenums="0"}
        --8<-- "hello_world/hello_world_short.rs"
        ```

!!! info "Приклад з 2 блоками"
    === "{{python}}"
        ```{.python}
        --8<-- "hello_world/hello_world.py"
        ```
    === "{{rust}}"
        ```rs
        --8<-- "hello_world/hello_world.rs"
        ```

!!! tip "Перемикання владок"
    Є можливість переміщення вкладок (коли вии натискаєте на одну вкладку, і вона вже активна).
    
    Щоб змінити її, використовуйте ++left++ або ++right++.
