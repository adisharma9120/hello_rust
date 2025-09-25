#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    for d in directions {
        move_around(d);
    }
}

fn move_around(direction: Direction) {
    match direction {
        Direction::North => println!("Moving North 🧭"),
        Direction::East  => println!("Moving East ➡️"),
        Direction::South => println!("Moving South ⬇️"),
        Direction::West  => println!("Moving West ⬅️"),
    }
}
