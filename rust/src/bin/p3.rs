fn get_input() -> &'static str {
    "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
}

fn get_count() -> usize {
    get_input()
        .lines()
        .enumerate()
        .flat_map(|(idx, line)| line.chars().nth(idx * 3 % line.len()))
        .filter(|&x| x == '#')
        .count()
}

fn main() {
    println!("{:?}", get_count())
}
