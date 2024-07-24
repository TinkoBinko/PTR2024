use std::cmp;

struct Location (usize, usize);
struct Grid {
    size: (usize, usize),
    position: Vec<Vec<bool>>,
}
impl Grid {
    fn new(size: (usize, usize)) -> Self {
        Grid {
            size,
            position: vec![vec![false; size.1]; size.0],
        }
    }
}

fn string_to_info(input: String) -> (Location, Grid) {
    let mut lines = input.lines();
    let height: usize = lines.next().unwrap().parse().unwrap();
    let width: usize = lines.next().unwrap().parse().unwrap();
    let mut blahaj = Location (0, 0);
    let mut grid = Grid::new((height, width));
    for y in 0..height {
        let line = lines.next().unwrap();
        for x in 0..width {
            let char = line.as_bytes()[x] as char;
            match char {
                'H' => { grid.position[y][x] = true },
                'B' => { blahaj = Location (x, y) },
                _ => (),
            };

        }
    }
    
    (blahaj, grid)
}

fn get_best_path(blahaj: Location, grid: &Grid) -> isize {
    let (height, width) = grid.size;
    let mut dp = vec![vec![-1; width]; height];
    dp[blahaj.1][blahaj.0] = 0;

    for x in blahaj.0..width {
        for y in 0..height {
            if dp[y][x] != -1 {
                let current_food = dp[y][x];
                let directions = [(-1, 1), (0, 1), (1, 1)];
                for &(dy, dx) in &directions {
                    let ny = y as isize + dy;
                    let nx = x + dx;
                    if nx < width && ny >= 0 && ny < height as isize {
                        let ny = ny as usize;
                        let extra_food = if grid.position[ny][nx] { 1 } else { 0 };
                        dp[ny][nx] = cmp::max(dp[ny][nx], current_food + extra_food);

                    }
                }
            }
        }
    }

    let mut max_food = 0;
    for y in 0..height {
        if dp[y][width - 1] !=! -1 {
            max_food = cmp::max(max_food, dp[y][width-1]);
        }
    }

    max_food
}

fn main() {
    let input = String::from("5\n10\n.HH.H.HHH.\n.H........\n...H......\nB.....H...\n..........");
    let (blahaj, grid) = string_to_info(input);
    // println!("Blahaj is @ {:?}", blahaj);
    // println!("{:?}", grid);
    let max_food = get_best_path(blahaj, &grid);
    println!("{max_food}");
}

