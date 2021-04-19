fn main() {
    let mut x = 10; 
     
        println!("The value of original variable is {}", x);
        let y = &mut x;
        *y+=1;
    
    // This will give error if we don't use braces in soem cases. Borrower concepts
    println!("The value of mutable variable x is {}", x);
    
}