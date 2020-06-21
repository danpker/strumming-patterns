use rand::prelude::*;

fn main() {
    println!("1 + 2 + 3 + 4 +");
    let mut strums = String::new();

    for _beat in 1..=4 {
        if random() {
            strums.push_str("D ");
        } else {
            strums.push_str("  ");
        }

        if random() {
            strums.push_str("U ");
        } else {
            strums.push_str("  ");
        }
    }

    println!("{}", strums);
}
