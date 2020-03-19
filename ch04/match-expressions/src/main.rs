enum Action {
    Drive,
    Pickup,
    Turn(Direction),
    Stop,
}

enum Direction {
    Left,
    Right,
}

fn print_action(a: Action) {
    match a {
        Action::Drive => println!("Drive forward!"),
        Action::Pickup => println!("Pick up object!"),
        Action::Turn(direction) => match direction {
            Direction::Left => println!("Turn left!"),
            Direction::Right => println!("Turn right!"),
        },
        Action::Stop => println!("Stop!"),
    }
}

fn main() {
    use self::Action::*;
    use self::Direction::*;
    let program = vec![
        Drive,
        Turn(Left),
        Drive,
        Pickup,
        Turn(Left),
        Turn(Left),
        Turn(Left),
        Drive,
        Turn(Right),
        Drive,
        Stop,
    ];
    for action in program {
        print_action(action);
    }
}
