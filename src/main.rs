use rand::distributions::{Distribution, Uniform};
use raylib::prelude::*;

mod agent;
use crate::agent::Agent;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;
const AGENT_COUNT: i32 = 10;
const MAX_SPEED: f32 = 50.0;
const MAX_TURN_RATE: f32 = 0.1;
const AGENT_SIZE: f32 = 10.0;

fn create_agents() -> Vec<Agent> {
    let mut out: Vec<Agent> = vec![];
    let mut rng = rand::thread_rng();
    let ranges = [
        Uniform::from(0.0..SCREEN_WIDTH as f32),
        Uniform::from(0.0..SCREEN_HEIGHT as f32),
    ];
    let vector_range: Uniform<f32> = Uniform::from(0.0..2.0);
    let color_range: Uniform<u8> = Uniform::from(128..=255);
    for _ in 0..AGENT_COUNT {
        let a = Agent {
            max_speed: MAX_SPEED,
            max_turn_rate: MAX_TURN_RATE,
            position: Vector2::new(ranges[0].sample(&mut rng), ranges[1].sample(&mut rng)),
            walls: [SCREEN_WIDTH, SCREEN_HEIGHT],
            velocity: Vector2::new(
                1.0 - vector_range.sample(&mut rng),
                1.0 - vector_range.sample(&mut rng),
            ),
            size: AGENT_SIZE,
            colour: Color::new(
                color_range.sample(&mut rng),
                color_range.sample(&mut rng),
                color_range.sample(&mut rng),
                255,
            ),
        };
        out.push(a);
    }

    out
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("mouse followers for bakas")
        .build();

    let mut agents = create_agents();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        //update loop goes here
        for a in &mut agents {
            a.update(dt, false, Vector2::new(0.0, 0.0));
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        //render loop goes here

        for a in &mut agents {
            a.render(&mut d, false, Vector2::new(0.0, 0.0));
        }
    }
}
