// Declaration of Structure 
struct Color {
  red: u8,
  green: u8,
  blue: u8
}
fn main() {
      
    // Getting value from the Struct
    let mut bg = Color {red:255 , green : 155, blue : 65};
    
    println!("Original values are {}, {}, {}" , bg.red, bg.green, bg.blue);
    
    // Changing the value of struct
    bg.blue = 45;
    println!("Changed values are {}, {}, {}" , bg.red, bg.green, bg.blue);

}