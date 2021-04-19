fn main() {
    print_my_number(10);
    
}

fn print_my_number(num:u32){
    for i in 0..num{
        
        if is_even(i) {
            println!("Even number is {}", i);
        }
        else {
            println!("Odd number is {}", i);
        }
    }
    
}

fn is_even(num: u32) ->bool {
       return num%2==0;
}