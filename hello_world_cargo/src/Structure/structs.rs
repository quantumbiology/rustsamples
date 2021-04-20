// Declaration of Structure 
struct Color {
  red: u8;
  green: u8;
  blue: u8;
}
fn main() {

    let mut bg = Color {red:255 , green : 155, blue : 65};l
    
    println!("Original values are {}, {}, {}" , bg.red, bg.green, bg.blue);

    bg.blue = 45;
    println!("Changed values are {}, {}, {}" , bg.red, bg.green, bg.blue);

}