pub fn functions() {
    //
    demo_function();

    //
    let result = function_with_parameters(5, 10);
    println!("The result of the function with parameters is: {}", result);

    // 
    let result2 = function_with_parameters_with_return(20, 30);
    println!("The result of the second function call is: {}", result2);
}

fn demo_function(){
    println!("This is a demo function.");
}


// without the return keyword, this function know as 'Tail Expression' it will not have any impact.
fn function_with_parameters(x: i32, y: i32) -> i32 {
   x + y
}

fn function_with_parameters_with_return(x: i32, y: i32) -> i32 {
    return x + y;
}