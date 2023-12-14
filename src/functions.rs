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
}

fn another_function(num: i32) -> i32 {
    println!("Called another_function");
    return num;
}