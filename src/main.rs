use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("raylib [core] example - basic window")
        .build();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        //update loop goes here

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        //render loop goes here
    }
}
