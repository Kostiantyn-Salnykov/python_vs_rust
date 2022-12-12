# Hello World 🌍!

## {{python}}
To create standard developers program "Hello World 🌍!" with {{python}} follow this steps:

1. Create directory (where you want to store {{pyt}} projects, for example `PyExamples`), to creat it, use: `cd ~ && mkdir PyExamples`;
2. Move to created directory, `cd PyExamples/`;
3. Run command `:::py python3.11 -m venv .venv` - (it will create virtualenv inside `.venv` directory);
4. To active virtualenv, use:
    
    === "Windows"
        ```{.commandline linenums="0"}
        .venv\Scripts\activate.bat
        ```
    === "Unix (MacOS, Linux)"
        ```{.commandline linenums="0"}
        source .venv/bin/activate
        ```

    !!! success
        Inside terminal appears `(.venv)` before your username, this means that your virtualenv activated successfully.
        
        For example:
        `:::commandline (.venv) user@User:~/PyExamples$ `

    !!! tip "Deactivation"
        To deactivate virtualenv use:
        ```{.commandline linenums="0"}
        deactivate
        ```

5. Create `hello_world.py` inside directory;
6. Set content of a file to:

    === "{{python}}"
        ```{.python}
        --8<-- "hello_world/hello_world.py"
        ```
    === "{{py}}"
        ```{.python}
        --8<-- "hello_world/hello_world_short.py"
        ```

7. Run this code with this command: `python -m hello_world`.
You'll see output inside terminal: `Hello World!`

## {{rust}}

1. Create directory (where you want to store {{rst}} projects, for example `RsExamples`), to creat it, use: `cd ~ && mkdir RsExamples`;
2. Move to created directory, `cd RsExamples/`;
3. Initialize binary project with `cargo new <binary_name>`, for example `cargo new hello_world`
4. Mode to binary project `cd <binary_name>/`, for example `cd hello_world/`
5. Cargo creates `src` directory with `main.rs` file (`src/main.rs`).
6. Cargo by default create `main.rs` with "Hello, world!" program, but you can change it a little:

    === "{{rust}}"
        ```{.rust}
        --8<-- "hello_world/hello_world.rs"
        ```


7. Run code with this command `cargo run`. You'll se output inside terminal: `Hello World!`

