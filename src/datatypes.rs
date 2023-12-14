pub fn intro() {
    // Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.
    // Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.
    // More about data type thoery part visit the site : https://doc.rust-lang.org/book/ch03-02-data-types.html

    // syntax 
    // let <variable_name>: <data_type> = statement;

    // the `isize` and `usize` types depend on the architecture of the computer your program is running on.

    // rust defaults
    // interger is i32
    // float is f64

    // explicitly type annotation and type casting
    let num: i32 = 10; 
    println!("{num}");
    let min_num: i32 = i32::MIN; 
    println!("{min_num}");
    let max_num: i32 = i32::MAX; 
    println!("{max_num}");

    let float_num: f32 = 10.02;  
    println!("{float_num}");

    let is_rust_hard: bool = false;
    println!("{is_rust_hard}");
    let is_rust_fun: bool = true;
    println!("{is_rust_fun}");

    let letter_a: char = 'a';
    println!("{letter_a}");

    /*
        &str : 
        - It is a reference to a fixed-sized sequence of characters. 
        - It can be stored either in the heap or in the program's binary. 

        String: 
        - It is a growable string type.
        - It is stored in the heap.
    */
    let message: &str = "Hello, world!"; // Reference to String Slice 
    println!("{message}");
    let mut name: String = String::from("Rust"); // String is Growable string.
    name = "Rust Superb".to_string();
    println!("{name}");


    /*
        Tuples:
        - A tuple is a general way of grouping together a number of values with a variety of types into one compound type. 
    */
    let tuple: (i32, f32, u8) = (500, 10.23, 9);
    println!("{} {} {}", tuple.0, tuple.1, tuple.2);
    let (x,y,z) = tuple;
    println!("{} {} {}", x, y, z);


    // Array
    let arr1 = [1,2,3,4,5];
    println!("{}", arr1[0]);
    let arr2: [i8; 5] = [1,2,3,4,5];
    println!("{}", arr2[0]);
    let arr3 = [3; 5];
    println!("{}", arr3[0]);
}