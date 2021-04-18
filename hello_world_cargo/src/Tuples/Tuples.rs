fn main() {
    // Declaring Tuples
    let tup1 = (20, 25, 30);

    // Access Tuples using Index
    println!("{}", tup1.2);
    // Accessing using different variable 

    let (a, b, c) = tup1;

    println!("the value of a is {}", a);
    println!("the value of b is {}", b);
    println!("the value of c is {}", c);

    // Accessing Differnt Types of Tuples
    let tup2 = (1, 2.0, "Three", true);
    println!("{}", tup2.0);
    println!("{}", tup2.1);
    println!("{}", tup2.2);
    println!("{}", tup2.3);

    // Nested Tuples
    let tup3 = (1, 2.0, (3, "Three", false), true);
    println!("Nested Tuple 2nd elment value is {}", (tup3.2).2);


}
