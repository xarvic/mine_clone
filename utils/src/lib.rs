use std::cmp::min;
use std::mem::swap;

pub const MAP_SIZE_EXP: usize = 4;
pub const MAP_SIZE: usize = 1 << MAP_SIZE_EXP;

pub type MapData = [[f32; MAP_SIZE as usize]; MAP_SIZE as usize];

fn coordinate_random(x: f32, z: f32, seed: u64) -> f32 {
    //let random = 2920.0 * sin(ix * 21942.0 + iy * 171324.0 + 8912.0) * cos(ix * 23157.0 * iy * 217832.0 + 9758.0);
    println!("read perlin value [{}, {}]", x, z);
    /*if (x + z) % 10.0 == 0.0 { //This makes interpolation tests easier
        -1.0
    } else {
        1.0
    }*/
    (x + z) / 5.0
}

pub fn write_line(line: &mut [f32; MAP_SIZE], seed: u64, z: f32, x_start: f32, mut x_read: f32, x_offset: f32, step_size: f32) {
    //read in first may lie outsite of the range
    let mut current = coordinate_random(x_read, z, seed);
    let mut index = x_start;
    //This will get set before the first change
    let mut step = 0_f32;

    if x_start > x_read {
        x_read += step_size;
        let end = coordinate_random(x_read, z, seed);
        step = (end - current) / step_size;
        current += step * x_offset - 1.0;
    }
    //belongs to first read but must occur after second read to check positions first

    for element in line.iter_mut() {
        if index > x_read {
            //we exceeded the last interval => read the next interval end!
            x_read += step_size;
            step = (coordinate_random(x_read, z, seed) - current) / step_size;
        }
        index += 1.0;
        current += step;
        *element = current;
    }
}

fn mod_signed(x: i64, n: usize) -> i64 {
    ((x % n as i64) + n as i64) % n as i64
}

pub fn write_perlin_noise(data: &mut MapData, seed: u64, x_start: i64, z_start: i64, step_size_i: usize) {
    let     step_size = step_size_i as f32;
    let     x_offset = mod_signed(x_start, step_size_i) as f32;
    let     z_offset = mod_signed(z_start, step_size_i) as f32;
    let     x_start = x_start as f32;
    let mut z_start = z_start as f32;
    let     x_read = x_start - x_offset;
    let mut z_read = z_start - z_offset;

    let mut array_index = 0;

    //the first line may not lie inside the array
    let mut start_buf = [0.0_f32; MAP_SIZE];
    let mut end_buf = [0.0f32; MAP_SIZE];
    let mut start = &mut start_buf;
    let mut end = &mut end_buf;
    write_line(end, seed, z_read, x_start, x_read, x_offset, step_size);

    if z_start > z_read {
        //write first part with offset!
        swap(&mut start, &mut end);
        z_read += step_size;
        write_line(end, seed, z_read, x_start, x_read, x_offset, step_size);
        let write_count = min(step_size_i - z_offset as usize, MAP_SIZE - array_index);

        for (line, (start, end)) in data.iter_mut().zip(start.iter().zip(end.iter())) {
            let step = (end - start) / step_size;
            let mut current = *start + step * z_offset;
            for value in &mut line[array_index..array_index+write_count] {
                *value = current;
                current += step;
            }
        }
        array_index += write_count;
    }

    while array_index < MAP_SIZE {
        //The old end is now the start
        swap(&mut start, &mut end);
        z_read += step_size;
        write_line(end, seed, z_read, x_start, x_read, x_offset, step_size);
        let write_count = min(step_size_i, MAP_SIZE - array_index);

        for (line, (start, end)) in data.iter_mut().zip(start.iter().zip(end.iter())) {
            let step = (end - start) / step_size;
            let mut current = *start;
            for value in &mut line[array_index..array_index+write_count] {
                *value = current;
                current += step;
            }
        }
        array_index += step_size_i;
    }
}

pub fn create_perlin_noise(seed: u64, start_x: i64, start_z: i64, step_i: usize) -> MapData {
    let mut map_data = Default::default();
    write_perlin_noise(&mut map_data, seed, start_x, start_z, step_i);
    map_data
}