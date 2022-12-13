fn main() {
    if true {
        println!("if true"); // always works code
    }

    if false {
        println!("if false"); // unreachable code
    }

    if !true {
        println!("if !true"); // unreachable code
    }

    if !false {
        println!("if !false"); // always works code
    }

    if true & false {
        println!("if true & false"); // unreachable code
    }

    if true & true {
        println!("if true & true"); // always works code
    }

    if false & false {
        println!("if false & false"); // unreachable code
    }

    if true | false {
        println!("if true | false"); // always works code
    }

    if true | true {
        println!("if true | true"); // always works code
    }

    if false | false {
        println!("if false | false"); // unreachable code
    }
}
