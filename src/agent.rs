use libm;
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

    fn get_triangle_points(&self) -> [Vector2; 3] {
        //returns an array of three vectors representing the points of a triangle
        //takes into acount the direction the velocity vector is heading, and points the triangle in that direction
        let angle = atan(self.velocity.y as f64 / self.velocity.x as f64);
        let angle_rad = angle * PI / 180.0;
        let angle_cos = angle_rad.cos() as f32;
        let angle_sin = angle_rad.clone().sin() as f32;
        let point_1 = Vector2::new(
            // point along the velocity vector
            self.position.x + self.size * angle_cos,
            self.position.y + self.size * angle_sin,
        );
        let point_2 = Vector2::new(
            // point at the right of the velocity vector
            self.position.x + self.size * angle_cos + self.size * angle_sin,
            self.position.y + self.size * angle_sin - self.size * angle_cos,
        );
        let point_3 = Vector2::new(
            // point at the left of the velocity vector
            self.position.x + self.size * angle_cos - self.size * angle_sin,
            self.position.y + self.size * angle_sin + self.size * angle_cos,
        );
        let points = [point_1, point_2, point_3];
        println!("{:?}, {:?}", self.position, points);
        points
    }
    pub fn render(&mut self, d: &mut RaylibDrawHandle, debug: bool, pointer_position: Vector2) {
        let points = self.get_triangle_points();
        d.draw_triangle(points[0], points[1], points[2], self.colour);
        d.draw_circle_v(self.position, 10.0, Color::RED);

        d.draw_line_v(
            self.position,
            (self.velocity * self.max_speed) + 100.0 + self.position,
            Color::WHITE,
        );
        // d.draw_line_v(
        // self.position,
        // Vector2::new(1.0, 0.0) * self.max_speed + self.position,
        // Color::GREEN,
        // );
    }
}
