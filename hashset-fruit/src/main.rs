use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::io::stdin;

static FRUITS: [&'static str; 8] = [
    "Apple",
    "Banana",
    "Cherry",
    "Date",
    "Elderberry",
    "Fig",
    "Grape",
    "Honeydew",
];
fn generate_fruit() -> &'static str {
    let mut rng = thread_rng();
    FRUITS.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    let mut choice = 0;
    while choice != 9 {
        println!("List of Fruit to chose from :");
        for (i, f) in FRUITS.iter().enumerate() {
            println!(" {i}. {f}");
        }
        println!("9. Exit");
        println!("input number of fruit you want to select: ");
        let mut val = String::new();
        stdin().read_line(&mut val);
        let val = val.trim().parse().expect("Value is not Integer");
        let x = FRUITS.get(if val > 0 { val - 1 } else { val });
        if val < 9 {
            fruit_set.insert(x.unwrap());
        }
        choice = val;
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());
}
