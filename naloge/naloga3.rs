fn calculate_duration(input: String) -> f64 {
    let mut lines = input.lines();
    let first_line = lines.next();
    let values: Vec<f64> = first_line.unwrap()
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect("Neveljaven vhod"))
        .collect();

    let mut total_duration = 0.0;
    let mut old_distance = 0.0;
    let mut new_distance;
    let path_length = values[0];
    let mut speed_limit = values[1];
    let number_of_signs = values[2] as i32;

    for _ in 0..number_of_signs {
        let line = lines.next().unwrap();
        let mut pair = line.split_whitespace();

        new_distance = pair.next().unwrap().parse().unwrap();
        if new_distance > path_length { new_distance = path_length };
        total_duration += (new_distance-old_distance)/speed_limit;
        println!("{total_duration}");
        old_distance = new_distance;
        speed_limit = pair.next().unwrap().parse().unwrap();
        if old_distance == path_length { break }
    }

    total_duration
}

fn main() {
    let input = String::from("60 50 5\n1 70\n5 100\n50 20\n 65 130\n 90 120");
    let output = calculate_duration(input);
    let output = (output * 100.0).trunc() / 100.0;
    println!("{output}");
}
