use rand::Rng;

fn generate_input() -> String {
    let mut input = String::new();
    let mut rng = rand::thread_rng();
    let chars = ['|','/','R','M'];
    let length: u8 = rng.gen_range(5..10);
    let to_eat: u8 = rng.gen_range(1..length);

    input.push_str(&to_eat.to_string());
    input.push('\n');
    input.push_str(&length.to_string());
    input.push('\n');
    for _ in 0..length {
        let random_index = rng.gen_range(0..chars.len());
        let random_char = chars[random_index];
        input.push(random_char);
    }
    input
}

fn get_eaten_length(input: String) -> u64 {
    let mut lines = input.lines();
    let to_eat: u64 = lines.next().unwrap().trim().parse().expect("Neveljaven vhod");
    let total: u64 = lines.next().unwrap().trim().parse().expect("Neveljaven vhod");
    
    let mut eaten_count = 0;
    let mut consecutive_grass_eaten = 0;
    let mut eaten_dandelion = false;

    println!("To eat: {}, total: {}", to_eat, total);
    for (i, char) in lines.next().unwrap().chars().enumerate() {
        if eaten_count == to_eat { return i as u64 ;}
        if consecutive_grass_eaten < 3 && char == '|' {
            eaten_count += 1;
            consecutive_grass_eaten += 1;
        } else {
            consecutive_grass_eaten = 0;
        }
        if char == 'R' {
            eaten_dandelion = true;
            eaten_count += 1;
        }
        if char == 'M' && !eaten_dandelion {
            eaten_count += 1;
        }
    }
    total + 1
}

fn main() {
    let input = generate_input();
    let test_input = String::from("5\n10\n||||//RM||");

    // let input = test_input;
    println!("{}", input);
    let length = get_eaten_length(input);
    println!("{}", length);
}
