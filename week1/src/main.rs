use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    println!("Hello, world!");

    let fruits = vec!["Apple", "Banana", "Cherry"];
    for fruit in &fruits {
        println!("{}", fruit);
    }

    let mut map = std::collections::HashMap::new();
    map.insert("apple", 10);
    map.insert("banana", 8);
    map.insert("orange", 15);
    for (key, value) in &map {
    println!("{}: {}", key, value);
    }

    let mut fruit_salad: Vec<&str> = vec![
        "Apple",
        "Banana",
        "Cherry",
        "Peach",
        "Pear",
    ];

    let mut rng = thread_rng();
    fruit_salad.shuffle(&mut rng);

    for (i, item) in fruit_salad.iter().enumerate() {
        if i != fruit_salad.len() - 1 {
            println!("{} ", item);
        } else {
            println!("{}", item);
        }
    }
}
