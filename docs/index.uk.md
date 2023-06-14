# Головна 🏡

## {{python}} проти {{rust}}

Це не змагання чи порівняння тестів, це проста документація, яка може бути корисною для Розробники програмного 
забезпечення на Python, які хочуть вивчити Rust.

## Description 📑

This documentation is designed for developers who know Python 🐍 (like me) in order to gradually understand Rust 🦀 by 
analyzing code examples and differences and similarities in these programming languages.

## Links 🔗

=== "{{python}}"
    [Official Site 🐍](https://www.python.org/){{button}}

    [Download Python ⬇️](https://www.python.org/downloads/){{button}}

    [Python Documentation 🗃️](https://docs.python.org/3/){{button}}

    [Python Standard Library 🧱](https://docs.python.org/3/library/index.html){{button}}
=== "{{rust}}"
    [Official Site 🦀](https://www.rust-lang.org/){{button}}

    [Download Rust ⬇️](https://www.rust-lang.org/tools/install){{button}}

    [Rust Documentation 🗃️](https://www.rust-lang.org/learn){{button}}

    [Rust Standard Library 🧱](https://doc.rust-lang.org/std/index.html){{button}}



## Definitions in docs 🗃

Code inside examples will be written in 2, 3 or 4 blocks, where:

- "{{python}}" - full Python example (usually declared with `:::py3 if __name__ = "__main__:`), possible to run with 
`python -m <path.to.module>`;
- "{{py}}" - short Python example;
- "{{rust}}" - full Rust example (usually declared with `:::rs fn main() {}`), possible to run with 
`cargo run --bin <bin_name>`;
- "{{rs}}" - short Rust example (example, ^^probably not possible to compile^^);

!!! info "Example with 4 blocks"
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

!!! info "Example with 2 block"
    === "{{python}}"
        ```{.python}
        --8<-- "hello_world/hello_world.py"
        ```
    === "{{rust}}"
        ```rs
        --8<-- "hello_world/hello_world.rs"
        ```

!!! tip "Switch tabs"
    It's a possible to move tabs (when you click on one tab and it's already active).
    
    To move it, use ++left++ or ++right++.
