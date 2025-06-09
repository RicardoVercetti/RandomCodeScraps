fn fib(n: u32) -> u32 {
    if n <= 3{
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove<'a> {
    Pass,
    Run(&'a Direction),
    Teleport { x: u32, y: u32 },
}

fn main() {
    // println!("Hello, world!");
    // println!("Here it run...");
    println!("Hello 🌍!");

    let x = 45;
    println!("The fib no: {x}");
    println!("Ans: {}", fib(x));
    dbg!("here is a debug log");

    let pos = Direction::Left;
    let player_pos = PlayerMove::Run(&pos);

    println!("Current direction: {pos:?}");
    println!("Player pos: {player_pos:?}");


}
