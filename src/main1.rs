fn main() {
    println!("Hello, world!");
    let x = 4; // The compiler implicitly decides the type of variable
    println!("x is: {}", x);
    // All varibales defined are immutable, once a value is assigned other value can not be assigned

    // let y =7;
    // y = 8;
    // error[E0384]: cannot assign twice to immutable variable `y`
    // --> src/main.rs:8:5
    // |
    // 6 |     let y = 5;
    // |         -
    // |         |
    // |         first assignment to `y`
    // |         help: consider making this binding mutable: `mut y`
    // 7 |
    // 8 |     y =7;
    // |     ^^^^ cannot assign twice to immutable variable

    // We must `mut y = 7;` in order to assign a second value
    let mut y = 7;
    println!("{}", y);
    y = 8;
    println!("{}", y);
    // This metgod however does not allow one to change the type of the variable

    // One can also re create the variable in the scope like this:
    let y = 9;
    println!("{}", y);
    // This method allows one to change the variable type as well
}
