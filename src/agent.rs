use libm::atan;
use raylib::consts::PI;
use raylib::core::math::Vector2;

use raylib::prelude::*;

pub struct Agent {
    pub position: Vector2,
    pub velocity: Vector2,
    pub walls: [i32; 2],
    pub max_speed: f32,
    pub max_turn_rate: f32,
    pub size: f32,
    pub colour: Color,
}

impl Agent {
    pub fn update(&mut self, dt: f32, is_pointer_pressed: bool, pointer_position: Vector2) {
        if is_pointer_pressed {
            self.add_forces(pointer_position);
        }
        self.handle_walls();
        self.walk(dt);
    }
    fn walk(&mut self, dt: f32) {
        self.position += self.velocity * dt * self.max_speed;
    }
    fn add_forces(&mut self, pointer_position: Vector2) {}
    fn handle_walls(&mut self) {
        if self.position.x <= 0.0 {
            self.position.x = 0.0;
            self.velocity.x *= -1.0;
        }
        if self.position.x >= self.walls[0] as f32 {
            self.position.x = self.walls[0] as f32;
            self.velocity.x *= -1.0;
        }
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
            self.velocity.y *= -1.0;
        }
        if self.position.y >= self.walls[1] as f32 {
            self.position.y = self.walls[1] as f32;
            self.velocity.y *= -1.0;
        }
    }

    fn get_direction(&self) -> f32 {
        //returns the direction of the velocity vector in degrees
        let angle = 180.0 / atan((self.velocity.y / self.velocity.x) as f64 * PI) as f32;
        if self.velocity.x < 0.0 {
            return angle + 180.0;
        } else {
            return angle;
        }
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle, debug: bool, pointer_position: Vector2) {
        d.draw_poly(
            self.position,
            3,
            self.size,
            -self.get_direction() + 90.0,
            // 090.0,
            self.colour,
        );
        d.draw_line_v(
            self.position,
            (self.velocity * self.max_speed) + self.position,
            Color::WHITE,
        );
        d.draw_line_v(
            self.position,
            Vector2::new(1.0, 0.0) * self.max_speed + self.position,
            Color::GREEN,
        );
    }
}
