extern crate rasciigraph;

use rasciigraph::{plot, plot_many, Config};

fn main() {
    let cities = vec![
        "Lisbon",
        "Madrid",
        "Paris",
        "Berlin",
        "Copenhagen",
        "Stockholm",
        "Moscow",
    ];
    let distances_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];

    let distances_travelled = vec![0.0, 150.56, 1103.36, 2217.27, 2266.42, 3000.0];
    println!("{}", cities.join(" > "));

    println!(
        "{}",
        plot_many(
            vec![distances_travelled.clone(), distances_travelled],
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_width(15)
                .with_caption("Travelled distances (km)".to_string())
        )
    );
}
