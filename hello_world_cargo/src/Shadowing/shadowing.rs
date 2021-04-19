fn main() {
    let mut x =10;
    
    {
        println!("Initial Value of X is {}", x);
        // Chaning the value of X in Inner Scope
        let x =15;
        println!("Changed Value of X is {}", x);
    }

    let x= "X is a string";
    println!("String Data type change Value of X is {}", x);
    
    let x= true;
    println!("Bool Data type change Value of X is {}", x);
}