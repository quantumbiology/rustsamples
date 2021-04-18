#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {

    let mut player_direction:Direction = Direction::Up;
   
    // match keyword act as switch case of other langauges
    match player_direction {

        Direction::Up=> println!("Direction of player is Up and Enum value is {:?}", player_direction),
        Direction::Down => println!("Direction of player is Down and Enum value is {:?}", player_direction),
        Direction::Left=> println!("Direction of player is Left and Enum value is {:?}", player_direction),
        Direction::Right=> println!("Direction of player is Right and Enum value is {:?}", player_direction),
    }
}