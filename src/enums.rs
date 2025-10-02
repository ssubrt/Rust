
enum Direction{
    North,
    South,
    East,
    West
};

fn main() {
    let go : Direction = Direction::North;

    helper(go);
}


// Pattern Matching
fn helper(go: Direction){
       match go {
    Direction::North => println!("We are going North"),
        Direction::South => println!("We are going South"),
        
        _ => println!("We are going East or West"),
    }

}