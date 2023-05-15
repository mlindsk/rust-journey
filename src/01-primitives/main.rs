// Note: All prefix underscores are there to remove the
// warning: variable `mutable` is assigned to, but never used

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;
    return (bool_param, int_param)
}

// A slice is a type that represents a
// dynamically sized view into a contiguous
// sequence of elements. Slices allow you to
// work with a portion or a "slice" of an array,
// vector, or another sequence without taking ownership of the data.

// Taking a slice of an array by referencing it:
// let array: [i32; 5] = [1, 2, 3, 4, 5];
// let slice: &[i32] = &array;

// This function borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}


fn main() {
    // Variables can be type annotated.
    let _logical: bool = true;
    let _a_float: f64 = 1.0;  // Regular annotation
    let _an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let _default_float   = 3.0; // `f64`
    let _default_integer = 7;   // `i32`

    // A type can also be inferred from context.
    let mut _inferred_type = 12; // Type i64 is inferred from another line.
    _inferred_type = 4294967296i64;

    // A mutable variable's value can be changed.
    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21;

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let _mutable = true;


    // https://doc.rust-lang.org/std/result/enum.Result.html#method.unwrap
    let ok: Result<i32, &str> = Ok(10);
    let o: i32 = ok.unwrap();

    // NOTE: Since Result has implemented the Debug trait we can print using {:?}
    // For custom types we must do: impl std::fmt::Debug for <T> { fn fmt(&self, ...)}
    println!("{:?}", ok); 
    println!("{:?}", o);


    // NOTE: This is a 'string-slice' (&str) which is borrowed immutable view.
    // Often used with string data without taking ownership.
    // As we have seen we can also have slices of other types like arrays of ints e.g. &[i32]
    
    let text = "Hello, World!";
    let my_string = text.replace("Hello, ", "");
    println!("{}", my_string);
    

    // TUPLES:
    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("The reversed pair is {:?}", reverse(pair));

    // ARRAYS:
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    analyze_slice(&xs);

    for i in 0..xs.len() + 1 { // Oops, one element too far!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // NOTE:
    // Some and None are variants of the Option<T> enum use to represent
    // values that are either present or not

    let k: i32 = 5;
    match k {
        5 => println!("{}", k),
        3 => println!("{}", k),
        _ => println!("{}", k), // A a match arm (handles all remaining cases)
    }



    // Easy way to check the type of a variable; set it to a wrong type
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // let slice: () = &array;

}
