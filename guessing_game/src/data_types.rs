use std::any::type_name_of_val;

pub fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43.2 % 5.0;

    println!("The sum is: {}, type: {}", sum, type_name_of_val(&sum));
    println!("The difference is: {}, type: {}", difference, type_name_of_val(&difference));
    println!("The product is: {}, type: {}", product, type_name_of_val(&product));
    println!("The quotient is: {}, type: {}", quotient, type_name_of_val(&quotient));
    println!("The remainder is: {}, type: {}", remainder, type_name_of_val(&remainder));


    // Compound types - tuple
    const TUP: (i32, f64, u8) = (500, 6.4, 255);
    let names: (&str, i128, &str) = ("Alice", 19999999999999999932345441454454549356, "Charlie");
    println!("The value of tup is: ({}, {}, {}), type: {}", TUP.0, TUP.1, TUP.2, type_name_of_val(&TUP));
    println!("The value of names is: {:?}, type: {}", names, type_name_of_val(&names));

    const I_: i32 = TUP.0;
    const F_FLOAT: f64 = TUP.1;
    const U_BYTE: u8 = TUP.2;

    println!("The value of i_integer is: {}, type: {}", I_, type_name_of_val(&I_));
    println!("The value of f_float is: {}, type: {}", F_FLOAT, type_name_of_val(&F_FLOAT));
    println!("The value of u_byte is: {}, type: {}", U_BYTE, type_name_of_val(&U_BYTE));

    // Compound types - array
    /* below code is not valid */
    // let e: u8 = "A".parse::<u8>().expect("Not a number!");
    // let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, e];
    // println!("The value of arr is: {:?}, type: {}", arr, type_name_of_val(&arr));

    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("The value of arr is: {:?}, type: {}", arr, type_name_of_val(&arr));

    let sb = [3; 5]; // array of 5 elements, all initialized to 3
    println!("The value of sb is: {:?}, type: {}", sb, type_name_of_val(&sb));
}