use utils::{MAP_SIZE, write_line, write_perlin_noise};
use std::time::Instant;
use std::cmp::{min, max};
use std::ops::Div;

struct Bencher{
    runs: usize,
    name: String,
}

impl Bencher {
    pub fn new(name: &str, runs: usize) -> Self {
        Self {
            name: name.to_string(),
            runs,
        }
    }
    pub fn iter<T>(&mut self, mut run: impl FnMut() -> T) {

        let start = Instant::now();
        run();
        let elapsed = start.elapsed();

        let mut min_time = elapsed;
        let mut max_time = elapsed;
        let mut av = elapsed;

        for _ in 1..self.runs {
            let start = Instant::now();
            run();
            let elapsed = start.elapsed();
            av += elapsed;
            min_time = min(min_time, elapsed);
            max_time = max(max_time, elapsed);
        }
        av = av.div(self.runs as u32);

        println!("'{}' took ~{}sec ({} - {})",
                 self.name,
                 av.as_secs_f32(),
                 min_time.as_secs_f32(),
                 max_time.as_secs_f32()
        );
    }
}

fn main() {
    let mut data = [[0.0; MAP_SIZE]; MAP_SIZE];

    let mut bencher = Bencher::new("Perlin noise (100)", 100000);
    bencher.iter(|| {
        write_perlin_noise(&mut data, 0, -1, 0, 100, 0.0..=1.0);
    });
    let mut bencher = Bencher::new("Perlin noise (16)", 100000);
    bencher.iter(|| {
        write_perlin_noise(&mut data, 0, -1, 0, 16, 0.0..=1.0);
    });
    let mut bencher = Bencher::new("Perlin noise (13)", 100000);
    bencher.iter(|| {
        write_perlin_noise(&mut data, 0, -1, 0, 13, 0.0..=1.0);
    });
    let mut bencher = Bencher::new("Perlin noise (4)", 100000);
    bencher.iter(|| {
        write_perlin_noise(&mut data, 0, -1, 0, 4, 0.0..=1.0);
    });

}
