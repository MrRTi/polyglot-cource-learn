fn get_input() -> &'static str {
    "forward 5
down 5
forward 8
up 3
down 8
forward 2"
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Point {
    let (direction, amount) = line.split_once(" ").expect("must contain whitespace");
    let amount = str::parse::<i32>(amount).expect("second arh must be integer");

    if direction == "forward" {
        return Point { x: amount, y: 0 };
    }

    if direction == "up" {
        return Point { x: 0, y: -amount };
    }

    Point { x: 0, y: amount }
}

fn main() {
    let result = get_input()
        .lines()
        .map(|x| parse_line(x))
        .fold(Point{x: 0, y: 0}, |mut acc, point| {
            acc.x += point.x;
            acc.y += point.y;
            return acc;
        });

    println!("{:?}, {}", result, result.x * result.y); 
}
