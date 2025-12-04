use crate::tools;

const CURRENT_DAY : &str = "day04";

fn get_input() -> Vec<Vec<char>> {
    tools::read_as_grid(CURRENT_DAY)
}

pub fn solve1() -> usize {
    let mut grid = get_input();
    if tools::is_test_mode() {print_grid(&grid);}
    let result = remove_accessible_roll(&mut grid, 'x');
    if tools::is_test_mode() {print_grid(&grid);}
    return result;
}

pub fn solve2() -> usize {
    let mut total_removed = 0;
    let mut grid = get_input();
    if tools::is_test_mode() {print_grid(&grid);}
    loop {
        let removed = remove_accessible_roll(&mut grid, 'o');
        if tools::is_test_mode() {print_grid(&grid);}
        if removed == 0 {break;}
        total_removed += removed;
    }
    return total_removed;
}

fn count_neighbors(grid: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let max_x = grid[0].len() as isize;
    let max_y = grid.len() as isize;
    let directions = [
        (-1, -1),
        ( 0, -1),
        ( 1, -1),
        (-1,  0),
        ( 1,  0),
        (-1,  1),
        ( 0,  1),
        ( 1,  1),
    ];
    directions.iter()
        .filter_map(|(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < max_x && ny >= 0 && ny < max_y {
                Some((nx as usize, ny as usize))
            } else {
                None
            }
        })
        .map(|(a, b)| match grid[b][a] {
            '@' => 1,
            'x' => 1,
            _ => 0,
        })
        .sum()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("");
    grid.iter().for_each(
        |l| println!("{:?}", l)
    );
    println!("");
}

fn remove_accessible_roll(grid: &mut Vec<Vec<char>>, replacement_char: char) -> usize {
    let mut removed = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                let neighbors = count_neighbors(&grid, x, y);
                if neighbors < 4 {
                    grid[y][x] = replacement_char;
                    removed += 1
                }
            }
        }
    }
    return removed
}
