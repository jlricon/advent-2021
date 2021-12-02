enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}
impl From<&str> for Command {
    fn from(i: &str) -> Self {
        let first_letter = i.chars().next().unwrap();
        let magnitude = i.split_whitespace().nth(1).unwrap().parse().unwrap();
        match first_letter {
            'f' => Self::Forward(magnitude),
            'd' => Self::Down(magnitude),
            'u' => Self::Up(magnitude),
            _ => panic!(),
        }
    }
}
fn main() {
    let commands: Vec<Command> = include_str!("../../input/day2.txt")
        .lines()
        .map(|n| n.into())
        .collect();
    let final_pos_prod = commands
        .iter()
        .fold([0, 0], |[horiz, depth], next| match next {
            Command::Forward(n) => [horiz + n, depth],
            Command::Down(n) => [horiz, depth + n],
            Command::Up(n) => [horiz, depth - n],
        })
        .iter()
        .fold(1, |acc, next| acc * next);
    println!("{}", final_pos_prod);
    let final_pos_prod = commands
        .iter()
        .fold([0, 0, 0], |[horiz, depth, aim], next| match next {
            Command::Forward(n) => [horiz + n, depth + n * aim, aim],
            Command::Down(n) => [horiz, depth, aim + n],
            Command::Up(n) => [horiz, depth, aim - n],
        })
        .iter()
        .take(2)
        .fold(1, |acc, next| acc * next);
    println!("{}", final_pos_prod)
}
