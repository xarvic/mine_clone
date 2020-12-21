use utils::{MAP_SIZE, write_line, write_perlin_noise};

fn main() {
    let mut data = [[0.0; MAP_SIZE]; MAP_SIZE];

    write_perlin_noise(&mut data, 0, -16, -16, 3);

    println!("perlin line: {:?}", data);
}