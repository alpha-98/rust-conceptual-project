pub fn intro() {
    // Functions are prevalent(powerful) in Rust code.
    /*
        SYNTAX : 
        Declaration:
        fn <function_name> (paremeters) -> return_data_type {
            //statement;
            return statement;
        } 
        CALLING : 
        <function_name>(arguments);
    */

   println!("Num : {}", another_function(10));
   println!("{}", function_return_expression(20));
//    println!("{}", function_return_statement(20));
}

fn another_function(num: i32) -> i32 {
    println!("Called another_function");
    return num;
}

fn function_return_expression(x: i32) -> i32 {
    x + 1 // it's an expession without semicolon
}

// fn function_return_statement(x: i32) -> i32 {
//     x + 1; // it's a statement with semicolon // Error
// }