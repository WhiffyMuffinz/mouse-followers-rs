use rand::distributions::{Distribution, Uniform};
use raylib::prelude::*;

mod agent;
use crate::agent::Agent;

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;
const AGENT_COUNT: i32 = 100;
const MAX_SPEED: f32 = 1000.0;
const MAX_TURN_RATE: f32 = 1e-4;
const AGENT_SIZE: f32 = 10.0;
const DEBUG_VECTORS: bool = false;
const DEBUG_POINTS: bool = false;

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
            trail_locations: vec![],
        };
        out.push(a);
    }

    out
}

fn create_fixed_agent() -> Vec<Agent> {
    let a: Agent = Agent {
        max_speed: 100.0,
        max_turn_rate: MAX_TURN_RATE,
        position: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        walls: [SCREEN_WIDTH, SCREEN_HEIGHT],
        velocity: Vector2::new(1.0, 1.0),
        size: AGENT_SIZE,
        colour: Color::BLUE,
        trail_locations: vec![],
    };

    vec![a]
}
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("mouse followers for bakas")
        .build();

    // let mut agents = create_fixed_agent();
    let mut agents = create_agents();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        //update loop goes here
        let pointer_pressed = rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON);
        let pointer_position = rl.get_mouse_position();
        for a in &mut agents {
            a.update(dt, pointer_pressed, pointer_position);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_fps(10, 10);
        //render loop goes here

        for a in &mut agents {
            a.render(&mut d, DEBUG_POINTS, DEBUG_VECTORS, pointer_position);
        }
    }
}
