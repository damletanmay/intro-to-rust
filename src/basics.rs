use std::io;

pub fn main() {
    explaination_basics();

    loops();

    exercise();
}

fn leave_space() {
    println!("");
    println!("");
    println!("");
}

fn explaination_basics() {
    // define a variable
    let x = 12; // signed int
    let y = 11.5; // float
    let z = (1, 2, 45, "1231"); // tuple
    let w = [123, 123, 123, 123]; // array
    let v = true;
    let mut k = vec!["rust", "is", "easy", ","];
    k.push("isn't it ?");

    println!("x: {}, y: {}", x, y);
    println!("Tuple z: {:?}", z);
    println!("Array w: {:?}", w);
    println!("Boolenan v: {}", v);
    println!("Vector k: {:?}", k);

    leave_space();

    // shadowing
    let x = "Super";
    let y = "Easy";

    println!("x: {}, y: {}", x, y);

    leave_space();

    // immutable borrow reference
    let mut x: u8 = 11;

    {
        println!("Immumatable Reference");
        let w = &x;
        let y = &x;
        // x = 12; not allowed as x is borrowed and is not mutable
        println!("Value of w:{}", w);
        println!("Value of y:{}", y);
        println!("Value of x:{}", x);
    }

    leave_space();

    // mutable borrow reference
    {
        println!("Mumatable Reference");
        let y = &mut x; // borrowing x
                        // let w = &x; // not allowed as x is already borrowed.
                        // let mut v = &x; // not allowed as x is mutably already borrowed.
        *y = 11;
        // x = 12; // not allowed as x is borrowed
        println!("Value of x:{}", y);
        println!("Value of x:{}", x);
    }

    println!("{}", y);

    // just an example not what actually option enum looks like
    // enum Option {
    //     Some,
    //     None,
    // }

    leave_space();
    println!("Option Enum");

    // option enum & match statement
    let variable = Some(55);

    match variable {
        Some(var) => {
            println!("{}", var);
        }
        None => {}
    }

    println!("{}", variable.unwrap());

    // dangling reference
    // let x = dangle();

    // // dangling reference
    // fn dangle() -> &i32 {
    //     let x = 3;
    //     &x
    // }
    leave_space();
}

fn loops() {
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
    leave_space();

    // loop {
    //     println!("To Infinity and Beyond");
    // }

    leave_space();
    for j in 1..10 {
        println!("{}", j);
    }

    leave_space();
}

fn exercise() {
    // TODO: Create a vector of str type,
    // interate in them using variable_name.iter() function and print all elements

    let x = vec!["str", "ts"];

    for i in x.iter() {
        println!("{}", i);
    }
    leave_space();
}
