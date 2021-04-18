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

    // While Loop 

     let mut num = 5;
     while num <= 50
     {
         if num%5 == 0
         {
             println!("Number is {}", num);
         }
         num += 1;
     }

     // For Loop 

   // Simple for loop inlcusive of upper bound that is 11 in below case

     for number in 1..11 {
         println!("The numbers are {}", number);
     }

     // Assigning number before using it 

      let number =0;
      for i in number..11{
         println!("The numbers are {}", i);
      }

      // Using vectors in loop formally known as Array in other languages

      let animals = vec!["Rabbit", "Dog", "Cat"];
       for animal in animals.iter(){ 
         println!("The animal is {}", animal);
       }

       // using vectors with index 
       for (index,animal) in animals.iter().enumerate(){ 
        println!("The animal index is  {} and name is {}", index, animal);
      }
}
