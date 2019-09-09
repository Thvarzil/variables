fn main() {

    //let implies a variable which are immutable by default, but can be
    //made mutable by the mut keyword
    let mut x = 5;
    //const implies a constant which is always immutable. The naming
    //convention for constants is all caps with words separated by
    //underscores. Constants require the data type to be specified.
    const MAX_SCORE:u32 = 100_000;

    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //Incrementing x as an example of shadowing.
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let mut spaces = "     ";
    //the following line doesn't work because we can't mutate a
    //variable's type - shadowing allows reusing a simple name where
    //the mut keyword doesn't:

    //spaces = spaces.len();

    //with shadowing:
    let spaces = spaces.len();

}
