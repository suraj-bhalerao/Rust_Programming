fn main() {
    let s2 = String::from("Hello, world!");
    {
        let s1 = "Hello";
        println!("s1 is: {}", s1);
    }  
    // std::mem::drop(s2); // Explicitly drop s2 to free its memory
    // println!("Value of s1 is: {}", s1); // This line will cause a compile-time error because s1 is out of scope

    // accessing s2 after it has been dropped will also cause a compile-time error
    println!("Value of s2 is: {}", s2); // This line will cause a compile-time error because s2 has been dropped


    {
        let s3 = String::from("Hello, Rust!");
        let s4 = s3; // Move ownership of s3 to s4
        println!("s4 is: {}", s4);
    }
    println!("s4 is: {}", s4);
}
