pub fn intro() {
    /*
        SYNTAX:
        let/const <variable_name>: <data_type> = expression;
    */

    println!("----------");
    // Variables are immutable by default.
    let num = 5; // immutable : When a variable is immutable, once a value is bound to a name, you can’t change that value
    // num = 10; // Error : cannot assign twice to immutable variable
    println!("Nubmer1 : {num}");


    println!("----------");
    let mut num2 = 7; // immutable
    println!("Number2 : {}", num2);
    num2 = 6; // 
    println!("Number2 : {}", num2);


    println!("----------");
    // const are ALWAYS immutable by default.
    // you're not allowed to use `mut` with constants.
    // const are delcared and initialize at same time.

    // const mut NUM: i32 = 10; // Error: cannot be mutable
    /* 
    const NUM;
    NUM =10; // Error : cannot assign to this expression
    */
    const NUM_3: i32 = 5;
    println!("Number3 : {}", NUM_3);
    

    println!("----------");
    // Shadowing Variables
    // you can declare a new variable with the same name as a previous variable. 
    // Rustaceans say that the first variable is shadowed by the second in SAME SCOPE, which means that the second variable is what the compiler will see when you use the name of the variable.
    let x = 5;
    let x = x + 1; // shadowed and declare the x again.
    {
        let x = x * 2; // also shadowed but after its scope end its shadowing also ends.   
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}"); 

    let _str = "   "; // if this is intentional unused_variable, prefix it with an underscore,
    let _str = _str.len(); 
    // let mut str1 = "   ";
    // str1 = str1.len(); // Assigning value to it; Error : The error says we’re not allowed to mutate a variable’s type.
    println!("----------");

}