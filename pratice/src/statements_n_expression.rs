pub fn stm_n_expr(){
    // this is an expression
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}