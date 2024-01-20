# Перші кроки 👣

В цьому розділі описано основні кроки для початку програмування на {{pyt}} або {{rst}}.

## {{python}}

### Встановлення 🔌

=== "Windows"
    1. Просто завантажте файл `.exe`;
    2. Встановіть його;
    3. Додайте {{pyt}} до системного `PATH` (для нього повинен бути :octicons-checkbox-16:);
    4. Встановіть :octicons-checkbox-16: для опції "__install for all users__";
=== "Linux"
    1. Download archive;
    2. Extract it;
    3. Go go extracted folder;
    4. Use these commands:

        1. `./configure` - prepare for installation;
        2. `make` - extract / download / check files;
        3. `make test` - to rust CPython tests before installation;
        4. `make altinstall` - to install new version together with already installed, without change the default one;
        
        !!! info
            All commands can require `sudo` access (but it depends).

        !!! danger
            If you need override your current python enterpreter with new one, use can use `make install`.

### Virtual Environment 🔮✨

For {{python}} projects are __best practices__ to use different {{pyt}} versions and separate installed versions for 
dependencies.

[venv](https://docs.python.org/3/tutorial/venv.html){{button}}

Here is another good option to manage virtual environments, called Poetry:

[Poetry](https://python-poetry.org/){{button}}
[Poetry - Managing environments](https://python-poetry.org/docs/managing-environments/){{button}}

### Code Formatters 🧾

{{pyt}} has no standard auto formatters included to it 😢.

Several of common community formatters is:

[black](https://black.readthedocs.io/en/stable/){{button}} - code formatter;
[isort](https://pycqa.github.io/isort/){{button}} - imports formatter;

## {{rust}}

### Installation 🔌

To install {{rst}} it's better to use the official method via `rustup` (this is {{rst}} toolchain).

=== "Windows "

    1. Go to [Rustup](https://rustup.rs/#){{button}};
    2. Click "display all supported installers";
    3. Select **x32** or **x64** bit version of `rustup-init.exe`;
    4. Install it;
=== "Unix (MacOS, Linux)"

    1. Go to [Rustup](https://rustup.rs/#){{button}};
    2. Copy `cURL` link and execute it in terminal (maybe it will require `sudo` previliages);

    !!! tip "Check versions"
        ```{.commandline linenums="0"}
        rustup -V && rustc -V && cargo -V
        ```

### Virtual Environment 🔮✨

{{rust}} no need to manage virtual environment in such way like a {{pyt}}, because every package 
(called `Crate` in {{rst}}) is separated from others and it builds at compile time.

To manage dependencies and crates in {{rst}} toolchain - exists 
[Cargo](https://doc.rust-lang.org/cargo/index.html){{button}}.

### Code Formatters 🧾

{{rst}} has standard formatter called `rustfmt`.

You can call it directly from Cargo inside your crate:
```{.commandline linenums="0"}
cargo fmt
```

Or from `rustfmt` by one file:
```{.commandline linenums="0"}
rustfmt <path_to_file>
```

!!! example
    ```{.commandline linenums="0"}
    rustfmt docs/src/hello_world/hello_world.rs
    ```
