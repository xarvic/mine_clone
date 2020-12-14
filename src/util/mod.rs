use bevy::utils::Instant;
use bevy::ecs::ResMut;

impl Default for FPS {
    fn default() -> Self {
        let now = Instant::now();
        FPS{
            passed_frames: 0,
            start_time: now,
            last_time: now,
        }
    }
}

pub struct FPS{
    passed_frames: u64,
    start_time: Instant,
    last_time: Instant,
}

pub fn print_fps(mut time: ResMut<FPS>) {
    time.passed_frames += 1;
    if time.passed_frames % 60 == 0 {
        let now = Instant::now();

        let current_fps = 60.0 / (now - time.last_time).as_secs_f32();
        let fps = time.passed_frames as f32 / (now - time.start_time).as_secs_f32();
        time.last_time = now;
        println!("{:.1} FPS ({:.1})", current_fps, fps);
    }
}