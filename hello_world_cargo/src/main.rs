fn main() {
    let mut  x : i64= 46; // Mutable values  & Unassigned variables
    let mut y :i32 =61;   

    // If Else 
    if x==45 {
        println!("Hello, world!");
    }
   
    else if y!= 60 {
    
    println!("The value of y is {0}", y);
    }

    else 
    {
    println!("The value of x & y are {0} and are {1}", x,y);
        
    }
    x=60;
    y=45;
    println!("The value of x & y are {0} and are {1}", x,y);

    // Looping Concept 
     let mut n = 0;
    loop {
        n +=1;
        if n==7
        {
            continue;
        }

        if n>10
        {
            break;
        }

        println!("The value of n is {}", n);
    }


}
