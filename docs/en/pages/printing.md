# Printing 🖨️ & Formatting 📝

In this section, we will figure out in the printing to **stdout** and **stderr** using built-in 
functions / macros.

## Printing 🖨️

### Standard Output (stdout)

=== "{{python}}"
    {{pyt}} uses the built-in function `:::py print()` to write output to **stdout**
    ([Docs](https://docs.python.org/3/library/functions.html#print){{link}}).

    To remove or change space between objects to print you can provide `:::py sep` keyword argument.

    To remove or change new line at the end, you can use `:::py end` keyword argument.


    ```{.python}
    --8<-- "printing/printing_stdout_1.py"
    ```

=== "{{rust}}"
    {{rst}} uses build-in macros `:::rs println!()` to write output to **stdout**
    ([Docs](https://doc.rust-lang.org/std/macro.println.html){{link}}).

    To remove or change space between objects to print should do it manually.

    To remove new line at the end, you should use another built-in macro `:::rs print!`
    ([Docs](https://doc.rust-lang.org/std/macro.print.html){{link}}).

    ```{.rust}
    --8<-- "printing/printing_stdout_1.rs"
    ```

Output will be: 
```{.commandline linenums="0"}
Hello World<NEW_LINE>
Hello_World<NEW_LINE>
Hello World
```


### Standard Error (stderr)

=== "{{python}}"
    To print something in **stderr** you can modify `:::py file` keyword argument to `:::py sys.stderr`.

    To remove or change space between objects to print you can provide `:::py sep` keyword argument.

    To remove or change new line at the end, you can use `:::py end` keyword argument.

    ```{.python}
    --8<-- "printing/printing_stderr_1.py"
    ```


=== "{{rust}}"
    To print something in **stderr** you should use another built-in macro `:::rs eprintln!`
    ([Docs](https://doc.rust-lang.org/std/macro.eprintln.html){{link}}).

    To remove or change space between objects to print should do it manually.

    To remove new line at the end, you should use another built-in macro `:::rs eprint!`
    ([Docs](https://doc.rust-lang.org/std/macro.eprint.html){{link}}).

    ```{.rust}
    --8<-- "printing/printing_stderr_1.rs"
    ```

Output will be: 
```{.commandline linenums="0"}
Hello World<NEW_LINE>
Hello_World<NEW_LINE>
Hello World
```


### Formatting 📝

=== "{{python}}"
    In latest {{pyt}} version the best approach to format strings and data types is `f-string` or formatted string literal (declared with `:::py f"{}"`)
    ([f-string docs](https://docs.python.org/3/reference/lexical_analysis.html#formatted-string-literals){{link}},
    [.format() docs](https://docs.python.org/3/library/string.html#format-specification-mini-language){{link}}).

    ```{.python title="Basic str interpolation"}
    --8<-- "printing/formatting_1.py"
    ```

    ```{.commandline title="Output" linenums="0"}
    --8<-- "printing/formatting_1_py_output.txt"
    ```


=== "{{rust}}"
    In {{rst}} standard approach to format string and data types is `:::rust std::fmt` or `:::rust format!` macro (declared `:::rust "{}"`).

    ```{.rust title="Basic str interpolation"}
    --8<-- "printing/formatting_1.rs"
    ```

    ```{.commandline title="Output" linenums="0"}
    --8<-- "printing/formatting_1_rs_output.txt"
    ```