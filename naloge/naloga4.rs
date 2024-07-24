use std::cmp;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

struct Grid {
    size: (usize, usize),
    position: Vec<Vec<bool>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    location: Location,
    f: usize,
    g: usize,
    h: usize,
}

impl Node {
    fn new(location: Location, g: usize, h: usize) -> Self {
        let f = g + h;
        Node { location, f, g, h }
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Grid {
    fn new(size: (usize, usize)) -> Self {
        Grid { 
            size, 
            position: vec![vec![false; size.0]; size.1],
        }
    }
}

fn get_numbers_from_string(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
fn string_to_info(input: String) -> (Location, Location, Grid) {
    let mut lines = input.lines();

    let first_line = get_numbers_from_string(lines.next().unwrap());
    let size = (first_line[0], first_line[1]);

    let second_line = get_numbers_from_string(lines.next().unwrap());
    let umag = Location (second_line[0]-1, second_line[1]-1);
    let rescue = Location (second_line[2]-1, second_line[3]-1);

    let k: usize = lines.next().unwrap().parse().unwrap();

    let mut grid = Grid::new(size);
    for _ in 0..k {
        let line = lines.next().unwrap();    
        let values = get_numbers_from_string(line);

        let (x0, x1) = (cmp::min(values[0]-1,values[2]-1), cmp::max(values[0]-1,values[2]-1));
        let (y0, y1) = (cmp::min(values[1]-1,values[3]-1), cmp::max(values[1]-1,values[3]-1));
        let (dx, dy) = (x1-x0, y1-y0);

        let ship_length = cmp::max(dx, dy)+1;
        let sx = if dx == 0 {0} else {1};
        let sy = if dy == 0 {0} else {1};
        
        let mut x = x0;
        let mut y = y0;

        for _ in 0..ship_length {
            grid.position[y][x] = true;    
            x += sx;
            y += sy;
        }
    }

    (umag, rescue, grid)
}
fn print_grid(umag: Location, rescue: Location, grid: &Grid, path: &Vec<Location>) {
    for row in (0..grid.size.1).rev() {
        for col in 0..grid.size.0 {
            let mut char = if grid.position[row][col] {'o'} else {'.'};
            let location = Location (col, row);
        if path.contains(&location) { char = 'x' }
        if location == umag { char = 'u'; }
            if location == rescue { char = 'r'; }
            print!("{}", char);
        }
        println!("");
    }
}

fn heuristic(start: Location, end: Location) -> usize {
    ((start.0 as isize - end.0 as isize).abs() +
     (start.1 as isize - end.1 as isize).abs()) as usize
}
fn get_neighbours(location: Location, grid: &Grid) -> Vec<Location> {
    let mut neighbours: Vec<Location> = Vec::new();
    for i in 0..4 {
        if let Some(neighbour) = get_neighbour(location, i, grid) {
            neighbours.push(neighbour);
        }
    }
    neighbours
}

fn get_neighbour(location: Location, direction: u8, grid: &Grid) -> Option<Location> {
    let Location (x, y) = location;
    let (dx, dy) = match direction {
        0 => (1, 0),
        1 => (-1, 0),
        2 => (0, 1),
        3 => (0, -1),
        _ => panic!("Invalid direction"),
    };
    let new_y = y as isize + dy;
    let new_x = x as isize + dx;
    if new_x >= 0 && new_x < grid.size.0 as isize &&
        new_y >= 0 && new_y < grid.size.1 as isize {
        let new_location = Location(new_x as usize, new_y as usize);
        if !grid.position[new_y as usize][new_x as usize] {
            return Some(new_location);
        }
    }
    None
}

fn get_shortest_path(start: Location, end: Location, grid: &Grid) -> Option<Vec<Location>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Location, Location> = HashMap::new();

    let start_node = Node::new(start, 0, heuristic(start, end));
    open_set.push(start_node);

    let mut g_score: HashMap<Location, usize> = HashMap::new();
    g_score.insert(start, 0);

    while let Some(current_node) = open_set.pop() {
        let current_pos = current_node.location;

        if current_pos == end {
            let mut path = Vec::new();
            let mut current = current_pos;
            while let Some(&prev) = came_from.get(&current) {
                path.push(current);
                current = prev;
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        let neighbours = get_neighbours(current_pos, grid);
        for neighbour in neighbours {
            let tentative_g_score = g_score[&current_pos] + 1;

            if tentative_g_score < *g_score.get(&neighbour).unwrap_or(&usize::MAX) {
                came_from.insert(neighbour, current_pos);
                g_score.insert(neighbour, tentative_g_score);
                let h = heuristic(neighbour, end);
                let neighbour_node = Node::new(neighbour, tentative_g_score, h);
                open_set.push(neighbour_node);
            }
        }

    }

    None
}

fn main() {
    let input = String::from("15 10\n3 1 8 7\n2\n1 5 14 5\n2 3 15 3");
    let (umag, rescue, grid) = string_to_info(input);
    let path = get_shortest_path(rescue, umag, &grid).expect("No path found");
    print_grid(umag, rescue, &grid, &path);
    println!("Path length: {}", path.len()-1);
}
