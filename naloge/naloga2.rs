fn get_number_of_combinations(n: u32, a: u32, b: u32) -> u32 {
    let mut m = n;
    let mut number = 0;
    let mut options: Vec<(u32,u32)> = Vec::new();
    loop {
        if m % b == 0 {
            number += 1;
            options.push(((n-m)/a,m/b))
        }
        if m >= a {m -= a;} else {break}
    }
    // println!("{:?}", options);
    number
}

fn main() {
    println!("{}", get_number_of_combinations(1000, 5, 9));
}
